<script setup lang="ts">
import { computed, onMounted } from 'vue'
import AppContent from './components/AppContent.vue'
import { useAppInitialization } from './composables/useAppInitialization'
import { useAudioManager } from './composables/useAudioManager'
import { useMcpHandler } from './composables/useMcpHandler'
import { useSettings } from './composables/useSettings'
import { useTheme } from './composables/useTheme'

// 使用组合式函数
const { currentTheme, naiveTheme, setTheme } = useTheme()
const settings = useSettings()
const { isInitializing, initializeApp } = useAppInitialization()
const { handleTestAudioError } = useAudioManager()
const {
  mcpRequest,
  showMcpPopup,
  handleMcpResponse,
  handleMcpCancel,
} = useMcpHandler()

// 创建配置对象
const appConfig = computed(() => ({
  theme: currentTheme.value,
  window: {
    alwaysOnTop: settings.alwaysOnTop.value,
    width: settings.windowWidth.value,
    height: settings.windowHeight.value,
    fixed: settings.fixedWindowSize.value,
  },
  audio: {
    enabled: settings.audioNotificationEnabled.value,
    url: settings.audioUrl.value,
  },
  reply: {
    enabled: settings.continueReplyEnabled.value,
    prompt: settings.continuePrompt.value,
  },
}))

// 创建设置操作对象
const settingsActions = {
  toggleAlwaysOnTop: settings.toggleAlwaysOnTop,
  toggleAudioNotification: settings.toggleAudioNotification,
  updateAudioUrl: settings.updateAudioUrl,
  testAudio: settings.testAudioSound,
  stopAudio: settings.stopAudioSound,
  updateWindowSize: settings.updateWindowSize,
  updateReplyConfig: settings.updateReplyConfig,
}

// 处理消息实例就绪
function handleMessageReady(message: any) {
  settings.setMessageInstance(message)
}

// 初始化
onMounted(async () => {
  try {
    await initializeApp()
  }
  catch (error) {
    console.error('应用初始化失败:', error)
  }
})
</script>

<template>
  <div class="min-h-screen bg-black">
    <n-config-provider :theme="naiveTheme">
      <n-message-provider>
        <n-notification-provider>
          <n-dialog-provider>
            <AppContent
              :mcp-request="mcpRequest" :show-mcp-popup="showMcpPopup" :app-config="appConfig"
              :is-initializing="isInitializing" @mcp-response="handleMcpResponse" @mcp-cancel="handleMcpCancel"
              @theme-change="setTheme" @toggle-always-on-top="settingsActions.toggleAlwaysOnTop"
              @toggle-audio-notification="settingsActions.toggleAudioNotification"
              @update-audio-url="settingsActions.updateAudioUrl" @test-audio="settingsActions.testAudio"
              @stop-audio="settingsActions.stopAudio" @test-audio-error="handleTestAudioError"
              @update-window-size="settingsActions.updateWindowSize"
              @update-reply-config="settingsActions.updateReplyConfig" @message-ready="handleMessageReady"
            />
          </n-dialog-provider>
        </n-notification-provider>
      </n-message-provider>
    </n-config-provider>
  </div>
</template>
