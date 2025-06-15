// 统一的颜色配置 - 供 UnoCSS 和 Naive UI 主题使用
export const colors = {
  primary: {
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
  },
  gray: {
    50: '#f9fafb',
    100: '#f3f4f6',
    200: '#e5e7eb',
    300: '#d1d5db',
    400: '#9ca3af',
    500: '#6b7280',
    600: '#4b5563',
    700: '#374151',
    800: '#1f2937',
    900: '#111827',
    950: '#030712',
  },
  dark: {
    primary: '#101014', // 暗黑模式背景色
    secondary: '#18181c', // 暗黑模式前景色
    accent: '#374151',
  },
  light: {
    primary: '#ffffff',
    secondary: '#f9fafb',
    accent: '#f3f4f6',
  },
}

// 主题变量映射 - 用于 CSS 变量
export const themeVariables = {
  // 使用 CSS 变量
  theme: {
    primary: 'var(--primary-color)',
    body: 'var(--body-color)',
    card: 'var(--card-color)',
    text: 'var(--text-color)',
    'text-secondary': 'var(--text-color-secondary)',
    'text-muted': 'var(--text-color-muted)',
    border: 'var(--border-color)',
  },
}

// 统一的文字颜色配置
export const textColors = {
  light: {
    primary: '#111827',      // 主要文字
    secondary: '#6b7280',    // 次要文字
    muted: '#9ca3af',        // 弱化文字
    disabled: '#d1d5db',     // 禁用文字
  },
  dark: {
    primary: '#e5e7eb',      // 主要文字
    secondary: '#d1d5db',    // 次要文字
    muted: '#9ca3af',        // 弱化文字
    disabled: '#6b7280',     // 禁用文字
  },
}
