import type { Ref } from 'vue'
import { computed, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import zhCN from './locales/zh-CN'
import zhTW from './locales/zh-TW'

// 語言類型
export type Locale = 'zh-CN' | 'zh-TW'

// 語言包類型
export type Messages = typeof zhCN

// 可用語言
export const locales: Record<Locale, Messages> = {
  'zh-CN': zhCN,
  'zh-TW': zhTW,
}

// 語言名稱映射
export const localeNames: Record<Locale, string> = {
  'zh-CN': '简体中文',
  'zh-TW': '繁體中文',
}

// 當前語言（預設為繁體中文）
const currentLocale = ref<Locale>('zh-TW')

// 當前語言包
const currentMessages = computed(() => locales[currentLocale.value])

/**
 * 獲取翻譯文本
 * @param key 翻譯鍵，支持點號分隔的路徑，如 'settings.theme.title'
 * @param params 參數對象，用於替換文本中的佔位符
 * @returns 翻譯後的文本
 */
function t(key: string, params?: Record<string, string | number>): string {
  const keys = key.split('.')
  let value: any = currentMessages.value

  // 遍歷鍵路徑
  for (const k of keys) {
    if (value && typeof value === 'object' && k in value) {
      value = value[k]
    }
    else {
      console.warn(`Translation key not found: ${key}`)
      return key
    }
  }

  // 如果最終值不是字符串，返回鍵本身
  if (typeof value !== 'string') {
    console.warn(`Translation value is not a string: ${key}`)
    return key
  }

  // 替換參數
  if (params) {
    return value.replace(/\{(\w+)\}/g, (match, paramKey) => {
      return paramKey in params ? String(params[paramKey]) : match
    })
  }

  return value
}

/**
 * 設置當前語言
 * @param locale 語言代碼
 */
async function setLocale(locale: Locale) {
  currentLocale.value = locale

  // 保存到配置
  try {
    const config = await invoke('load_config') as any
    config.ui_config.language = locale
    await invoke('save_config_cmd', { configValue: config })
  }
  catch (error) {
    console.error('Failed to save language setting:', error)
  }
}

/**
 * 從配置加載語言設置
 */
async function loadLocale() {
  try {
    console.log('[i18n] 開始載入語言設定...')
    const config = await invoke('load_config') as any
    const language = config.ui_config?.language || 'zh-TW'
    console.log('[i18n] 載入的語言:', language)
    currentLocale.value = language as Locale
    console.log('[i18n] 語言設定完成:', currentLocale.value)
  }
  catch (error) {
    console.error('[i18n] 載入語言設定失敗:', error)
    currentLocale.value = 'zh-TW'
    console.log('[i18n] 使用預設語言: zh-TW')
  }
}

/**
 * i18n composable
 */
export function useI18n() {
  return {
    t,
    locale: currentLocale as Ref<Locale>,
    setLocale,
    loadLocale,
    locales: Object.keys(locales) as Locale[],
    localeNames,
  }
}

// 導出默認實例
export { t, currentLocale as locale, setLocale, loadLocale }

