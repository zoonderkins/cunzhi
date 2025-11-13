import { listen } from '@tauri-apps/api/event'

/**
 * 退出警告監聽器管理
 */
class ExitWarningManager {
  private unlistenExitWarning: (() => void) | null = null

  /**
   * 設定退出警告監聽器
   */
  async setupListener(messageInstance: any): Promise<void> {
    // 先移除已存在的監聽器
    this.removeListener()

    if (typeof window !== 'undefined') {
      try {
        this.unlistenExitWarning = await listen('exit-warning', (event: any) => {
          const message = event.payload as string

          // 顯示退出警告消息
          if (messageInstance) {
            messageInstance.warning(message, {
              duration: 3000,
              closable: true,
            })
          }
          else {
            console.warn('Message实例未准备好')
          }
        })
      }
      catch (error) {
        console.error('設定退出警告監聽器失敗:', error)
      }
    }
  }

  /**
   * 移除退出警告監聽器
   */
  removeListener(): void {
    if (this.unlistenExitWarning) {
      this.unlistenExitWarning()
      this.unlistenExitWarning = null
    }
  }
}

// 全局單例实例
const exitWarningManager = new ExitWarningManager()

/**
 * 設定退出警告監聽器（独立函數，不依赖Vue）
 */
export async function setupExitWarningListener(messageInstance: any): Promise<void> {
  await exitWarningManager.setupListener(messageInstance)
}

/**
 * 移除退出警告監聽器（独立函數，不依赖Vue）
 */
export function removeExitWarningListener(): void {
  exitWarningManager.removeListener()
}

/**
 * 退出警告處理組合式函數（保留向后相容性）
 * @deprecated 推荐直接使用 setupExitWarningListener 和 removeExitWarningListener
 */
export function useExitWarning() {
  return {
    setupExitWarningListener,
    removeExitWarningListener,
  }
}
