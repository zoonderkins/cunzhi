<script setup lang="ts">
import { useMessage } from 'naive-ui'
import { computed, onMounted } from 'vue'
import { useVersionCheck } from '../../composables/useVersionCheck'

const message = useMessage()
const { versionInfo, manualCheckUpdate, safeOpenUrl, lastCheckTime, isChecking, getVersionInfo } = useVersionCheck()

// 格式化最后檢查时间
const formattedLastCheckTime = computed(() => {
  return lastCheckTime.value ? lastCheckTime.value.toLocaleString('zh-CN') : ''
})

// 安全開啟GitHub連結
async function openGitHub() {
  try {
    await safeOpenUrl('https://github.com/zoonderkins/cunzhi')
    message.success('正在開啟GitHub页面...')
  }
  catch (error) {
    const errorMsg = error instanceof Error ? error.message : '開啟GitHub失敗，请手動访问'
    if (errorMsg.includes('已複製到剪贴板')) {
      message.warning(errorMsg)
    }
    else {
      message.error(errorMsg)
    }
  }
}

// 安全開啟GitHub Star页面
async function openGitHubStars() {
  try {
    await safeOpenUrl('https://github.com/zoonderkins/cunzhi/stargazers')
    message.success('正在開啟Star页面...')
  }
  catch (error) {
    const errorMsg = error instanceof Error ? error.message : '開啟Star页面失敗，请手動访问'
    if (errorMsg.includes('已複製到剪贴板')) {
      message.warning(errorMsg)
    }
    else {
      message.error(errorMsg)
    }
  }
}

// 檢查版本更新
async function checkVersion() {
  try {
    const info = await manualCheckUpdate()
    if (info?.hasUpdate) {
      message.info(`发现新版本 v${info.latest}！`)
    }
    else {
      message.success('当前已是最新版本')
    }
  }
  catch (error) {
    console.error('檢查版本失敗:', error)
    message.error('檢查版本失敗，请稍后重试')
  }
}

// 元件挂载时初始化版本訊息
onMounted(async () => {
  try {
    await getVersionInfo()
  }
  catch (error) {
    console.error('初始化版本訊息失敗:', error)
  }
})
</script>

<template>
  <n-card
    size="small"
    class="transition-all duration-200 hover:shadow-md"
  >
    <!-- 主要内容区域 -->
    <div class="flex items-center justify-between mb-2">
      <!-- 左侧：專案訊息 -->
      <div class="flex items-center gap-3">
        <div class="w-8 h-8 rounded-lg bg-blue-100 dark:bg-blue-900 flex items-center justify-center">
          <div class="i-carbon-logo-github text-blue-600 dark:text-blue-400" />
        </div>
        <div>
          <h3 class="font-semibold text-gray-900 dark:text-white text-sm">
            寸止 {{ versionInfo ? `v${versionInfo.current}` : 'v0.2.0' }}
          </h3>
          <p class="text-xs text-gray-500 dark:text-gray-400">
            智慧程式碼審查工具，支持MCP协议集成
          </p>
        </div>
      </div>

      <!-- 右侧：版本檢查区域 -->
      <div class="flex flex-col items-end gap-1">
        <n-button
          size="medium"
          secondary
          :loading="isChecking"
          @click="checkVersion"
        >
          <template #icon>
            <div class="i-carbon-renew text-green-600 dark:text-green-400" />
          </template>
          檢查更新
        </n-button>

        <!-- 最后檢查时间 -->
        <div
          v-if="formattedLastCheckTime"
          class="text-xs text-gray-400 dark:text-gray-500"
        >
          最后檢查: {{ formattedLastCheckTime }}
        </div>
      </div>
    </div>

    <!-- 底部：GitHub区域 -->
    <div class="flex items-center justify-between border-t border-gray-100 dark:border-gray-700 pt-2">
      <div class="flex items-center gap-1">
        <n-button
          size="medium"
          type="primary"
          @click="openGitHub"
        >
          <template #icon>
            <div class="i-carbon-logo-github" />
          </template>
          GitHub
        </n-button>

        <n-button
          size="medium"
          secondary
          @click="openGitHubStars"
        >
          <template #icon>
            <div class="i-carbon-star text-yellow-500" />
          </template>
          Star
        </n-button>
      </div>

      <!-- 弱化的提示文字 -->
      <p class="text-xs text-gray-400 dark:text-gray-500">
        如果对您有幫助，请给我们一个 Star ⭐
      </p>
    </div>
  </n-card>
</template>
