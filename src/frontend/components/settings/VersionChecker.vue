<script setup lang="ts">
import { useMessage } from 'naive-ui'
import { computed, onMounted, ref } from 'vue'
import { useVersionCheck } from '../../composables/useVersionCheck'

const loading = ref(false)
const message = useMessage()

const {
  versionInfo,
  isChecking,
  lastCheckTime,
  isUpdating,
  updateProgress,
  updateStatus,
  manualCheckUpdate,
  getVersionInfo,
  openDownloadPage,
  openReleasePage,
  performOneClickUpdate,
  restartApp,
} = useVersionCheck()

// 格式化最后檢查時间
const formattedLastCheckTime = computed(() => {
  return lastCheckTime.value ? lastCheckTime.value.toLocaleString('zh-CN') : ''
})

// 手動檢查更新
async function handleCheckUpdate() {
  try {
    const info = await manualCheckUpdate()

    if (info?.hasUpdate) {
      message.info(`发现新版本 v${info.latest}！`)
    }
    else {
      message.success('當前已是最新版本')
    }
  }
  catch (error) {
    console.error('檢查版本更新失敗:', error)
    message.error(`檢查版本更新失敗: ${error}`)
  }
}

// 安全下载更新
async function handleDownloadUpdate() {
  try {
    await openDownloadPage()
    message.success('正在開啟下载页面...')
  }
  catch (error) {
    const errorMsg = error instanceof Error ? error.message : '開啟下载页面失敗，請手動访问GitHub'
    if (errorMsg.includes('已複製到剪贴板')) {
      message.warning(errorMsg)
    }
    else {
      message.error(errorMsg)
    }
  }
}

// 查看更新日誌
async function handleViewReleaseNotes() {
  try {
    await openReleasePage()
    message.success('正在開啟更新日誌...')
  }
  catch (error) {
    const errorMsg = error instanceof Error ? error.message : '開啟更新日誌失敗，請手動访问GitHub'
    if (errorMsg.includes('已複製到剪贴板')) {
      message.warning(errorMsg)
    }
    else {
      message.error(errorMsg)
    }
  }
}

// 一键更新
async function handleOneClickUpdate() {
  try {
    message.info('開始下载更新...')
    await performOneClickUpdate()

    if (updateStatus.value === 'completed') {
      message.success('更新完成！点击重新啟動按钮應用更新')
    }
  }
  catch (error) {
    console.error('一键更新失敗:', error)
    const errorMsg = error instanceof Error ? error.message : '更新失敗，請稍后重試或手動下载'
    message.error(errorMsg)
  }
}

// 重新啟動應用
async function handleRestartApp() {
  try {
    await restartApp()
  }
  catch (error) {
    console.error('重新啟動失敗:', error)
    message.error('重新啟動失敗，請手動重新啟動應用')
  }
}

// 元件挂载時初始化版本訊息
onMounted(async () => {
  loading.value = true
  try {
    await getVersionInfo()
  }
  catch (error) {
    console.error('初始化版本訊息失敗:', error)
  }
  finally {
    loading.value = false
  }
})
</script>

<template>
  <div class="space-y-4">
    <!-- 版本訊息顯示 -->
    <div
      v-if="!loading && versionInfo"
      class="space-y-3"
    >
      <div class="flex items-center justify-between">
        <span class="text-sm text-on-surface-secondary">當前版本:</span>
        <n-tag
          size="small"
          type="info"
        >
          v{{ versionInfo.current }}
        </n-tag>
      </div>

      <div
        v-if="versionInfo.latest !== versionInfo.current"
        class="flex items-center justify-between"
      >
        <span class="text-sm text-on-surface-secondary">最新版本:</span>
        <n-tag
          size="small"
          :type="versionInfo.hasUpdate ? 'warning' : 'success'"
        >
          v{{ versionInfo.latest }}
        </n-tag>
      </div>

      <!-- 更新提示 -->
      <div
        v-if="versionInfo.hasUpdate"
        class="p-3 bg-warning/10 dark:bg-warning/20 rounded-lg border border-warning/20 dark:border-warning/30"
      >
        <div class="flex items-start gap-2">
          <div class="i-carbon-warning text-warning mt-0.5" />
          <div class="flex-1">
            <p class="text-sm font-medium text-on-surface dark:text-on-surface">
              发现新版本 v{{ versionInfo.latest }}
            </p>
            <p class="text-xs text-on-surface-secondary dark:text-on-surface-secondary mt-1">
              建议更新到最新版本以获得更好的体验
            </p>
          </div>
        </div>
      </div>

      <!-- 更新进度顯示 -->
      <div
        v-if="isUpdating"
        class="p-3 bg-surface-100 dark:bg-surface-800 rounded-lg border border-surface-200 dark:border-surface-700"
      >
        <div class="space-y-2">
          <div class="flex items-center gap-2">
            <n-spin size="small" />
            <span class="text-sm font-medium text-on-surface dark:text-on-surface">
              {{ updateStatus === 'checking' ? '檢查更新中...'
                : updateStatus === 'downloading' ? '下载更新中...'
                  : updateStatus === 'installing' ? '安装更新中...'
                    : updateStatus === 'completed' ? '更新完成' : '更新中...' }}
            </span>
          </div>

          <!-- 下载进度条 -->
          <div
            v-if="updateProgress && updateStatus === 'downloading'"
            class="space-y-1"
          >
            <n-progress
              type="line"
              :percentage="Math.round(updateProgress.percentage)"
              :show-indicator="false"
              :height="6"
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

      <!-- 最后檢查時间 -->
      <div
        v-if="formattedLastCheckTime"
        class="text-xs text-on-surface-muted dark:text-on-surface-muted"
      >
        最后檢查: {{ formattedLastCheckTime }}
      </div>
    </div>

    <!-- 載入狀態 -->
    <div
      v-else-if="loading"
      class="flex items-center justify-center py-4"
    >
      <n-spin size="small" />
      <span class="ml-2 text-sm text-on-surface-secondary">載入版本訊息...</span>
    </div>

    <!-- 操作按钮 -->
    <div class="flex items-center gap-2 pt-2 border-t border-surface-200 dark:border-surface-700">
      <n-button
        size="small"
        :loading="isChecking"
        :disabled="isUpdating"
        @click="handleCheckUpdate"
      >
        <template #icon>
          <div class="i-carbon-renew" />
        </template>
        檢查更新
      </n-button>

      <!-- 立即更新按钮 -->
      <n-button
        v-if="versionInfo?.hasUpdate && updateStatus !== 'completed'"
        type="primary"
        size="small"
        :loading="isUpdating"
        @click="handleOneClickUpdate"
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
        size="small"
        @click="handleRestartApp"
      >
        <template #icon>
          <div class="i-carbon-restart" />
        </template>
        重新啟動應用
      </n-button>

      <!-- 手動下载按钮（备选方案） -->
      <n-button
        v-if="versionInfo?.hasUpdate"
        secondary
        size="small"
        :disabled="isUpdating"
        @click="handleDownloadUpdate"
      >
        <template #icon>
          <div class="i-carbon-download" />
        </template>
        手動下载
      </n-button>

      <n-button
        v-if="versionInfo?.releaseUrl"
        secondary
        size="small"
        :disabled="isUpdating"
        @click="handleViewReleaseNotes"
      >
        <template #icon>
          <div class="i-carbon-document" />
        </template>
        更新日誌
      </n-button>
    </div>
  </div>
</template>
