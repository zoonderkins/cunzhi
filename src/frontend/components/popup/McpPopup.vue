<script setup lang="ts">
import type { McpRequest } from '../../types/popup'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { useMessage } from 'naive-ui'
import { computed, onMounted, onUnmounted, ref, watch } from 'vue'

import PopupActions from './PopupActions.vue'
import PopupContent from './PopupContent.vue'
import PopupInput from './PopupInput.vue'

interface AppConfig {
  theme: string
  window: {
    alwaysOnTop: boolean
    width: number
    height: number
    fixed: boolean
  }
  audio: {
    enabled: boolean
    url: string
  }
  reply: {
    enabled: boolean
    prompt: string
  }
}

interface Props {
  request: McpRequest | null
  appConfig: AppConfig
  mockMode?: boolean
  testMode?: boolean
}

interface Emits {
  response: [response: any]
  cancel: []
  themeChange: [theme: string]
  openMainLayout: []
  toggleAlwaysOnTop: []
  toggleAudioNotification: []
  updateAudioUrl: [url: string]
  testAudio: []
  stopAudio: []
  testAudioError: [error: any]
  updateWindowSize: [size: { width: number, height: number, fixed: boolean }]
}

const props = withDefaults(defineProps<Props>(), {
  mockMode: false,
  testMode: false,
})

const emit = defineEmits<Emits>()

// 使用消息提示
const message = useMessage()

// 響應式狀態
const loading = ref(false)
const submitting = ref(false)
const selectedOptions = ref<string[]>([])
const userInput = ref('')
const draggedImages = ref<string[]>([])
const inputRef = ref()

// 繼續回覆設定
const continueReplyEnabled = ref(true)
const continuePrompt = ref('请按照最佳实践繼續')

// 计算属性
const isVisible = computed(() => !!props.request)
const hasOptions = computed(() => (props.request?.predefined_options?.length ?? 0) > 0)
const canSubmit = computed(() => {
  if (hasOptions.value) {
    return selectedOptions.value.length > 0 || userInput.value.trim().length > 0 || draggedImages.value.length > 0
  }
  return userInput.value.trim().length > 0 || draggedImages.value.length > 0
})

// 獲取輸入元件的狀態文本
const inputStatusText = computed(() => {
  return inputRef.value?.statusText || '等待輸入...'
})

// 載入繼續回覆設定
async function loadReplyConfig() {
  try {
    const config = await invoke('get_reply_config')
    if (config) {
      const replyConfig = config as any
      continueReplyEnabled.value = replyConfig.enable_continue_reply ?? true
      continuePrompt.value = replyConfig.continue_prompt ?? '请按照最佳实践繼續'
    }
  }
  catch (error) {
    console.log('載入繼續回覆設定失敗，使用預設值:', error)
  }
}

// 監聽設定变化（当从設定页面切换回来时）
watch(() => props.appConfig.reply, (newReplyConfig) => {
  if (newReplyConfig) {
    continueReplyEnabled.value = newReplyConfig.enabled
    continuePrompt.value = newReplyConfig.prompt
  }
}, { deep: true, immediate: true })

// 監聽请求变化
watch(() => props.request, (newRequest) => {
  if (newRequest) {
    resetForm()
    loading.value = true
    // 每次显示弹窗时重新載入設定
    loadReplyConfig()
    setTimeout(() => {
      loading.value = false
    }, 300)
  }
}, { immediate: true })

// Telegram 功能已移除

// 處理選項切换
function handleOptionToggle(option: string) {
  const index = selectedOptions.value.indexOf(option)
  if (index > -1) {
    // 取消選擇
    selectedOptions.value.splice(index, 1)
  }
  else {
    // 新增選擇
    selectedOptions.value.push(option)
  }

  // 同步到PopupInput元件
  if (inputRef.value) {
    inputRef.value.updateData({ selectedOptions: selectedOptions.value })
  }
}

// 處理文本更新
function handleTextUpdate(text: string) {
  userInput.value = text

  // 同步到PopupInput元件
  if (inputRef.value) {
    inputRef.value.updateData({ userInput: text })
  }
}

// 元件挂载时載入設定
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

// 處理提交
async function handleSubmit() {
  if (!canSubmit.value || submitting.value)
    return

  submitting.value = true

  try {
    // 使用新的结构化資料格式
    const response = {
      user_input: userInput.value.trim() || null,
      selected_options: selectedOptions.value,
      images: draggedImages.value.map(imageData => ({
        data: imageData.split(',')[1], // 移除 data:image/png;base64, 前缀
        media_type: 'image/png',
        filename: null,
      })),
      metadata: {
        timestamp: new Date().toISOString(),
        request_id: props.request?.id || null,
        source: 'popup',
      },
    }

    // 如果没有任何有效内容，設定預設用户輸入
    if (!response.user_input && response.selected_options.length === 0 && response.images.length === 0) {
      response.user_input = '用户確認繼續'
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
    console.error('提交响应失敗:', error)
    message.error('提交失敗，请重试')
  }
  finally {
    submitting.value = false
  }
}

// 處理輸入更新
function handleInputUpdate(data: { userInput: string, selectedOptions: string[], draggedImages: string[] }) {
  userInput.value = data.userInput
  selectedOptions.value = data.selectedOptions
  draggedImages.value = data.draggedImages
}

// 處理图片新增 - 移除重复逻辑，避免双重新增
function handleImageAdd(_image: string) {
  // 这个函數现在只是为了保持介面相容性，实际新增在PopupInput中完成
}

// 處理图片移除
function handleImageRemove(index: number) {
  draggedImages.value.splice(index, 1)
}

// 「繼續」按鈕已移除，因為它會忽略使用者輸入

// 處理引用消息
function handleQuoteMessage(messageContent: string) {
  if (inputRef.value) {
    inputRef.value.handleQuoteMessage(messageContent)
  }
}

// 處理增强按钮点击
async function handleEnhance() {
  if (submitting.value)
    return

  submitting.value = true

  try {
    // 建構增强prompt - 简化版本
    const enhancePrompt = `請優化並增強以下指令，使其更清晰、更具體、更明確。考慮對話歷史的上下文，直接回覆增強後的版本，完成後使用「寸止」工具回報。

原始指令：
《${userInput.value.trim()}》`

    // 使用新的结构化資料格式
    const response = {
      user_input: enhancePrompt,
      selected_options: [],
      images: [],
      metadata: {
        timestamp: new Date().toISOString(),
        request_id: props.request?.id || null,
        source: 'popup_enhance',
      },
    }

    if (props.mockMode) {
      // 模拟模式下的延迟
      await new Promise(resolve => setTimeout(resolve, 1000))
      message.success('增强请求发送成功')
    }
    else {
      // 实际发送增强请求
      await invoke('send_mcp_response', { response })
      await invoke('exit_app')
    }

    emit('response', response)
  }
  catch (error) {
    console.error('发送增强请求失敗:', error)
    message.error('增强请求失敗，请重试')
  }
  finally {
    submitting.value = false
  }
}
</script>

<template>
  <div v-if="isVisible" class="flex flex-col flex-1">
    <!-- 内容区域 - 可滚动 -->
    <div class="flex-1 overflow-y-auto scrollbar-thin">
      <!-- 消息内容 - 允许选中 -->
      <div class="mx-2 mt-2 mb-1 px-4 py-3 bg-black-100 rounded-lg select-text" data-guide="popup-content">
        <PopupContent :request="request" :loading="loading" :current-theme="props.appConfig.theme" @quote-message="handleQuoteMessage" />
      </div>

      <!-- 輸入和選項 - 允许选中 -->
      <div class="px-4 pb-3 bg-black select-text">
        <PopupInput
          ref="inputRef" :request="request" :loading="loading" :submitting="submitting"
          @update="handleInputUpdate" @image-add="handleImageAdd" @image-remove="handleImageRemove"
        />
      </div>
    </div>

    <!-- 底部操作栏 - 固定在底部 -->
    <div class="flex-shrink-0 bg-black-100 border-t-2 border-black-200" data-guide="popup-actions">
      <PopupActions
        :request="request" :loading="loading" :submitting="submitting" :can-submit="canSubmit"
        :input-status-text="inputStatusText"
        @submit="handleSubmit" @enhance="handleEnhance"
      />
    </div>
  </div>
</template>
