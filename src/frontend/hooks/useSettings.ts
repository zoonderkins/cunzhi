import { invoke } from '@tauri-apps/api/core'
import { ref } from 'vue'

export function useSettings() {
  const alwaysOnTop = ref(true)
  const audioNotificationEnabled = ref(true)
  const audioUrl = ref('')
  const windowConfig = ref({
    auto_resize: true,
    max_width: 800,
    max_height: 1000,
    min_width: 600,
    min_height: 800,
  })

  // Naive UI 消息实例
  let message: any = null

  function setMessageInstance(messageInstance: any) {
    message = messageInstance
  }

  // 加载窗口设置
  async function loadWindowSettings() {
    try {
      const enabled = await invoke('get_always_on_top')
      alwaysOnTop.value = enabled as boolean

      // 加载音频通知设置
      const audioEnabled = await invoke('get_audio_notification_enabled')
      audioNotificationEnabled.value = audioEnabled as boolean

      // 加载音效URL设置
      const audioUrlValue = await invoke('get_audio_url')
      audioUrl.value = audioUrlValue as string

      // 同步窗口状态
      await invoke('sync_window_state')
    }
    catch (error) {
      console.error('加载窗口设置失败:', error)
    }
  }

  // 切换置顶设置
  async function toggleAlwaysOnTop() {
    try {
      const newValue = !alwaysOnTop.value
      await invoke('set_always_on_top', { enabled: newValue })
      alwaysOnTop.value = newValue
    }
    catch (error) {
      console.error('切换置顶设置失败:', error)
    }
  }

  // 切换音频通知设置
  async function toggleAudioNotification() {
    try {
      const newValue = !audioNotificationEnabled.value
      await invoke('set_audio_notification_enabled', { enabled: newValue })
      audioNotificationEnabled.value = newValue
    }
    catch (error) {
      console.error('切换音频通知设置失败:', error)
    }
  }

  // 更新音效URL
  async function updateAudioUrl(url: string) {
    try {
      await invoke('set_audio_url', { url })
      audioUrl.value = url
    }
    catch (error) {
      console.error('更新音效URL失败:', error)
    }
  }

  // 测试音效
  async function testAudioSound() {
    try {
      await invoke('test_audio_sound')
      // 只有在成功时才显示成功提示
      if (message) {
        message.success('音效测试播放成功')
      }
    }
    catch (error) {
      // 显示错误提示给用户
      console.error('音效测试失败:', error)
      if (message) {
        message.error(`音效测试失败: ${error}`)
      }
    }
  }

  // 停止音效
  async function stopAudioSound() {
    try {
      await invoke('stop_audio_sound')
    }
    catch (error) {
      console.error('停止音效失败:', error)
    }
  }

  // 更新窗口大小
  async function updateWindowSize(size: { width: number; height: number; fixed: boolean }) {
    try {
      await invoke('update_window_size', { sizeUpdate: size })
      if (message) {
        message.success(`窗口大小已更新为 ${size.width}x${size.height}`)
      }
    }
    catch (error) {
      console.error('更新窗口大小失败:', error)
      if (message) {
        message.error(`更新窗口大小失败: ${error}`)
      }
    }
  }

  // 加载窗口配置
  async function loadWindowConfig() {
    try {
      const config = await invoke('get_window_config')
      windowConfig.value = config as any
    }
    catch (error) {
      console.error('加载窗口配置失败:', error)
    }
  }

  return {
    alwaysOnTop,
    audioNotificationEnabled,
    audioUrl,
    windowConfig,
    setMessageInstance,
    loadWindowSettings,
    toggleAlwaysOnTop,
    toggleAudioNotification,
    updateAudioUrl,
    testAudioSound,
    stopAudioSound,
    updateWindowSize,
    loadWindowConfig,
  }
}
