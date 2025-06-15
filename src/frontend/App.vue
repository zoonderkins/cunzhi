<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

import { onMounted, ref } from 'vue'
import AppContent from './components/AppContent.vue'
import { useSettings } from './composables/useSettings'
import { useTheme } from './composables/useTheme'

// 响应式数据
const mcpRequest = ref(null)
const showMcpPopup = ref(false)
const isInitializing = ref(true)

const { currentTheme, naiveTheme, setTheme, loadTheme, setupSystemThemeListener } = useTheme()
const {
  alwaysOnTop,
  audioNotificationEnabled,
  audioUrl,
  setMessageInstance,
  loadWindowSettings,
  toggleAlwaysOnTop,
  toggleAudioNotification,
  updateAudioUrl,
  testAudioSound,
  stopAudioSound,
  updateWindowSize,
  loadWindowConfig,
} = useSettings()

// 统一的MCP响应处理
async function handleMcpResponse(response: any) {
  try {
    // 通过Tauri命令发送响应并退出应用
    await invoke('send_mcp_response', { response })
    await invoke('exit_app')
  }
  catch (error) {
    console.error('处理MCP响应失败:', error)
  }
}

// 统一的MCP取消处理
async function handleMcpCancel() {
  try {
    // 发送取消信息并退出应用
    await invoke('send_mcp_response', { response: 'CANCELLED' })
    await invoke('exit_app')
  }
  catch (error) {
    console.error('处理MCP取消失败:', error)
  }
}

// 显示MCP弹窗
async function showMcpDialog(request: any) {
  // 同时设置请求数据和显示状态，避免中间状态
  mcpRequest.value = request
  showMcpPopup.value = true

  // 播放音频通知
  try {
    await invoke('play_notification_sound')
  }
  catch (error) {
    console.error('显示MCP弹窗失败:', error)
  }
}

// 检查MCP模式
async function checkMcpMode() {
  try {
    const args = await invoke('get_cli_args')

    if (args && (args as any).mcp_request) {
      // 读取MCP请求文件
      const content = await invoke('read_mcp_request', { filePath: (args as any).mcp_request })

      if (content) {
        showMcpDialog(content)
      }
      return true // 表示是MCP模式
    }
  }
  catch (error) {
    console.error('检查MCP模式失败:', error)
  }
  return false // 表示不是MCP模式
}

// 设置MCP事件监听器
async function setupMcpEventListener() {
  try {
    await listen('mcp-request', (event) => {
      showMcpDialog(event.payload)
    })
  }
  catch (error) {
    console.error('设置MCP事件监听器失败:', error)
  }
}

// 处理消息实例就绪
function handleMessageReady(message: any) {
  setMessageInstance(message)
}

// 处理音频测试错误
function handleTestAudioError(error: any) {
  console.error('音频测试错误:', error)
  // 这里可以显示错误提示给用户
}

// 初始化
onMounted(async () => {
  // 首先加载主题设置
  await loadTheme()

  // 检查是否为MCP模式
  const isMcp = await checkMcpMode()

  // 无论是否为MCP模式，都加载窗口设置
  await loadWindowSettings()
  await loadWindowConfig()

  // 如果不是MCP模式，设置事件监听器
  if (!isMcp) {
    await setupMcpEventListener()
  }

  // 监听系统主题变化
  setupSystemThemeListener()

  // 结束初始化状态
  isInitializing.value = false
})
</script>

<template>
  <n-config-provider :theme="naiveTheme">
    <n-message-provider>
      <n-notification-provider>
        <n-dialog-provider>
          <AppContent
            :mcp-request="mcpRequest" :show-mcp-popup="showMcpPopup" :current-theme="currentTheme"
            :always-on-top="alwaysOnTop" :audio-notification-enabled="audioNotificationEnabled" :audio-url="audioUrl"
            :is-initializing="isInitializing"
            @mcp-response="handleMcpResponse" @mcp-cancel="handleMcpCancel" @theme-change="setTheme"
            @toggle-always-on-top="toggleAlwaysOnTop" @toggle-audio-notification="toggleAudioNotification"
            @update-audio-url="updateAudioUrl" @test-audio="testAudioSound" @stop-audio="stopAudioSound"
            @test-audio-error="handleTestAudioError" @update-window-size="updateWindowSize"
            @message-ready="handleMessageReady"
          />
        </n-dialog-provider>
      </n-notification-provider>
    </n-message-provider>
  </n-config-provider>
</template>
