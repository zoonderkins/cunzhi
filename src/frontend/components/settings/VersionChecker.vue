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
  checkLatestVersion,
  getVersionInfo,
  openDownloadPage,
  openReleasePage,
} = useVersionCheck()

// 格式化最后检查时间
const formattedLastCheckTime = computed(() => {
  return lastCheckTime.value ? lastCheckTime.value.toLocaleString('zh-CN') : ''
})

// 手动检查更新
async function handleCheckUpdate() {
  try {
    const info = await checkLatestVersion()

    if (info?.hasUpdate) {
      message.info(`发现新版本 v${info.latest}！`)
    } else {
      message.success('当前已是最新版本')
    }
  }
  catch (error) {
    console.error('检查版本更新失败:', error)
    message.error(`检查版本更新失败: ${error}`)
  }
}

// 安全下载更新
async function handleDownloadUpdate() {
  try {
    await openDownloadPage()
    message.success('正在打开下载页面...')
  }
  catch (error) {
    const errorMsg = error instanceof Error ? error.message : '打开下载页面失败，请手动访问GitHub'
    if (errorMsg.includes('已复制到剪贴板')) {
      message.warning(errorMsg)
    } else {
      message.error(errorMsg)
    }
  }
}

// 查看更新日志
async function handleViewReleaseNotes() {
  try {
    await openReleasePage()
    message.success('正在打开更新日志...')
  }
  catch (error) {
    const errorMsg = error instanceof Error ? error.message : '打开更新日志失败，请手动访问GitHub'
    if (errorMsg.includes('已复制到剪贴板')) {
      message.warning(errorMsg)
    } else {
      message.error(errorMsg)
    }
  }
}

// 组件挂载时初始化版本信息
onMounted(async () => {
  loading.value = true
  try {
    await getVersionInfo()
  }
  catch (error) {
    console.error('初始化版本信息失败:', error)
  }
  finally {
    loading.value = false
  }
})
</script>

<template>
  <n-card size="small">
    <template #header>
      <div class="flex items-center gap-2">
        <div class="i-carbon-upgrade text-lg text-blue-500" />
        <span class="font-medium">版本信息</span>
      </div>
    </template>
    
    <div class="space-y-4">
      <!-- 版本信息显示 -->
      <div v-if="!loading && versionInfo" class="space-y-3">
        <div class="flex items-center justify-between">
          <span class="text-sm text-gray-600 dark:text-gray-400">当前版本:</span>
          <n-tag size="small" type="info">
            v{{ versionInfo.current }}
          </n-tag>
        </div>
        
        <div v-if="versionInfo.latest !== versionInfo.current" class="flex items-center justify-between">
          <span class="text-sm text-gray-600 dark:text-gray-400">最新版本:</span>
          <n-tag size="small" :type="versionInfo.hasUpdate ? 'warning' : 'success'">
            v{{ versionInfo.latest }}
          </n-tag>
        </div>
        
        <!-- 更新提示 -->
        <div v-if="versionInfo.hasUpdate" class="p-3 bg-orange-50 dark:bg-orange-900/20 rounded-lg border border-orange-200 dark:border-orange-800">
          <div class="flex items-start gap-2">
            <div class="i-carbon-warning text-orange-500 mt-0.5" />
            <div class="flex-1">
              <p class="text-sm font-medium text-orange-800 dark:text-orange-200">
                发现新版本 v{{ versionInfo.latest }}
              </p>
              <p class="text-xs text-orange-600 dark:text-orange-300 mt-1">
                建议更新到最新版本以获得更好的体验
              </p>
            </div>
          </div>
        </div>
        
        <!-- 最后检查时间 -->
        <div v-if="formattedLastCheckTime" class="text-xs text-gray-500 dark:text-gray-500">
          最后检查: {{ formattedLastCheckTime }}
        </div>
      </div>
      
      <!-- 加载状态 -->
      <div v-else-if="loading" class="flex items-center justify-center py-4">
        <n-spin size="small" />
        <span class="ml-2 text-sm text-gray-500">加载版本信息...</span>
      </div>
      
      <!-- 操作按钮 -->
      <div class="flex items-center gap-2 pt-2 border-t border-gray-200 dark:border-gray-700">
        <n-button
          size="small"
          :loading="isChecking"
          @click="handleCheckUpdate"
        >
          <template #icon>
            <div class="i-carbon-renew" />
          </template>
          检查更新
        </n-button>
        
        <n-button
          v-if="versionInfo?.hasUpdate"
          type="primary"
          size="small"
          @click="handleDownloadUpdate"
        >
          <template #icon>
            <div class="i-carbon-download" />
          </template>
          下载更新
        </n-button>
        
        <n-button
          v-if="versionInfo?.releaseUrl"
          secondary
          size="small"
          @click="handleViewReleaseNotes"
        >
          <template #icon>
            <div class="i-carbon-document" />
          </template>
          更新日志
        </n-button>
      </div>
    </div>
  </n-card>
</template>
