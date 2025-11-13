<script setup lang="ts">
import type { McpRequest } from '../../types/popup'
import { computed, onMounted } from 'vue'
import { useShortcuts } from '../../composables/useShortcuts'

interface Props {
  request: McpRequest | null
  loading?: boolean
  submitting?: boolean
  canSubmit?: boolean
  connectionStatus?: string
  inputStatusText?: string
}

interface Emits {
  submit: []
  enhance: []
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
  submitting: false,
  canSubmit: false,
  connectionStatus: '已連接',
  inputStatusText: '',
})

const emit = defineEmits<Emits>()

// 使用自訂快捷键系統
const {
  quickSubmitShortcutText,
  enhanceShortcutText,
  useQuickSubmitShortcut,
  useEnhanceShortcut,
  loadShortcutConfig,
} = useShortcuts()

const shortcutText = quickSubmitShortcutText

const statusText = computed(() => {
  // 如果可以提交，直接顯示快捷键提示
  if (props.canSubmit) {
    return shortcutText.value
  }

  // 如果有輸入狀態文本且不是預設狀態，顯示輸入狀態
  if (props.inputStatusText && props.inputStatusText !== '等待輸入...') {
    return props.inputStatusText
  }

  // 根据請求類型顯示不同的提示
  if (props.request?.predefined_options) {
    return '選擇選項或輸入文本'
  }
  return '請輸入內容'
})

// 處理快捷键
useQuickSubmitShortcut(() => {
  if (props.canSubmit && !props.submitting) {
    handleSubmit()
  }
})

useEnhanceShortcut(() => {
  if (!props.submitting) {
    handleEnhance()
  }
})

function handleSubmit() {
  if (props.canSubmit && !props.submitting) {
    emit('submit')
  }
}

function handleEnhance() {
  if (!props.submitting) {
    emit('enhance')
  }
}

// 元件挂载時載入快捷鍵設定
onMounted(() => {
  loadShortcutConfig()
})
</script>

<template>
  <div class="px-4 py-3 bg-gray-100 min-h-[60px] select-none">
    <div v-if="!loading" class="flex justify-between items-center">
      <!-- 左侧狀態訊息 -->
      <div class="flex items-center">
        <div class="flex items-center gap-2 text-xs text-gray-600">
          <div class="w-2 h-2 rounded-full bg-primary-500" />
          <span class="font-medium">{{ connectionStatus }}</span>
          <span class="opacity-60">|</span>
          <span class="opacity-60">{{ statusText }}</span>
        </div>
      </div>

      <!-- 右侧操作按钮 -->
      <div class="flex items-center" data-guide="popup-actions">
        <n-space size="small">
          <!-- 增强按钮 -->
          <n-tooltip trigger="hover" placement="top">
            <template #trigger>
              <n-button
                :disabled="!canSubmit || submitting"
                size="medium"
                type="info"
                data-guide="enhance-button"
                @click="handleEnhance"
              >
                <template #icon>
                  <div class="i-carbon-magic-wand w-4 h-4" />
                </template>
                增强
              </n-button>
            </template>
            {{ enhanceShortcutText }}
          </n-tooltip>

          <!-- 傳送按钮 -->
          <n-tooltip trigger="hover" placement="top">
            <template #trigger>
              <n-button
                type="primary"
                :disabled="!canSubmit || submitting"
                :loading="submitting"
                size="medium"
                data-guide="submit-button"
                @click="handleSubmit"
              >
                <template #icon>
                  <div v-if="!submitting" class="i-carbon-send w-4 h-4" />
                </template>
                {{ submitting ? '傳送中...' : '傳送' }}
              </n-button>
            </template>
            {{ shortcutText }}
          </n-tooltip>
        </n-space>
      </div>
    </div>
  </div>
</template>
