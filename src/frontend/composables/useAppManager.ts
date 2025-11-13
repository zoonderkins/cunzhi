import { computed } from 'vue'
import { useAppInitialization } from './useAppInitialization'
import { useAudioManager } from './useAudioManager'
import { useMcpHandler } from './useMcpHandler'
import { useSettings } from './useSettings'
import { useTheme } from './useTheme'

/**
 * 統一的應用管理器
 * 封裝所有組合式函數，提供簡潔的API
 */
export function useAppManager() {
  // 初始化各個模組
  const theme = useTheme()
  const settings = useSettings()
  const audioManager = useAudioManager()
  const mcpHandler = useMcpHandler()
  const appInit = useAppInitialization(mcpHandler)

  // 建立統一的設定物件
  const appConfig = computed(() => {
    const config = {
      theme: theme.currentTheme.value,
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
    }

    return config
  })

  // 建立統一的操作物件
  const actions = {
    // 主題操作
    theme: {
      setTheme: theme.setTheme,
    },
    // 設定操作
    settings: {
      toggleAlwaysOnTop: settings.toggleAlwaysOnTop,
      toggleAudioNotification: settings.toggleAudioNotification,
      updateAudioUrl: settings.updateAudioUrl,
      testAudio: settings.testAudioSound,
      stopAudio: settings.stopAudioSound,
      updateWindowSize: settings.updateWindowSize,
      updateReplyConfig: settings.updateReplyConfig,
      setMessageInstance: settings.setMessageInstance,
      reloadAllSettings: settings.reloadAllSettings,
    },
    // MCP操作
    mcp: {
      handleResponse: mcpHandler.handleMcpResponse,
      handleCancel: mcpHandler.handleMcpCancel,
    },
    // 音訊操作
    audio: {
      handleTestError: audioManager.handleTestAudioError,
    },
    // 應用操作
    app: {
      initialize: appInit.initializeApp,
      cleanup: () => {
        // 清理視窗焦點監聽器
        settings.removeWindowFocusListener()
      },
    },
  }

  // 傳回狀態和操作 - 保持響應式
  return {
    // 直接解构狀態，Vue模板会自動處理響應式
    naiveTheme: theme.naiveTheme,
    mcpRequest: mcpHandler.mcpRequest,
    showMcpPopup: mcpHandler.showMcpPopup,
    appConfig,
    isInitializing: appInit.isInitializing,

    // 操作
    actions,
  }
}
