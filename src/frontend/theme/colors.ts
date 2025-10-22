/**
 * 统一颜色管理系統 - UnoCSS + Naive UI 最佳实践
 *
 * 设计原则：
 * 1. 重新定义 UnoCSS 基础颜色，让它们自動适配主題
 * 2. 使用语义化命名，便于理解和維護
 * 3. 通过 CSS 變數實作主題切换
 * 4. 避免硬编码颜色值
 */

// 主色调設定 - 保持不变，用于强调色
export const primaryColors = {
  50: '#f0fdfa',
  100: '#ccfbf1',
  200: '#99f6e4',
  300: '#5eead4',
  400: '#2dd4bf',
  500: '#14b8a6',
  600: '#0d9488',
  700: '#0f766e',
  800: '#115e59',
  900: '#134e4a',
  950: '#042f2e',
}

// 功能色設定
export const functionalColors = {
  success: '#22c55e',
  warning: '#f59e0b',
  error: '#ef4444',
  info: '#3b82f6',
}

// 语义化颜色系統 - 重新定义 UnoCSS 基础颜色
export const semanticColors = {
  // 重新定义 black/white 让它们适配主題
  black: {
    DEFAULT: 'var(--color-surface)',
    50: 'var(--color-surface-50)',
    100: 'var(--color-surface-100)',
    200: 'var(--color-surface-200)',
    300: 'var(--color-surface-300)',
    400: 'var(--color-surface-400)',
    500: 'var(--color-surface-500)',
    600: 'var(--color-surface-600)',
    700: 'var(--color-surface-700)',
    800: 'var(--color-surface-800)',
    900: 'var(--color-surface-900)',
    950: 'var(--color-surface-950)',
  },
  white: {
    DEFAULT: 'var(--color-on-surface)',
  },
  // 重新定义 gray 让它适配主題
  gray: {
    50: 'var(--color-surface-50)',
    100: 'var(--color-surface-100)',
    200: 'var(--color-surface-200)',
    300: 'var(--color-surface-300)',
    400: 'var(--color-surface-400)',
    500: 'var(--color-surface-500)',
    600: 'var(--color-surface-600)',
    700: 'var(--color-surface-700)',
    800: 'var(--color-surface-800)',
    900: 'var(--color-surface-900)',
    950: 'var(--color-surface-950)',
  },
  // 主色调
  primary: primaryColors,
  // 功能色
  ...functionalColors,
}

// 主題颜色定义
export const themeColors = {
  light: {
    // 表面色（背景相关）
    surface: '#ffffff',
    surface50: '#f9fafb',
    surface100: '#f3f4f6',
    surface200: '#e5e7eb',
    surface300: '#d1d5db',
    surface400: '#9ca3af',
    surface500: '#6b7280',
    surface600: '#4b5563',
    surface700: '#374151',
    surface800: '#1f2937',
    surface900: '#111827',
    surface950: '#030712',
    // 文字色
    onSurface: '#111827',
    onSurfaceSecondary: '#6b7280',
    onSurfaceMuted: '#9ca3af',
    onSurfaceDisabled: '#d1d5db',
    // 容器色
    container: '#f9fafb',
    containerSecondary: '#f3f4f6',
    // 边框色
    border: '#e5e7eb',
    divider: '#e5e7eb',
  },
  dark: {
    // 表面色（背景相关）
    surface: '#101014',
    surface50: '#18181c',
    surface100: '#1f1f23',
    surface200: '#27272a',
    surface300: '#374151',
    surface400: '#4b5563',
    surface500: '#6b7280',
    surface600: '#9ca3af',
    surface700: '#d1d5db',
    surface800: '#e5e7eb',
    surface900: '#f3f4f6',
    surface950: '#f9fafb',
    // 文字色
    onSurface: '#e5e7eb',
    onSurfaceSecondary: '#d1d5db',
    onSurfaceMuted: '#9ca3af',
    onSurfaceDisabled: '#6b7280',
    // 容器色
    container: '#18181c',
    containerSecondary: '#1f1f23',
    // 边框色
    border: '#374151',
    divider: '#374151',
  },
}
