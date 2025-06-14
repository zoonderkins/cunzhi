import { invoke } from '@tauri-apps/api/core'
import { computed, ref } from 'vue'
import { applyThemeVariables, getTheme } from '../theme'

export function useTheme() {
  const currentTheme = ref('dark')

  // 计算 Naive UI 主题
  const naiveTheme = computed(() => {
    return getTheme(currentTheme.value)
  })

  // 应用主题
  function applyTheme(theme: string) {
    // 应用主题变量和类
    applyThemeVariables(theme)
    currentTheme.value = theme
  }

  // 切换主题
  async function setTheme(theme: string) {
    try {
      await invoke('set_theme', { theme })
      applyTheme(theme)
    }
    catch (error) {
      console.error('切换主题失败:', error)
    }
  }

  // 加载主题设置
  async function loadTheme() {
    try {
      const theme = await invoke('get_theme')
      applyTheme(theme as string)
    }
    catch (error) {
      // 默认使用深色主题
      applyTheme('dark')
      console.error('加载主题失败:', error)
    }
  }

  // 监听系统主题变化
  function setupSystemThemeListener() {
    if (currentTheme.value === 'system') {
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
      mediaQuery.addEventListener('change', () => {
        if (currentTheme.value === 'system') {
          applyTheme('system')
        }
      })
    }
  }

  return {
    currentTheme,
    naiveTheme,
    setTheme,
    loadTheme,
    setupSystemThemeListener,
  }
}
