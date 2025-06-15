import type { GlobalTheme } from 'naive-ui'
import { darkTheme, lightTheme } from 'naive-ui'
import { functionalColors, primaryColors, themeColors } from './colors'

// 自定义暗黑主题
export const customDarkTheme: GlobalTheme = {
  ...darkTheme,
  common: {
    ...darkTheme.common,
    primaryColor: primaryColors[500],
    primaryColorHover: primaryColors[400],
    primaryColorPressed: primaryColors[600],
    primaryColorSuppl: primaryColors[400],

    // 背景色 - 使用新的主题颜色系统
    bodyColor: themeColors.dark.surface,
    popoverColor: themeColors.dark.container,
    cardColor: themeColors.dark.container,
    modalColor: themeColors.dark.container,

    // 边框色
    borderColor: themeColors.dark.border,
    dividerColor: themeColors.dark.divider,

    // 文字色 - 使用新的颜色系统
    textColorBase: themeColors.dark.onSurface,
    textColor1: themeColors.dark.onSurface,
    textColor2: themeColors.dark.onSurfaceSecondary,
    textColor3: themeColors.dark.onSurfaceMuted,
    textColorDisabled: themeColors.dark.onSurfaceDisabled,

    // 输入框
    inputColor: themeColors.dark.containerSecondary,
    inputColorDisabled: themeColors.dark.surface300,

    // 悬停色
    hoverColor: 'rgba(255, 255, 255, 0.09)',
    pressedColor: 'rgba(255, 255, 255, 0.13)',

    // 成功、警告、错误色
    successColor: functionalColors.success,
    warningColor: functionalColors.warning,
    errorColor: functionalColors.error,
    infoColor: functionalColors.info,
  },
}

// 自定义浅色主题
export const customLightTheme: GlobalTheme = {
  ...lightTheme,
  common: {
    ...lightTheme.common,
    primaryColor: primaryColors[500],
    primaryColorHover: primaryColors[400],
    primaryColorPressed: primaryColors[600],
    primaryColorSuppl: primaryColors[400],

    // 背景色 - 使用新的主题颜色系统
    bodyColor: themeColors.light.surface,
    popoverColor: themeColors.light.container,
    cardColor: themeColors.light.container,
    modalColor: themeColors.light.container,

    // 边框色
    borderColor: themeColors.light.border,
    dividerColor: themeColors.light.divider,

    // 文字色 - 使用新的颜色系统
    textColorBase: themeColors.light.onSurface,
    textColor1: themeColors.light.onSurface,
    textColor2: themeColors.light.onSurfaceSecondary,
    textColor3: themeColors.light.onSurfaceMuted,
    textColorDisabled: themeColors.light.onSurfaceDisabled,

    // 输入框
    inputColor: themeColors.light.containerSecondary,
    inputColorDisabled: themeColors.light.surface100,

    // 悬停色
    hoverColor: 'rgba(0, 0, 0, 0.09)',
    pressedColor: 'rgba(0, 0, 0, 0.13)',

    // 成功、警告、错误色
    successColor: functionalColors.success,
    warningColor: functionalColors.warning,
    errorColor: functionalColors.error,
    infoColor: functionalColors.info,
  },
}

// 主题工具函数
export function getTheme(themeName: string): GlobalTheme {
  if (themeName === 'system') {
    const isDark = window.matchMedia('(prefers-color-scheme: dark)').matches
    return isDark ? customDarkTheme : customLightTheme
  }
  return themeName === 'dark' ? customDarkTheme : customLightTheme
}

// CSS 变量映射 - 新的颜色系统
export function applyThemeVariables(themeName: string) {
  const root = document.documentElement

  // 确定实际主题
  const effectiveTheme = themeName === 'system'
    ? (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light')
    : themeName

  const colors = themeColors[effectiveTheme as keyof typeof themeColors]

  // 设置语义化 CSS 变量 - 用于 UnoCSS
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

  // 设置body和text颜色变量（兼容旧CSS）
  root.style.setProperty('--body-color', colors.surface)
  root.style.setProperty('--text-color', colors.onSurface)

  // 设置主题类
  root.classList.remove('light', 'dark')
  root.classList.add(effectiveTheme)
}
