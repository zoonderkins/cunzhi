<script setup lang="ts">
import type { McpRequest } from '../../types/popup'
import { invoke } from '@tauri-apps/api/core'
import { useMessage } from 'naive-ui'
import { computed, onMounted, ref, watch } from 'vue'

import MainLayout from '../layout/MainLayout.vue'
import PopupActions from './PopupActions.vue'
import PopupContent from './PopupContent.vue'
import PopupHeader from './PopupHeader.vue'
import PopupInput from './PopupInput.vue'

interface Props {
  request: McpRequest | null
  currentTheme?: string
  mockMode?: boolean
  testMode?: boolean
}

interface Emits {
  response: [response: any[]]
  cancel: []
  themeChange: [theme: string]
  openMainLayout: []
}

const props = withDefaults(defineProps<Props>(), {
  currentTheme: 'dark',
  mockMode: false,
  testMode: false,
})

const emit = defineEmits<Emits>()

// 使用消息提示
const message = useMessage()

// 响应式状态
const loading = ref(false)
const submitting = ref(false)
const selectedOptions = ref<string[]>([])
const userInput = ref('')
const draggedImages = ref<string[]>([])
const inputRef = ref()

// 继续回复设置
const continueReplyEnabled = ref(true)
const continuePrompt = ref('请按照最佳实践继续')

// 显示模式控制
const showMainLayout = ref(false)

// 计算属性
const isVisible = computed(() => !!props.request)
const hasOptions = computed(() => (props.request?.predefined_options?.length ?? 0) > 0)
const canSubmit = computed(() => {
  if (hasOptions.value) {
    return selectedOptions.value.length > 0 || userInput.value.trim().length > 0 || draggedImages.value.length > 0
  }
  return userInput.value.trim().length > 0 || draggedImages.value.length > 0
})

// 获取输入组件的状态文本
const inputStatusText = computed(() => {
  return inputRef.value?.statusText || '等待输入...'
})

// 加载继续回复设置
async function loadReplyConfig() {
  try {
    const config = await invoke('get_reply_config')
    const replyConfig = config as { enable_continue_reply: boolean, continue_prompt: string }
    continueReplyEnabled.value = replyConfig.enable_continue_reply
    continuePrompt.value = replyConfig.continue_prompt
  }
  catch (error) {
    console.error('加载继续回复配置失败:', error)
    // 使用默认值
    continueReplyEnabled.value = true
    continuePrompt.value = '请按照最佳实践继续'
  }
}

// 监听请求变化
watch(() => props.request, (newRequest) => {
  if (newRequest) {
    resetForm()
    loading.value = true
    setTimeout(() => {
      loading.value = false
    }, 300)
  }
}, { immediate: true })

// 组件挂载时加载设置
onMounted(() => {
  loadReplyConfig()
})

// 重置表单
function resetForm() {
  selectedOptions.value = []
  userInput.value = ''
  draggedImages.value = []
  submitting.value = false
}

// 处理提交
async function handleSubmit() {
  if (!canSubmit.value || submitting.value)
    return

  submitting.value = true

  try {
    const response: any[] = []

    // 添加选项响应
    if (selectedOptions.value.length > 0) {
      response.push({
        type: 'text',
        text: `选择的选项: ${selectedOptions.value.join(', ')}`,
      })
    }

    // 添加文本响应
    if (userInput.value.trim()) {
      response.push({
        type: 'text',
        text: userInput.value.trim(),
      })
    }

    // 添加图片响应
    if (draggedImages.value.length > 0) {
      for (const imageData of draggedImages.value) {
        response.push({
          type: 'image',
          source: {
            type: 'base64',
            media_type: 'image/png',
            data: imageData.split(',')[1],
          },
        })
      }
    }

    // 如果没有任何响应内容，添加默认响应
    if (response.length === 0) {
      response.push({
        type: 'text',
        text: '用户确认继续',
      })
    }

    if (props.mockMode) {
      // 模拟模式下的延迟
      await new Promise(resolve => setTimeout(resolve, 1000))
      message.success('模拟响应发送成功')
    }
    else {
      // 实际发送响应
      await invoke('send_mcp_response', { response })
      await invoke('exit_app')
    }

    emit('response', response)
  }
  catch (error) {
    console.error('提交响应失败:', error)
    message.error('提交失败，请重试')
  }
  finally {
    submitting.value = false
  }
}

// 处理主题切换
function handleThemeChange() {
  const newTheme = props.currentTheme === 'light' ? 'dark' : 'light'
  emit('themeChange', newTheme)
}

// 处理打开主界面
function handleOpenMainLayout() {
  showMainLayout.value = !showMainLayout.value
}

// 处理输入更新
function handleInputUpdate(data: { userInput: string, selectedOptions: string[], draggedImages: string[] }) {
  userInput.value = data.userInput
  selectedOptions.value = data.selectedOptions
  draggedImages.value = data.draggedImages
}

// 处理图片添加 - 移除重复逻辑，避免双重添加
function handleImageAdd(image: string) {
  // 这个函数现在只是为了保持接口兼容性，实际添加在PopupInput中完成
  console.log('图片添加事件:', `${image.substring(0, 50)}...`)
}

// 处理图片移除
function handleImageRemove(index: number) {
  draggedImages.value.splice(index, 1)
}

// 处理继续按钮点击
async function handleContinue() {
  if (submitting.value)
    return

  submitting.value = true

  try {
    const response = [{
      type: 'text',
      text: continuePrompt.value,
    }]

    if (props.mockMode) {
      // 模拟模式下的延迟
      await new Promise(resolve => setTimeout(resolve, 1000))
      message.success('继续请求发送成功')
    }
    else {
      // 实际发送继续请求
      await invoke('send_mcp_response', { response })
      await invoke('exit_app')
    }

    emit('response', response)
  }
  catch (error) {
    console.error('发送继续请求失败:', error)
    message.error('继续请求失败，请重试')
  }
  finally {
    submitting.value = false
  }
}
</script>

<template>
  <div v-if="isVisible" class="flex flex-col w-full h-screen bg-black text-white select-none">
    <!-- 头部 - 固定 -->
    <div class="flex-shrink-0 bg-gray-100 border-b-2 border-gray-200">
      <PopupHeader
        :current-theme="currentTheme" :loading="loading" :show-main-layout="showMainLayout"
        @theme-change="handleThemeChange" @open-main-layout="handleOpenMainLayout"
      />
    </div>

    <!-- 主界面模式 -->
    <div v-if="showMainLayout" class="flex-1 overflow-y-auto">
      <MainLayout
        :current-theme="currentTheme"
        :always-on-top="false"
        :audio-notification-enabled="false"
        audio-url=""
        @theme-change="handleThemeChange"
        @toggle-always-on-top="() => {}"
        @toggle-audio-notification="() => {}"
        @update-audio-url="() => {}"
        @test-audio="() => {}"
        @stop-audio="() => {}"
        @test-audio-error="() => {}"
        @update-window-size="() => {}"
      />
    </div>

    <!-- 弹窗模式 -->
    <template v-else>
      <!-- 内容区域 - 可滚动 -->
      <div class="flex-1 overflow-y-auto">
        <!-- 消息内容 - 允许选中 -->
        <div class="mx-2 mt-2 mb-1 px-4 py-3 bg-gray-100 rounded-lg select-text">
          <PopupContent :request="request" :loading="loading" :current-theme="currentTheme" />
        </div>

        <!-- 输入和选项 - 允许选中 -->
        <div class="px-4 pb-3 bg-black select-text">
          <PopupInput
            ref="inputRef" :request="request" :loading="loading" :submitting="submitting"
            @update="handleInputUpdate" @image-add="handleImageAdd" @image-remove="handleImageRemove"
          />
        </div>
      </div>

      <!-- 底部操作栏 - 固定在底部 -->
      <div class="flex-shrink-0 bg-gray-100 border-t-2 border-gray-200">
        <PopupActions
          :request="request" :loading="loading" :submitting="submitting" :can-submit="canSubmit"
          :continue-reply-enabled="continueReplyEnabled" :input-status-text="inputStatusText"
          @submit="handleSubmit" @continue="handleContinue"
        />
      </div>
    </template>
  </div>
</template>
