<script setup lang="ts">
import { useMessage } from 'naive-ui'
import { computed } from 'vue'
import { useVersionCheck } from '../../composables/useVersionCheck'

interface Props {
  show: boolean
  versionInfo: {
    current: string
    latest: string
    hasUpdate: boolean
    releaseUrl: string
    releaseNotes: string
  } | null
}

const props = defineProps<Props>()
const emit = defineEmits<{
  'update:show': [value: boolean]
}>()

const message = useMessage()
const {
  isUpdating,
  updateStatus,
  updateProgress,
  performOneClickUpdate,
  restartApp,
  dismissUpdate,
} = useVersionCheck()

// 简单的文本格式化（将换行转换为HTML）
const formattedReleaseNotes = computed(() => {
  if (!props.versionInfo?.releaseNotes)
    return ''
  return props.versionInfo.releaseNotes
    .replace(/\n/g, '<br>')
    .replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>')
    .replace(/\*(.*?)\*/g, '<em>$1</em>')
    .replace(/`(.*?)`/g, '<code>$1</code>')
})

const isVisible = computed({
  get: () => props.show,
  set: value => emit('update:show', value),
})

// 確認更新
async function handleConfirmUpdate() {
  try {
    message.info('正在准备更新...')
    await performOneClickUpdate()

    if (updateStatus.value === 'completed') {
      message.success('更新完成！')
    }
  }
  catch (error) {
    const errorMsg = error instanceof Error ? error.message : String(error)
    console.error('❌ 更新失敗:', errorMsg)

    // 如果是需要手動下载的錯誤，引导用户手動下载
    if (errorMsg.includes('手動下载') || errorMsg.includes('網路请求受限') || errorMsg.includes('403')) {
      let warningMsg = '自動更新不可用，将为您開啟下载页面'

      if (errorMsg.includes('網路请求受限') || errorMsg.includes('403')) {
        warningMsg = '網路请求受限，将为您開啟下载页面'
      }

      message.warning(warningMsg)

      // 開啟下载页面
      if (props.versionInfo?.releaseUrl) {
        try {
          window.open(props.versionInfo.releaseUrl, '_blank')
        }
        catch (openError) {
          console.error('❌ 開啟下载页面失敗:', openError)
          message.error('无法開啟下载页面，请手動访问 GitHub 下载最新版本')
        }
      }
      else {
        message.error('无法獲取下载連結，请手動访问 GitHub 下载最新版本')
      }

      // 延迟關閉弹窗，让用户看到提示
      setTimeout(() => {
        isVisible.value = false
      }, 2000)
    }
    else {
      // 其他錯誤显示具体錯誤訊息
      let displayMsg = errorMsg || '更新失敗，请稍后重试'

      // 檢查是否是網路相关錯誤
      if (errorMsg.includes('網路') || errorMsg.includes('连接') || errorMsg.includes('请求失敗')
        || errorMsg.includes('timeout') || errorMsg.includes('ENOTFOUND') || errorMsg.includes('ECONNREFUSED')) {
        displayMsg = '網路连接例外，请檢查網路后重试'
      }

      message.error(`更新失敗: ${displayMsg}`)
    }
  }
}

// 關閉弹窗（不再提醒）
function handleDismiss() {
  dismissUpdate()
  message.info('已關閉更新提醒')
}

// 重新啟動應用
async function handleRestart() {
  try {
    await restartApp()
  }
  catch (error) {
    console.error('重新啟動失敗:', error)
    message.error('重新啟動失敗，请手動重新啟動應用')
  }
}
</script>

<template>
  <n-modal
    v-model:show="isVisible"
    :mask-closable="false"
    :close-on-esc="false"
    preset="dialog"
    class="max-w-lg"
    :style="{ maxHeight: '80vh' }"
  >
    <template #header>
      <div class="flex items-center gap-3">
        <div class="i-carbon-upgrade text-xl text-blue-500" />
        <span class="font-medium text-lg">🚀 发现新版本</span>
      </div>
    </template>

    <div class="space-y-4">
      <!-- 版本訊息 -->
      <div v-if="versionInfo" class="space-y-3">
        <div class="p-4 bg-surface-100 dark:bg-surface-800 rounded-lg border border-surface-200 dark:border-surface-700">
          <div class="flex items-center justify-between mb-3">
            <span class="text-sm text-on-surface-secondary">当前版本:</span>
            <n-tag size="small" type="info">
              v{{ versionInfo.current }}
            </n-tag>
          </div>
          <div class="flex items-center justify-between">
            <span class="text-sm text-on-surface-secondary">最新版本:</span>
            <n-tag size="small" type="success">
              v{{ versionInfo.latest }}
            </n-tag>
          </div>
        </div>

        <!-- 更新进度 -->
        <div v-if="isUpdating" class="p-4 bg-blue-50 dark:bg-blue-900/30 rounded-lg border border-blue-200 dark:border-blue-700">
          <div class="space-y-3">
            <div class="flex items-center gap-2">
              <n-spin size="small" />
              <span class="text-sm font-medium text-on-surface dark:text-on-surface">
                {{ updateStatus === 'checking' ? '檢查更新中...'
                  : updateStatus === 'downloading' ? '下载更新中...'
                    : updateStatus === 'installing' ? '安装更新中...'
                      : updateStatus === 'completed' ? '更新完成！'
                        : '更新中...' }}
              </span>
            </div>

            <!-- 下载进度条 -->
            <div v-if="updateProgress && updateStatus === 'downloading'" class="space-y-2">
              <n-progress
                type="line"
                :percentage="Math.round(updateProgress.percentage)"
                :show-indicator="false"
                :height="8"
                color="#3b82f6"
              />
              <div class="flex justify-between text-xs text-on-surface-secondary dark:text-on-surface-secondary">
                <span>{{ Math.round(updateProgress.downloaded / 1024 / 1024 * 100) / 100 }}MB</span>
                <span v-if="updateProgress.content_length">
                  / {{ Math.round(updateProgress.content_length / 1024 / 1024 * 100) / 100 }}MB
                </span>
                <span>{{ Math.round(updateProgress.percentage) }}%</span>
              </div>
            </div>
          </div>
        </div>

        <!-- 更新说明 -->
        <div v-if="versionInfo.releaseNotes && !isUpdating" class="space-y-3">
          <div class="flex items-center gap-2">
            <div class="i-carbon-document text-blue-500" />
            <h4 class="text-sm font-medium text-on-surface">
              更新内容
            </h4>
          </div>
          <div class="max-h-40 overflow-y-auto">
            <div class="text-sm p-4 rounded-lg border bg-surface-50 dark:bg-surface-900 border-surface-200 dark:border-surface-700 text-on-surface-secondary">
              <div
                class="release-notes-content space-y-2"
                v-html="formattedReleaseNotes"
              />
            </div>
          </div>
        </div>
      </div>
    </div>

    <template #action>
      <div class="flex justify-end gap-3">
        <!-- 關閉按钮 -->
        <n-button
          v-if="updateStatus !== 'completed'"
          :disabled="isUpdating"
          @click="handleDismiss"
        >
          關閉
        </n-button>

        <!-- 立即更新按钮 -->
        <n-button
          v-if="updateStatus !== 'completed'"
          type="primary"
          :loading="isUpdating"
          @click="handleConfirmUpdate"
        >
          <template #icon>
            <div class="i-carbon-upgrade" />
          </template>
          立即更新
        </n-button>

        <!-- 重新啟動按钮 -->
        <n-button
          v-if="updateStatus === 'completed'"
          type="success"
          @click="handleRestart"
        >
          <template #icon>
            <div class="i-carbon-restart" />
          </template>
          重新啟動應用
        </n-button>
      </div>
    </template>
  </n-modal>
</template>

<style scoped>
.release-notes-content :deep(h1),
.release-notes-content :deep(h2),
.release-notes-content :deep(h3),
.release-notes-content :deep(h4) {
  font-weight: 600;
  margin: 0.75rem 0 0.5rem 0;
  color: var(--text-color-1);
}

.release-notes-content :deep(h2) {
  font-size: 1.1em;
  border-bottom: 1px solid var(--border-color);
  padding-bottom: 0.25rem;
}

.release-notes-content :deep(h3) {
  font-size: 1em;
}

.release-notes-content :deep(p) {
  margin: 0.5rem 0;
  line-height: 1.5;
}

.release-notes-content :deep(ul),
.release-notes-content :deep(ol) {
  margin: 0.5rem 0;
  padding-left: 1.5rem;
}

.release-notes-content :deep(li) {
  margin: 0.25rem 0;
  line-height: 1.4;
}

.release-notes-content :deep(strong) {
  font-weight: 600;
  color: var(--text-color-1);
}

.release-notes-content :deep(em) {
  font-style: italic;
}

.release-notes-content :deep(code) {
  padding: 0.125rem 0.375rem;
  font-size: 0.875em;
  border-radius: 0.25rem;
  font-family: ui-monospace, SFMono-Regular, 'SF Mono', monospace;
  background-color: var(--code-color);
  color: var(--text-color-1);
  border: 1px solid var(--border-color);
}

.release-notes-content :deep(blockquote) {
  margin: 0.75rem 0;
  padding: 0.5rem 1rem;
  border-left: 3px solid var(--primary-color);
  background-color: var(--code-color);
  border-radius: 0 0.25rem 0.25rem 0;
}
</style>
