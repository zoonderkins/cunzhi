import { ref } from 'vue'
import { useMcpHandler } from './useMcpHandler'
import { initMcpTools } from './useMcpTools'
import { useSettings } from './useSettings'
import { useTheme } from './useTheme'
import { useVersionCheck } from './useVersionCheck'

/**
 * 应用初始化组合式函数
 */
export function useAppInitialization() {
  const isInitializing = ref(true)
  const { loadTheme, setupSystemThemeListener } = useTheme()
  const settings = useSettings()
  const { silentCheckUpdate } = useVersionCheck()
  const { checkMcpMode, setupMcpEventListener } = useMcpHandler()

  /**
   * 初始化应用
   */
  async function initializeApp() {
    try {
      // 首先加载主题设置
      await loadTheme()

      // 检查是否为MCP模式
      const { isMcp, mcpContent } = await checkMcpMode()

      // 无论是否为MCP模式，都加载窗口设置
      await settings.loadWindowSettings()
      await settings.loadWindowConfig()

      // 初始化MCP工具配置（在非MCP模式下）
      if (!isMcp) {
        await initMcpTools()
        await setupMcpEventListener()
      }

      // 监听系统主题变化
      setupSystemThemeListener()

      // 静默检查版本更新（非阻塞）
      silentCheckUpdate().catch((error) => {
        console.warn('静默版本检查失败:', error)
      })

      // 结束初始化状态
      isInitializing.value = false

      return { isMcp, mcpContent }
    }
    catch (error) {
      console.error('应用初始化失败:', error)
      isInitializing.value = false
      throw error
    }
  }

  return {
    isInitializing,
    initializeApp,
    checkMcpMode,
    setupMcpEventListener,
  }
}
