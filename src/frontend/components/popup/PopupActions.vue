<script setup lang="ts">
import type { McpRequest } from '../../types/popup'
import { useMagicKeys } from '@vueuse/core'
import { computed, watch } from 'vue'

interface Props {
  request: McpRequest | null
  loading?: boolean
  submitting?: boolean
  canSubmit?: boolean
  connectionStatus?: string
  continueReplyEnabled?: boolean
}

interface Emits {
  submit: []
  continue: []
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
  submitting: false,
  canSubmit: false,
  connectionStatus: '已连接',
  continueReplyEnabled: true,
})

const emit = defineEmits<Emits>()

// 跨平台快捷键支持
const keys = useMagicKeys()
const ctrlEnter = keys['Ctrl+Enter']
const metaEnter = keys['Meta+Enter']

// 检测平台并显示正确的快捷键提示
const isMac = computed(() => {
  return navigator.userAgent.toUpperCase().includes('MAC')
})

const shortcutText = computed(() => {
  return isMac.value ? '⌘+回车 快速发送' : 'Ctrl+回车 快速发送'
})

const statusText = computed(() => {
  if (props.canSubmit) {
    return shortcutText.value
  }
  if (props.request?.predefined_options) {
    return '选择选项或输入文本'
  }
  return '请输入内容'
})

// 处理快捷键
watch([ctrlEnter, metaEnter], ([ctrlPressed, metaPressed]) => {
  if ((ctrlPressed || metaPressed) && props.canSubmit && !props.submitting) {
    handleSubmit()
  }
})

function handleSubmit() {
  if (props.canSubmit && !props.submitting) {
    emit('submit')
  }
}

function handleContinue() {
  if (!props.submitting) {
    emit('continue')
  }
}
</script>

<template>
  <div class="px-4 py-3 bg-theme-card min-h-[60px] select-none">
    <div v-if="!loading" class="flex justify-between items-center">
      <!-- 左侧状态信息 -->
      <div class="flex items-center">
        <div class="flex items-center gap-2 text-xs text-theme-text opacity-70">
          <div class="w-2 h-2 rounded-full bg-primary-500" />
          <span class="font-medium">{{ connectionStatus }}</span>
          <span class="opacity-60">|</span>
          <span class="opacity-60">{{ statusText }}</span>
        </div>
      </div>

      <!-- 右侧操作按钮 -->
      <div class="flex items-center">
        <n-space size="small">
          <!-- 继续按钮 -->
          <n-button
            v-if="continueReplyEnabled"
            :disabled="submitting"
            :loading="submitting"
            size="medium"
            type="default"
            @click="handleContinue"
          >
            <template #icon>
              <div class="i-carbon-play w-4 h-4" />
            </template>
            继续
          </n-button>

          <!-- 发送按钮 -->
          <n-button
            type="primary"
            :disabled="!canSubmit || submitting"
            :loading="submitting"
            size="medium"
            @click="handleSubmit"
          >
            <template #icon>
              <div v-if="!submitting" class="i-carbon-send w-4 h-4" />
            </template>
            {{ submitting ? '发送中...' : '发送' }}
          </n-button>
        </n-space>
      </div>
    </div>
  </div>
</template>
