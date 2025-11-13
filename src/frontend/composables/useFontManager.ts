import { invoke } from '@tauri-apps/api/core'
import { computed, ref, watch } from 'vue'

export interface FontInfo {
  font_family: string
  font_size: string
  custom_font_family: string
}

export interface FontFamilyOption {
  id: string
  name: string
  css_value: string
}

export interface FontSizeOption {
  id: string
  name: string
  scale: number
}

/**
 * 字型管理組合式函數
 */
export function useFontManager() {
  // 響應式狀態
  const fontConfig = ref<FontInfo>({
    font_family: 'inter',
    font_size: 'medium',
    custom_font_family: 'Inter, -apple-system, BlinkMacSystemFont, \'Segoe UI\', Roboto, sans-serif',
  })

  const fontFamilyOptions = ref<FontFamilyOption[]>([])
  const fontSizeOptions = ref<FontSizeOption[]>([])

  // 计算當前字型CSS值
  const currentFontFamily = computed(() => {
    if (fontConfig.value.font_family === 'custom') {
      return fontConfig.value.custom_font_family
    }

    const option = fontFamilyOptions.value.find(opt => opt.id === fontConfig.value.font_family)
    return option?.css_value || fontFamilyOptions.value[0]?.css_value || 'Inter, sans-serif'
  })

  // 计算當前字型大小比例
  const currentFontScale = computed(() => {
    const option = fontSizeOptions.value.find(opt => opt.id === fontConfig.value.font_size)
    return option?.scale || 1.0
  })

  /**
   * 應用字型變數到DOM
   */
  function applyFontVariables() {
    const root = document.documentElement

    // 設定字型系列CSS變數
    root.style.setProperty('--font-family', currentFontFamily.value)

    // 設定字型大小比例CSS變數
    root.style.setProperty('--font-size-scale', currentFontScale.value.toString())

    // 應用字型大小到各个尺寸级別
    const baseSizes = [
      { name: 'xs', base: 0.75 },
      { name: 'sm', base: 0.875 },
      { name: 'base', base: 0.875 },
      { name: 'lg', base: 1.0 },
      { name: 'xl', base: 1.125 },
      { name: '2xl', base: 1.25 },
      { name: '3xl', base: 1.5 },
    ]

    baseSizes.forEach(({ name, base }) => {
      const scaledSize = base * currentFontScale.value
      root.style.setProperty(`--font-size-${name}`, `${scaledSize}rem`)
    })

    // 設定body字型
    root.style.setProperty('--body-font-family', currentFontFamily.value)

    // 強制更新body元素的字型
    document.body.style.fontFamily = currentFontFamily.value
  }

  /**
   * 載入字型設定
   */
  async function loadFontConfig() {
    try {
      const config = await invoke<FontInfo>('get_font_config')
      fontConfig.value = config
      applyFontVariables()
    }
    catch (error) {
      console.error('載入字型設定失敗:', error)
    }
  }

  /**
   * 載入字型選項
   */
  async function loadFontOptions() {
    try {
      const [familyOptions, sizeOptions] = await Promise.all([
        invoke<FontFamilyOption[]>('get_font_family_options'),
        invoke<FontSizeOption[]>('get_font_size_options'),
      ])

      fontFamilyOptions.value = familyOptions
      fontSizeOptions.value = sizeOptions
    }
    catch (error) {
      console.error('載入字型選項失敗:', error)
    }
  }

  /**
   * 設定字型系列
   */
  async function setFontFamily(fontFamily: string) {
    try {
      await invoke('set_font_family', { fontFamily })
      fontConfig.value.font_family = fontFamily
      applyFontVariables()
    }
    catch (error) {
      console.error('設定字型系列失敗:', error)
      throw error
    }
  }

  /**
   * 設定字型大小
   */
  async function setFontSize(fontSize: string) {
    try {
      await invoke('set_font_size', { fontSize })
      fontConfig.value.font_size = fontSize
      applyFontVariables()
    }
    catch (error) {
      console.error('設定字型大小失敗:', error)
      throw error
    }
  }

  /**
   * 設定自訂字型系列
   */
  async function setCustomFontFamily(customFontFamily: string) {
    try {
      await invoke('set_custom_font_family', { customFontFamily })
      fontConfig.value.custom_font_family = customFontFamily
      applyFontVariables()
    }
    catch (error) {
      console.error('設定自訂字型系列失敗:', error)
      throw error
    }
  }

  /**
   * 重置字型設定
   */
  async function resetFontConfig() {
    try {
      await invoke('reset_font_config')
      await loadFontConfig()
    }
    catch (error) {
      console.error('重置字型設定失敗:', error)
      throw error
    }
  }

  // 監聽字型設定變化，自動應用
  watch([currentFontFamily, currentFontScale], () => {
    applyFontVariables()
  }, { immediate: true })

  // 監聽字型設定物件變化
  watch(fontConfig, () => {
    applyFontVariables()
  }, { deep: true })

  return {
    // 狀態
    fontConfig,
    fontFamilyOptions,
    fontSizeOptions,
    currentFontFamily,
    currentFontScale,

    // 方法
    loadFontConfig,
    loadFontOptions,
    setFontFamily,
    setFontSize,
    setCustomFontFamily,
    resetFontConfig,
    applyFontVariables,
  }
}
