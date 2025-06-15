<script setup lang="ts">
import type { McpRequest } from '../../types/popup'
import { useMessage } from 'naive-ui'
import { computed, ref, watch } from 'vue'

interface Props {
  request: McpRequest | null
  loading?: boolean
  submitting?: boolean
}

interface Emits {
  update: [data: {
    userInput: string
    selectedOptions: string[]
    draggedImages: string[]
  }]
  imageAdd: [image: string]
  imageRemove: [index: number]
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
  submitting: false,
})

const emit = defineEmits<Emits>()

// 响应式数据
const userInput = ref('')
const selectedOptions = ref<string[]>([])
const draggedImages = ref<string[]>([])
const textareaRef = ref<HTMLTextAreaElement | null>(null)

const message = useMessage()

// 计算属性
const hasOptions = computed(() => (props.request?.predefined_options?.length ?? 0) > 0)
const canSubmit = computed(() => {
  const hasOptionsSelected = selectedOptions.value.length > 0
  const hasInputText = userInput.value.trim().length > 0
  const hasImages = draggedImages.value.length > 0

  if (hasOptions.value) {
    return hasOptionsSelected || hasInputText
  }
  return hasInputText || hasImages
})

// 发送更新事件
function emitUpdate() {
  emit('update', {
    userInput: userInput.value,
    selectedOptions: selectedOptions.value,
    draggedImages: draggedImages.value,
  })
}

// 处理选项变化
function handleOptionChange(option: string, checked: boolean) {
  if (checked) {
    selectedOptions.value.push(option)
  }
  else {
    const idx = selectedOptions.value.indexOf(option)
    if (idx > -1)
      selectedOptions.value.splice(idx, 1)
  }
  emitUpdate()
}

// 处理选项切换（整行点击）
function handleOptionToggle(option: string) {
  const idx = selectedOptions.value.indexOf(option)
  if (idx > -1) {
    selectedOptions.value.splice(idx, 1)
  }
  else {
    selectedOptions.value.push(option)
  }
  emitUpdate()
}

// 处理图片拖拽
function handleImageDrop(event: DragEvent) {
  event.preventDefault()
  const files = event.dataTransfer?.files
  if (files) {
    handleImageFiles(files)
  }
}

function handleImagePaste(event: ClipboardEvent) {
  const items = event.clipboardData?.items
  let hasImage = false

  if (items) {
    for (const item of items) {
      if (item.type.includes('image')) {
        hasImage = true
        const file = item.getAsFile()
        if (file) {
          handleImageFiles([file])
        }
      }
    }
  }

  if (hasImage) {
    event.preventDefault()
  }
}

async function handleImageFiles(files: FileList | File[]) {
  for (const file of files) {
    if (file.type.startsWith('image/')) {
      try {
        const base64 = await fileToBase64(file)
        // 检查是否已存在相同图片，避免重复添加
        if (!draggedImages.value.includes(base64)) {
          draggedImages.value.push(base64)
          message.success(`图片 ${file.name} 已添加`)
          emitUpdate()
        }
        else {
          message.warning(`图片 ${file.name} 已存在`)
        }
      }
      catch {
        message.error(`图片 ${file.name} 处理失败`)
      }
    }
  }
}

function fileToBase64(file: File): Promise<string> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader()
    reader.onload = () => resolve(reader.result as string)
    reader.onerror = reject
    reader.readAsDataURL(file)
  })
}

function removeImage(index: number) {
  draggedImages.value.splice(index, 1)
  emit('imageRemove', index)
  emitUpdate()
}

// 监听用户输入变化
watch(userInput, () => {
  emitUpdate()
})

// 重置数据
function reset() {
  userInput.value = ''
  selectedOptions.value = []
  draggedImages.value = []
  emitUpdate()
}

// 暴露方法给父组件
defineExpose({
  reset,
  canSubmit,
})
</script>

<template>
  <div class="space-y-3">
    <!-- 预定义选项 -->
    <div v-if="!loading && hasOptions" class="space-y-3">
      <h4 class="text-sm font-medium text-theme-text">
        请选择选项
      </h4>
      <n-space vertical size="small">
        <div
          v-for="(option, index) in request!.predefined_options"
          :key="`option-${index}`"
          class="rounded-lg p-3 border border-theme-border bg-theme-card cursor-pointer hover:opacity-80 transition-opacity"
          @click="handleOptionToggle(option)"
        >
          <n-checkbox
            :value="option"
            :checked="selectedOptions.includes(option)"
            :disabled="submitting"
            size="medium"
            @update:checked="(checked: boolean) => handleOptionChange(option, checked)"
            @click.stop
          >
            {{ option }}
          </n-checkbox>
        </div>
      </n-space>
    </div>

    <!-- 图片预览区域 -->
    <div v-if="!loading && draggedImages.length > 0" class="space-y-3">
      <h4 class="text-sm font-medium text-theme-text">
        已添加的图片 ({{ draggedImages.length }})
      </h4>
      <div class="flex flex-wrap gap-2">
        <div
          v-for="(image, index) in draggedImages"
          :key="`image-${index}`"
          class="relative w-12 h-12 rounded-lg border border-theme-border bg-theme-card group"
          style="overflow: visible;"
        >
          <!-- 使用 n-image 组件进行渲染 -->
          <n-image
            :src="image"
            :alt="`上传的图片 ${index + 1}`"
            class="w-full h-full object-cover"
            :preview-src="image"
            show-toolbar-tooltip
            lazy
          />
          <!-- 删除按钮 -->
          <n-button
            class="absolute top-0 right-0 opacity-0 group-hover:opacity-100 transition-opacity duration-200"
            :disabled="submitting"
            size="tiny"
            type="error"
            circle
            style="width: 18px; height: 18px; min-width: 18px;"
            @click="removeImage(index)"
          >
            <template #icon>
              <div class="i-carbon-close w-2.5 h-2.5" />
            </template>
          </n-button>
          <!-- 图片序号 -->
          <div class="absolute bottom-2 left-2 px-2 py-1 bg-black bg-opacity-50 text-white text-xs rounded">
            {{ index + 1 }}
          </div>
        </div>
      </div>
    </div>

    <!-- 文本输入区域 -->
    <div v-if="!loading" class="space-y-3">
      <h4 class="text-sm font-medium text-theme-text">
        {{ hasOptions ? '补充说明 (可选)' : '请输入您的回复' }}
      </h4>

      <!-- 拖拽提示区域 -->
      <div class="relative rounded-lg border-2 border-dashed border-theme-border bg-theme-card p-4 transition-colors duration-200">
        <p class="text-xs text-center text-theme-text opacity-60">
          拖拽图片到此处或在输入框中粘贴图片 (⌘+V)
        </p>
      </div>

      <!-- 文本输入框 -->
      <n-input
        ref="textareaRef"
        v-model:value="userInput"
        type="textarea"
        size="small"
        :placeholder="hasOptions ? '您可以在这里添加补充说明...' : '请输入您的回复...'"
        :disabled="submitting"
        :autosize="{ minRows: 3, maxRows: 6 }"
        @drop="handleImageDrop"
        @dragover.prevent
        @dragenter.prevent
        @paste="handleImagePaste"
      />
    </div>
  </div>
</template>
