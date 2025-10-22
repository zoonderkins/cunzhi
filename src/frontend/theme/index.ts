import type { GlobalTheme } from 'naive-ui'
import { darkTheme, lightTheme } from 'naive-ui'
import { functionalColors, primaryColors, themeColors } from './colors'

// 自訂暗黑主題
export const customDarkTheme: GlobalTheme = {
  ...darkTheme,
  common: {
    ...darkTheme.common,
    primaryColor: primaryColors[500],
    primaryColorHover: primaryColors[400],
    primaryColorPressed: primaryColors[600],
    primaryColorSuppl: primaryColors[400],

    // 背景色 - 使用新的主題颜色系統
    bodyColor: themeColors.dark.surface,
    popoverColor: themeColors.dark.container,
    cardColor: themeColors.dark.container,
    modalColor: themeColors.dark.container,

    // 边框色
    borderColor: themeColors.dark.border,
    dividerColor: themeColors.dark.divider,

    // 文字色 - 使用新的颜色系統
    textColorBase: themeColors.dark.onSurface,
    textColor1: themeColors.dark.onSurface,
    textColor2: themeColors.dark.onSurfaceSecondary,
    textColor3: themeColors.dark.onSurfaceMuted,
    textColorDisabled: themeColors.dark.onSurfaceDisabled,

    // 輸入框
    inputColor: themeColors.dark.containerSecondary,
    inputColorDisabled: themeColors.dark.surface300,

    // 悬停色
    hoverColor: 'rgba(255, 255, 255, 0.09)',
    pressedColor: 'rgba(255, 255, 255, 0.13)',

    // 成功、警告、錯誤色
    successColor: functionalColors.success,
    warningColor: functionalColors.warning,
    errorColor: functionalColors.error,
    infoColor: functionalColors.info,
  },
}

// 自訂浅色主題
export const customLightTheme: GlobalTheme = {
  ...lightTheme,
  common: {
    ...lightTheme.common,
    primaryColor: primaryColors[500],
    primaryColorHover: primaryColors[400],
    primaryColorPressed: primaryColors[600],
    primaryColorSuppl: primaryColors[400],

    // 背景色 - 使用新的主題颜色系統
    bodyColor: themeColors.light.surface,
    popoverColor: themeColors.light.container,
    cardColor: themeColors.light.container,
    modalColor: themeColors.light.container,

    // 边框色
    borderColor: themeColors.light.border,
    dividerColor: themeColors.light.divider,

    // 文字色 - 使用新的颜色系統
    textColorBase: themeColors.light.onSurface,
    textColor1: themeColors.light.onSurface,
    textColor2: themeColors.light.onSurfaceSecondary,
    textColor3: themeColors.light.onSurfaceMuted,
    textColorDisabled: themeColors.light.onSurfaceDisabled,

    // 輸入框
    inputColor: themeColors.light.containerSecondary,
    inputColorDisabled: themeColors.light.surface100,

    // 悬停色
    hoverColor: 'rgba(0, 0, 0, 0.09)',
    pressedColor: 'rgba(0, 0, 0, 0.13)',

    // 成功、警告、錯誤色
    successColor: functionalColors.success,
    warningColor: functionalColors.warning,
    errorColor: functionalColors.error,
    infoColor: functionalColors.info,
  },
}

// 主題工具函數
export function getTheme(themeName: string): GlobalTheme {
  return themeName === 'dark' ? customDarkTheme : customLightTheme
}

// CSS 變數映射 - 新的颜色系統
export function applyThemeVariables(themeName: string) {
  const root = document.documentElement

  // 确定实际主題（移除system模式，預設dark）
  const effectiveTheme = themeName === 'light' ? 'light' : 'dark'

  const colors = themeColors[effectiveTheme as keyof typeof themeColors]

  // 确保颜色設定存在
  if (!colors) {
    console.warn(`未找到主題颜色設定: ${effectiveTheme}`)
    return
  }

  // 設定语义化 CSS 變數 - 用于 UnoCSS
  root.style.setProperty('--color-surface', colors.surface)
  root.style.setProperty('--color-surface-50', colors.surface50)
  root.style.setProperty('--color-surface-100', colors.surface100)
  root.style.setProperty('--color-surface-200', colors.surface200)
  root.style.setProperty('--color-surface-300', colors.surface300)
  root.style.setProperty('--color-surface-400', colors.surface400)
  root.style.setProperty('--color-surface-500', colors.surface500)
  root.style.setProperty('--color-surface-600', colors.surface600)
  root.style.setProperty('--color-surface-700', colors.surface700)
  root.style.setProperty('--color-surface-800', colors.surface800)
  root.style.setProperty('--color-surface-900', colors.surface900)
  root.style.setProperty('--color-surface-950', colors.surface950)
  root.style.setProperty('--color-on-surface', colors.onSurface)

  // 設定其他语义化變數
  root.style.setProperty('--color-on-surface-secondary', colors.onSurfaceSecondary)
  root.style.setProperty('--color-on-surface-muted', colors.onSurfaceMuted)
  root.style.setProperty('--color-container', colors.container)
  root.style.setProperty('--color-border', colors.border)
  root.style.setProperty('--color-divider', colors.divider)

  // 設定body和text颜色變數（兼容旧CSS）
  root.style.setProperty('--body-color', colors.surface)
  root.style.setProperty('--text-color', colors.onSurface)

  // 強制設定主題类 - 确保根节点类名正确
  root.classList.remove('light', 'dark')
  root.classList.add(effectiveTheme)
  root.setAttribute('data-theme', effectiveTheme)
}
