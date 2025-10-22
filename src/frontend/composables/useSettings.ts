import { invoke } from '@tauri-apps/api/core'
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { ref } from 'vue'

// 单例实例
let settingsInstance: ReturnType<typeof createSettings> | null = null

function createSettings() {
  const alwaysOnTop = ref(true) // 与后端預設值保持一致
  const audioNotificationEnabled = ref(true)
  const audioUrl = ref('')
  const windowConfig = ref({
    auto_resize: true,
    max_width: 1500,
    max_height: 1000,
    min_width: 600,
    min_height: 400,
  })
  const windowWidth = ref(600)
  const windowHeight = ref(900)
  const fixedWindowSize = ref(false)

  // 繼續回复設定
  const continueReplyEnabled = ref(true)
  const continuePrompt = ref('请按照最佳实践繼續')

  // 視窗约束和 UI 常量
  const windowConstraints = ref({
    min_width: 600,
    min_height: 400,
    max_width: 1500,
    max_height: 1000,
    resize_step: 50,
    resize_throttle_ms: 1000,
    size_update_delay_ms: 500,
    size_check_delay_ms: 100,
  })

  // Naive UI 消息实例
  let message: any = null

  // 节流定时器
  let resizeThrottleTimer: number | null = null

  // 視窗大小变化監聽器
  let windowResizeUnlisten: (() => void) | null = null

  function setMessageInstance(messageInstance: any) {
    message = messageInstance
  }

  function getMessageInstance() {
    return message
  }

  // 載入視窗约束
  async function loadWindowConstraints() {
    try {
      const constraints = await invoke('get_window_constraints_cmd')
      if (constraints) {
        windowConstraints.value = constraints as any
      }
    }
    catch (error) {
      console.error('載入視窗约束失敗:', error)
    }
  }

  // 載入視窗設定
  async function loadWindowSettings() {
    try {
      // 首先載入視窗约束
      await loadWindowConstraints()

      const enabled = await invoke('get_always_on_top')
      alwaysOnTop.value = enabled as boolean

      // 載入音訊通知設定
      const audioEnabled = await invoke('get_audio_notification_enabled')
      audioNotificationEnabled.value = audioEnabled as boolean

      // 載入音效URL設定
      const audioUrlValue = await invoke('get_audio_url')
      audioUrl.value = audioUrlValue as string

      // 載入視窗尺寸和模式設定
      try {
        const windowSettings = await invoke('get_window_settings')
        if (windowSettings) {
          const settings = windowSettings as any
          windowWidth.value = settings.current_width || 600
          windowHeight.value = settings.current_height || 900
          fixedWindowSize.value = settings.fixed || false
        }
      }
      catch {
        console.log('視窗設定不存在，使用預設值')
      }

      // 載入繼續回复設定
      try {
        const replyConfig = await invoke('get_reply_config')
        if (replyConfig) {
          const config = replyConfig as any
          continueReplyEnabled.value = config.enable_continue_reply || true
          continuePrompt.value = config.continue_prompt || '请按照最佳实践繼續'
        }
      }
      catch {
        console.log('繼續回复設定不存在，使用預設值')
      }

      // 同步視窗狀態
      await invoke('sync_window_state')

      // 根据目前模式設定監聽器
      if (!fixedWindowSize.value) {
        await setupWindowResizeListener()
      }
    }
    catch (error) {
      console.error('載入視窗設定失敗:', error)
    }
  }

  // 切换置頂設定
  async function toggleAlwaysOnTop() {
    try {
      const newValue = !alwaysOnTop.value
      await invoke('set_always_on_top', { enabled: newValue })
      alwaysOnTop.value = newValue
    }
    catch (error) {
      console.error('切换置頂設定失敗:', error)
    }
  }

  // 从后端同步視窗狀態（用于MCP弹窗啟動时确保狀態一致）
  async function syncWindowStateFromBackend() {
    try {
      console.log('同步視窗狀態...')

      // 重新獲取后端的置顶狀態
      const backendAlwaysOnTop = await invoke('get_always_on_top')
      alwaysOnTop.value = backendAlwaysOnTop as boolean

      // 同步視窗狀態到前端
      await invoke('sync_window_state')

      console.log('視窗狀態已从后端同步:', {
        alwaysOnTop: alwaysOnTop.value,
      })
    }
    catch (error) {
      console.error('同步視窗狀態失敗:', error)
      throw error
    }
  }

  // 切换音訊通知設定
  async function toggleAudioNotification() {
    try {
      const newValue = !audioNotificationEnabled.value
      await invoke('set_audio_notification_enabled', { enabled: newValue })
      audioNotificationEnabled.value = newValue
    }
    catch (error) {
      console.error('切换音訊通知設定失敗:', error)
    }
  }

  // 更新音效URL
  async function updateAudioUrl(url: string) {
    try {
      await invoke('set_audio_url', { url })
      audioUrl.value = url
    }
    catch (error) {
      console.error('更新音效URL失敗:', error)
    }
  }

  // 測試音效
  async function testAudioSound() {
    try {
      await invoke('test_audio_sound')
      // 只有在成功时才显示成功提示
      if (message) {
        message.success('音效測試播放成功')
      }
    }
    catch (error) {
      // 显示錯誤提示给用户
      console.error('音效測試失敗:', error)
      if (message) {
        message.error(`音效測試失敗: ${error}`)
      }
    }
  }

  // 停止音效
  async function stopAudioSound() {
    try {
      await invoke('stop_audio_sound')
    }
    catch (error) {
      console.error('停止音效失敗:', error)
    }
  }

  // 更新視窗大小
  async function updateWindowSize(size: { width: number, height: number, fixed: boolean }) {
    try {
      // update_window_size 命令已经会同时更新 WindowConfig 中的所有相关設定
      await invoke('update_window_size', { sizeUpdate: size })

      // 更新本地狀態
      windowWidth.value = size.width
      windowHeight.value = size.height
      fixedWindowSize.value = size.fixed

      // 根据模式設定監聽器
      if (size.fixed) {
        // 固定模式：移除監聽器
        removeWindowResizeListener()
      }
      else {
        // 自由拉伸模式：启用監聽器
        await setupWindowResizeListener()
      }

      if (message) {
        const mode = size.fixed ? '固定大小' : '自由拉伸'
        message.success(`視窗設定已更新：${mode} ${size.width}x${size.height}`)
      }
    }
    catch (error) {
      console.error('更新視窗大小失敗:', error)
      if (message) {
        message.error(`更新視窗大小失敗: ${error}`)
      }
    }
  }

  // 节流儲存視窗尺寸
  function throttledSaveWindowSize() {
    if (resizeThrottleTimer) {
      clearTimeout(resizeThrottleTimer)
    }

    resizeThrottleTimer = window.setTimeout(async () => {
      try {
        // 只在自由拉伸模式下儲存
        if (!fixedWindowSize.value) {
          // 獲取当前視窗的逻辑尺寸
          const result = await invoke('get_current_window_size')
          if (result && typeof result === 'object') {
            const { width, height } = result as any

            // 驗證尺寸，如果小于最小限制则调整为最小尺寸
            let adjustedWidth = width
            let adjustedHeight = height
            let wasAdjusted = false

            if (width < windowConstraints.value.min_width) {
              adjustedWidth = windowConstraints.value.min_width
              wasAdjusted = true
            }
            if (height < windowConstraints.value.min_height) {
              adjustedHeight = windowConstraints.value.min_height
              wasAdjusted = true
            }

            if (wasAdjusted) {
              console.log(`視窗尺寸已调整: ${width}x${height} -> ${adjustedWidth}x${adjustedHeight}`)
            }

            await invoke('set_window_settings', {
              windowSettings: {
                free_width: adjustedWidth,
                free_height: adjustedHeight,
                fixed: false,
              },
            })

            // 更新本地狀態
            windowWidth.value = adjustedWidth
            windowHeight.value = adjustedHeight

            console.log(`視窗尺寸已儲存: ${adjustedWidth}x${adjustedHeight}`)
          }
        }
      }
      catch (error) {
        // 如果是視窗最小化或尺寸过小的錯誤，静默處理
        if (error && typeof error === 'string'
          && (error.includes('視窗已最小化') || error.includes('視窗尺寸过小'))) {
          console.debug('跳過視窗尺寸儲存:', error)
        }
        else {
          console.error('儲存視窗尺寸失敗:', error)
        }
      }
    }, windowConstraints.value.resize_throttle_ms) // 使用設定的节流时间
  }

  // 設定視窗大小变化監聽器
  async function setupWindowResizeListener() {
    try {
      const webview = getCurrentWebviewWindow()

      // 移除之前的監聽器
      if (windowResizeUnlisten) {
        windowResizeUnlisten()
      }

      // 監聽視窗大小变化
      windowResizeUnlisten = await webview.onResized(() => {
        throttledSaveWindowSize()
      })

      console.log('視窗大小变化監聽器已設定')
    }
    catch (error) {
      console.error('設定視窗大小变化監聽器失敗:', error)
    }
  }

  // 移除視窗大小变化監聽器
  function removeWindowResizeListener() {
    if (windowResizeUnlisten) {
      windowResizeUnlisten()
      windowResizeUnlisten = null
    }

    if (resizeThrottleTimer) {
      clearTimeout(resizeThrottleTimer)
      resizeThrottleTimer = null
    }
  }

  // 載入視窗設定
  async function loadWindowConfig() {
    try {
      const config = await invoke('get_window_config')
      windowConfig.value = config as any
    }
    catch (error) {
      console.error('載入視窗設定失敗:', error)
    }
  }

  // 更新繼續回复設定
  async function updateReplyConfig(config: { enable_continue_reply?: boolean, continue_prompt?: string }) {
    try {
      await invoke('set_reply_config', { config })

      if (config.enable_continue_reply !== undefined) {
        continueReplyEnabled.value = config.enable_continue_reply
      }
      if (config.continue_prompt !== undefined) {
        continuePrompt.value = config.continue_prompt
      }

      if (message) {
        message.success('繼續回复設定已更新')
      }
    }
    catch (error) {
      console.error('更新繼續回复設定失敗:', error)
      if (message) {
        message.error('更新繼續回复設定失敗')
      }
    }
  }

  // 重新載入所有設定
  async function reloadAllSettings() {
    try {
      // 首先重新載入后端設定檔案到記憶體
      await invoke('reload_config')

      // 然后重新載入前端設定
      await loadWindowSettings()
      await loadWindowConfig()
    }
    catch (error) {
      console.error('重新載入設定失敗:', error)
    }
  }

  // 視窗焦點監聽器
  let windowFocusUnlisten: (() => void) | null = null

  // 設定視窗焦點監聽
  async function setupWindowFocusListener() {
    try {
      const webview = getCurrentWebviewWindow()

      // 監聽視窗获得焦點事件
      windowFocusUnlisten = await webview.onFocusChanged(({ payload: focused }) => {
        if (focused) {
          reloadAllSettings()
        }
      })
    }
    catch (error) {
      console.error('設定視窗焦點監聽器失敗:', error)
    }
  }

  // 移除視窗焦點監聽器
  function removeWindowFocusListener() {
    if (windowFocusUnlisten) {
      windowFocusUnlisten()
      windowFocusUnlisten = null
    }
  }

  return {
    // 狀態
    alwaysOnTop,
    audioNotificationEnabled,
    audioUrl,
    windowConfig,
    windowWidth,
    windowHeight,
    fixedWindowSize,
    windowConstraints,
    continueReplyEnabled,
    continuePrompt,

    // 方法
    setMessageInstance,
    getMessageInstance,
    loadWindowSettings,
    loadWindowConstraints,
    toggleAlwaysOnTop,
    syncWindowStateFromBackend,
    toggleAudioNotification,
    updateAudioUrl,
    testAudioSound,
    stopAudioSound,
    updateWindowSize,
    updateReplyConfig,
    loadWindowConfig,
    setupWindowResizeListener,
    removeWindowResizeListener,
    reloadAllSettings,
    setupWindowFocusListener,
    removeWindowFocusListener,
  }
}

export function useSettings() {
  if (!settingsInstance) {
    settingsInstance = createSettings()
  }
  return settingsInstance
}
