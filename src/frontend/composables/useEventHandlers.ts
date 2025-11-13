/**
 * 事件處理器封裝
 * 将复杂的事件傳遞简化为可复用的處理器
 */
export function useEventHandlers(actions: any) {
  return {
    // MCP 事件
    onMcpResponse: actions.mcp.handleResponse,
    onMcpCancel: actions.mcp.handleCancel,

    // 主題事件
    onThemeChange: actions.theme.setTheme,

    // 設定事件
    onToggleAlwaysOnTop: actions.settings.toggleAlwaysOnTop,
    onToggleAudioNotification: actions.settings.toggleAudioNotification,
    onUpdateAudioUrl: actions.settings.updateAudioUrl,
    onTestAudio: actions.settings.testAudio,
    onStopAudio: actions.settings.stopAudio,
    onUpdateWindowSize: actions.settings.updateWindowSize,
    onUpdateReplyConfig: actions.settings.updateReplyConfig,
    onMessageReady: actions.settings.setMessageInstance,

    // 音訊事件
    onTestAudioError: actions.audio.handleTestError,

    // 設定事件
    onConfigReloaded: actions.settings.reloadAllSettings,
  }
}
