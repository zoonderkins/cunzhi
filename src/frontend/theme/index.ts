import type { GlobalTheme } from 'naive-ui'
import { darkTheme, lightTheme } from 'naive-ui'
import { colors, textColors } from './colors'

// 使用统一的颜色配置
export const themeColors = colors

// 自定义暗黑主题
export const customDarkTheme: GlobalTheme = {
  ...darkTheme,
  common: {
    ...darkTheme.common,
    primaryColor: themeColors.primary[500],
    primaryColorHover: themeColors.primary[400],
    primaryColorPressed: themeColors.primary[600],
    primaryColorSuppl: themeColors.primary[400],

    // 背景色 - 使用指定的暗黑模式颜色
    bodyColor: themeColors.dark.primary, // #101014
    popoverColor: themeColors.dark.secondary, // #18181c
    cardColor: themeColors.dark.secondary, // #18181c
    modalColor: themeColors.dark.secondary, // #18181c

    // 边框色
    borderColor: themeColors.gray[700],
    dividerColor: themeColors.gray[700],

    // 文字色 - 使用统一配置
    textColorBase: textColors.dark.primary,
    textColor1: textColors.dark.primary,
    textColor2: textColors.dark.secondary,
    textColor3: textColors.dark.muted,
    textColorDisabled: textColors.dark.disabled,

    // 输入框
    inputColor: themeColors.dark.accent,
    inputColorDisabled: themeColors.gray[800],

    // 悬停色
    hoverColor: 'rgba(255, 255, 255, 0.09)',
    pressedColor: 'rgba(255, 255, 255, 0.13)',

    // 成功、警告、错误色
    successColor: '#22c55e',
    warningColor: '#f59e0b',
    errorColor: '#ef4444',
    infoColor: themeColors.primary[500],
  },
}

// 自定义浅色主题
export const customLightTheme: GlobalTheme = {
  ...lightTheme,
  common: {
    ...lightTheme.common,
    primaryColor: themeColors.primary[500],
    primaryColorHover: themeColors.primary[400],
    primaryColorPressed: themeColors.primary[600],
    primaryColorSuppl: themeColors.primary[400],

    // 背景色
    bodyColor: themeColors.light.primary,
    popoverColor: themeColors.light.primary,
    cardColor: themeColors.light.primary,
    modalColor: themeColors.light.primary,

    // 边框色
    borderColor: themeColors.gray[200],
    dividerColor: themeColors.gray[200],

    // 文字色 - 使用统一配置
    textColorBase: textColors.light.primary,
    textColor1: textColors.light.primary,
    textColor2: textColors.light.secondary,
    textColor3: textColors.light.muted,
    textColorDisabled: textColors.light.disabled,

    // 输入框
    inputColor: themeColors.light.primary,
    inputColorDisabled: themeColors.gray[50],

    // 悬停色
    hoverColor: 'rgba(0, 0, 0, 0.09)',
    pressedColor: 'rgba(0, 0, 0, 0.13)',

    // 成功、警告、错误色
    successColor: '#22c55e',
    warningColor: '#f59e0b',
    errorColor: '#ef4444',
    infoColor: themeColors.primary[500],
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

// CSS 变量映射
export function applyThemeVariables(themeName: string) {
  const theme = getTheme(themeName)
  const root = document.documentElement

  // 设置 CSS 变量
  if (theme.common) {
    root.style.setProperty('--primary-color', theme.common.primaryColor)
    root.style.setProperty('--body-color', theme.common.bodyColor)
    root.style.setProperty('--card-color', theme.common.cardColor)
    root.style.setProperty('--text-color', theme.common.textColorBase)
    root.style.setProperty('--text-color-secondary', theme.common.textColor2)
    root.style.setProperty('--text-color-muted', theme.common.textColor3)
    root.style.setProperty('--border-color', theme.common.borderColor)
  }

  // 设置主题类
  root.classList.remove('light', 'dark')
  const effectiveTheme = themeName === 'system'
    ? (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light')
    : themeName
  root.classList.add(effectiveTheme)
}
