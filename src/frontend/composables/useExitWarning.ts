/**
 * 退出警告监听器管理
 */
class ExitWarningManager {
  private unlistenExitWarning: (() => void) | null = null

  /**
   * 设置退出警告监听器
   */
  async setupListener(messageInstance: any): Promise<void> {
    // 先移除已存在的监听器
    this.removeListener()

    if (typeof window !== 'undefined' && (window as any).__TAURI__) {
      try {
        const { listen } = (window as any).__TAURI__.event

        this.unlistenExitWarning = await listen('exit-warning', (event: any) => {
          const message = event.payload as string
          console.log('收到退出警告:', message)

          // 显示退出警告消息
          if (messageInstance) {
            console.log('显示退出警告消息:', message)
            messageInstance.warning(message, {
              duration: 3000,
              closable: true,
            })
          }
          else {
            console.warn('Message实例未准备好')
          }
        })

        console.log('退出警告监听器已设置')
      }
      catch (error) {
        console.error('设置退出警告监听器失败:', error)
      }
    }
    else {
      console.warn('Tauri环境不可用，无法设置退出警告监听器')
    }
  }

  /**
   * 移除退出警告监听器
   */
  removeListener(): void {
    if (this.unlistenExitWarning) {
      this.unlistenExitWarning()
      this.unlistenExitWarning = null
      console.log('退出警告监听器已移除')
    }
  }
}

// 全局单例实例
const exitWarningManager = new ExitWarningManager()

/**
 * 设置退出警告监听器（独立函数，不依赖Vue）
 */
export async function setupExitWarningListener(messageInstance: any): Promise<void> {
  await exitWarningManager.setupListener(messageInstance)
}

/**
 * 移除退出警告监听器（独立函数，不依赖Vue）
 */
export function removeExitWarningListener(): void {
  exitWarningManager.removeListener()
}

/**
 * 退出警告处理组合式函数（保留向后兼容性）
 * @deprecated 推荐直接使用 setupExitWarningListener 和 removeExitWarningListener
 */
export function useExitWarning() {
  return {
    setupExitWarningListener,
    removeExitWarningListener,
  }
}
