<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { useMessage } from 'naive-ui'
import { ref } from 'vue'
import { useI18n } from '../../i18n'
import IntroTab from '../tabs/IntroTab.vue'
import McpToolsTab from '../tabs/McpToolsTab.vue'
import PromptsTab from '../tabs/PromptsTab.vue'
import SettingsTab from '../tabs/SettingsTab.vue'

const { t } = useI18n()

interface Props {
  currentTheme: string
  alwaysOnTop: boolean
  audioNotificationEnabled: boolean
  audioUrl: string
  windowWidth: number
  windowHeight: number
  fixedWindowSize: boolean
}

interface Emits {
  themeChange: [theme: string]
  toggleAlwaysOnTop: []
  toggleAudioNotification: []
  updateAudioUrl: [url: string]
  testAudio: []
  stopAudio: []
  testAudioError: [error: any]
  updateWindowSize: [size: { width: number, height: number, fixed: boolean }]
  configReloaded: []
}

defineProps<Props>()
const emit = defineEmits<Emits>()

// 處理設定重新載入事件
function handleConfigReloaded() {
  emit('configReloaded')
}

const activeTab = ref('intro')
const message = useMessage()

// 图标載入錯誤處理
function handleImageError(event: Event) {
  const img = event.target as HTMLImageElement
  // 如果图标載入失敗，隐藏图片元素
  img.style.display = 'none'
  console.warn('LOGO图标載入失敗，已隐藏')
}

// 測試popup功能 - 建立独立的popup視窗
async function showTestMcpPopup() {
  try {
    // 建立測試请求資料
    const testRequest = {
      id: `test-${Date.now()}`,
      message: `# 🧪 測試弹窗功能

这是一个**測試弹窗**，用于驗證MCP popup元件的功能。

## 功能特性
- ✅ 支持 Markdown 格式显示
- ✅ 支持预定义選項選擇
- ✅ 支持自由文本輸入
- ✅ 支持图片貼上上传

## 代码示例
\`\`\`javascript
// 这是一个代码示例
function testPopup() {
  console.log('測試弹窗功能')
  return '成功'
}
\`\`\`

请選擇您要測試的功能，或者在下方輸入框中新增您的反馈。`,
      predefined_options: ['測試選項功能', '測試文本輸入', '測試图片上传', '測試Markdown渲染'],
      is_markdown: true,
    }

    // 呼叫Tauri命令建立popup視窗
    await invoke('create_test_popup', { request: testRequest })
    message.success('測試popup視窗已建立')
  }
  catch (error) {
    console.error('建立測試popup失敗:', error)
    message.error(`建立測試popup失敗: ${error}`)
  }
}
</script>

<template>
  <div class="flex flex-col min-h-screen">
    <!-- 主要内容区域 -->
    <div class="flex-1 flex items-start justify-center p-6 pt-12">
      <div class="max-w-6xl w-full">
        <!-- 标题区域 -->
        <div class="text-center mb-8">
          <!-- 主标题 -->
          <div class="flex items-center justify-center gap-3 mb-3" data-guide="app-logo">
            <img
              src="/icons/icon-128.png"
              alt="寸止 Logo"
              class="w-10 h-10 rounded-xl shadow-lg"
              @error="handleImageError"
            >
            <h1 class="text-4xl font-medium text-white">
              寸止
            </h1>
            <!-- 測試按钮 -->
            <n-button
              size="small"
              type="tertiary"
              circle
              title="測試 Popup 功能"
              class="ml-2"
              data-guide="test-button"
              @click="showTestMcpPopup"
            >
              <template #icon>
                <div class="i-carbon-test-tool w-4 h-4" />
              </template>
            </n-button>
          </div>

          <!-- 服务器狀態 -->
          <div class="mb-4">
            <n-tag type="success" size="small" round class="px-3 py-1">
              <template #icon>
                <div class="w-2 h-2 bg-success rounded-full animate-pulse" />
              </template>
              MCP 服务已啟動
            </n-tag>
          </div>

          <!-- 副标题 -->
          <p class="text-base opacity-50 font-normal text-white">
            讓 AI Great Again! 持久
          </p>
        </div>

        <!-- Tab元件 -->
        <n-tabs v-model:value="activeTab" type="segment" size="small" justify-content="center" data-guide="tabs">
          <n-tab-pane name="intro" :tab="t('tabs.intro')">
            <IntroTab />
          </n-tab-pane>
          <n-tab-pane name="mcp-tools" :tab="t('tabs.mcpTools')">
            <McpToolsTab />
          </n-tab-pane>
          <n-tab-pane name="prompts" :tab="t('tabs.prompts')">
            <PromptsTab />
          </n-tab-pane>
          <n-tab-pane name="settings" :tab="t('tabs.settings')" data-guide="settings-tab">
            <SettingsTab
              :current-theme="currentTheme"
              :always-on-top="alwaysOnTop"
              :audio-notification-enabled="audioNotificationEnabled"
              :audio-url="audioUrl"
              :window-width="windowWidth"
              :window-height="windowHeight"
              :fixed-window-size="fixedWindowSize"
              @theme-change="$emit('themeChange', $event)"
              @toggle-always-on-top="$emit('toggleAlwaysOnTop')"
              @toggle-audio-notification="$emit('toggleAudioNotification')"
              @update-audio-url="$emit('updateAudioUrl', $event)"
              @test-audio="$emit('testAudio')"
              @stop-audio="$emit('stopAudio')"
              @test-audio-error="$emit('testAudioError', $event)"
              @update-window-size="$emit('updateWindowSize', $event)"
              @config-reloaded="handleConfigReloaded"
            />
          </n-tab-pane>
        </n-tabs>
      </div>
    </div>
  </div>
</template>
