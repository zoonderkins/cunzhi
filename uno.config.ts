import {
  defineConfig,
  presetAttributify,
  presetIcons,
  presetTypography,
  presetWebFonts,
  presetWind3,
} from 'unocss'
import { colors, themeVariables } from './src/frontend/theme/colors'

export default defineConfig({
  presets: [
    presetTypography(),
    presetWind3(),
    presetAttributify(),
    presetIcons({
      collections: {
        tabler: () => import('@iconify-json/tabler/icons.json').then(i => i.default),
        carbon: () => import('@iconify-json/carbon/icons.json').then(i => i.default),
        mdi: () => import('@iconify-json/mdi/icons.json').then(i => i.default),
        heroicons: () => import('@iconify-json/heroicons/icons.json').then(i => i.default),
      },
      extraProperties: {
        'display': 'inline-block',
        'vertical-align': 'middle',
      },
    }),
    presetWebFonts({
      fonts: {
        sans: 'Inter:400,500,600,700',
        mono: 'JetBrains Mono:400,500,600',
      },
    }),
  ],
  theme: {
    colors: {
      ...colors,
      ...themeVariables,
    },
    fontSize: {
      'xs': '0.75rem',
      'sm': '0.875rem',
      'base': '0.875rem',
      'lg': '1rem',
      'xl': '1.125rem',
      '2xl': '1.25rem',
    },
    fontFamily: {
      sans: ['-apple-system', 'BlinkMacSystemFont', 'Segoe UI', 'Roboto', 'Helvetica Neue', 'Arial', 'sans-serif'],
      mono: ['SF Mono', 'Monaco', 'Cascadia Code', 'Roboto Mono', 'Consolas', 'Courier New', 'monospace'],
    },
    spacing: {
      18: '4.5rem',
      88: '22rem',
    },
  },
  shortcuts: [
  ],
  rules: [
    // 自定义规则
    [/^animate-in$/, () => ({ animation: 'fadeIn 0.2s ease-in-out' })],
    [/^fade-in$/, () => ({ opacity: '1' })],
  ],
  safelist: [
    // 确保动态类名不被清除
    'bg-blue-100',
    'bg-purple-100',
    'bg-green-100',
    'bg-orange-100',
    'bg-cyan-100',
    'bg-blue-900',
    'bg-purple-900',
    'bg-green-900',
    'bg-orange-900',
    'bg-cyan-900',
    'text-primary-700',
    'border-primary-200',
    'bg-primary-50',
    'bg-primary-500',
    'animate-pulse',
    // 主题变量相关
    'bg-theme-body',
    'bg-theme-card',
    'text-theme-text',
    'text-theme-text-secondary',
    'text-theme-text-muted',
    'border-theme-border',
    // 图标类
    'i-carbon-settings-services',
    'i-carbon-data-base',
    'i-carbon-color-palette',
    'i-carbon-settings',
    'i-carbon-repeat',
    'i-carbon-document',
    'i-carbon-copy',
    'i-carbon-checkmark',
    'i-carbon-checkmark-filled',
    'i-carbon-rule',
    'i-carbon-chat',
    'i-carbon-information',
    'i-carbon-sun',
    'i-carbon-moon',
    'i-carbon-play',
    'i-carbon-close',
    'i-carbon-send',
    'i-carbon-warning',
    'i-carbon-volume-up',
  ],
})
