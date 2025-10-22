import { ref } from 'vue'
import { loadLocale } from '../i18n'
import { useFontManager } from './useFontManager'
import { initMcpTools } from './useMcpTools'
import { useSettings } from './useSettings'
import { useVersionCheck } from './useVersionCheck'

/**
 * 應用初始化组合式函數
 */
export function useAppInitialization(mcpHandler: ReturnType<typeof import('./useMcpHandler').useMcpHandler>) {
  const isInitializing = ref(true)
  const { loadFontConfig, loadFontOptions } = useFontManager()
  const settings = useSettings()
  const { autoCheckUpdate } = useVersionCheck()
  const { checkMcpMode, setupMcpEventListener } = mcpHandler

  /**
   * 檢查是否为首次啟動
   */
  function checkFirstRun(): boolean {
    // 檢查localStorage是否有初始化标记
    const hasInitialized = localStorage.getItem('app-initialized')
    return !hasInitialized
  }

  /**
   * 标记應用已初始化
   */
  function markAsInitialized() {
    localStorage.setItem('app-initialized', 'true')
  }

  /**
   * 初始化應用
   */
  async function initializeApp() {
    try {
      // 檢查是否为首次啟動
      const isFirstRun = checkFirstRun()

      // 主題已在useTheme初始化时載入，这里不需要重复載入

      // 載入語言設定和字型設定
      await Promise.all([
        loadLocale(),
        loadFontConfig(),
        loadFontOptions(),
      ])

      // 檢查是否为MCP模式
      const { isMcp, mcpContent } = await checkMcpMode()

      // 无论是否为MCP模式，都載入視窗設定
      await settings.loadWindowSettings()
      await settings.loadWindowConfig()

      // 設定視窗焦點監聽器，用于設定同步
      await settings.setupWindowFocusListener()

      // 在MCP模式下，确保前端狀態与后端視窗狀態同步
      if (isMcp) {
        console.log('MCP模式检测到，同步視窗狀態...')
        try {
          await settings.syncWindowStateFromBackend()
        }
        catch (error) {
          console.warn('MCP模式狀態同步失敗，繼續初始化:', error)
        }
      }

      // 初始化MCP工具設定（在非MCP模式下）
      if (!isMcp) {
        await initMcpTools()
        await setupMcpEventListener()
      }

      // 如果是首次啟動，标记已初始化（主題已在上面載入过）
      if (isFirstRun) {
        console.log('检测到首次啟動，标记應用已初始化')
        markAsInitialized()
      }

      // 自動檢查版本更新并弹窗（非阻塞）
      autoCheckUpdate().catch(() => {
        // 静默處理版本檢查失敗
      })

      // 结束初始化狀態
      isInitializing.value = false

      return { isMcp, mcpContent }
    }
    catch (error) {
      console.error('應用初始化失敗:', error)
      isInitializing.value = false
      throw error
    }
  }

  return {
    isInitializing,
    initializeApp,
  }
}
