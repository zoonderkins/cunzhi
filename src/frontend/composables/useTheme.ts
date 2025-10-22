import { invoke } from '@tauri-apps/api/core'
import { computed, ref } from 'vue'
import { applyThemeVariables, getTheme } from '../theme'

export function useTheme() {
  // 不設定預設主題，等待从config.json載入
  const currentTheme = ref('')

  // 计算 Naive UI 主題
  const naiveTheme = computed(() => {
    // 如果主題还未載入，使用預設深色主題
    const theme = currentTheme.value || 'dark'
    return getTheme(theme)
  })

  // 應用主題
  function applyTheme(theme: string) {
    // 應用主題變數和类
    applyThemeVariables(theme)
    currentTheme.value = theme
  }

  // 切换主題
  async function setTheme(theme: string) {
    try {
      // 先儲存到后端
      await invoke('set_theme', { theme })
      // 儲存成功后應用前端主題
      applyTheme(theme)
    }
    catch (error) {
      console.error('儲存主題設定失敗:', error)
      // 儲存失敗时不應用主題，保持一致性
    }
  }

  // 載入主題設定
  async function loadTheme() {
    try {
      const theme = await invoke('get_theme')
      // 确保主題值有效
      const validTheme = (theme === 'light' || theme === 'dark') ? theme : 'dark'

      // 應用后端主題（无论是否与当前主題相同，确保狀態同步）
      applyTheme(validTheme as string)
    }
    catch (error) {
      console.error('載入主題失敗:', error)
      // 載入失敗時使用預設深色主題
      applyTheme('dark')
    }
  }

  // 立即尝试載入主題，避免延迟
  loadTheme().catch(() => {
    // 如果載入失敗，應用預設主題
    applyTheme('dark')
  })

  return {
    currentTheme,
    naiveTheme,
    setTheme,
    loadTheme,
  }
}
