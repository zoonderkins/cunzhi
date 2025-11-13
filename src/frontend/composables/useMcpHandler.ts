import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { ref } from 'vue'

/**
 * MCP處理組合式函數
 */
export function useMcpHandler() {
  const mcpRequest = ref(null)
  const showMcpPopup = ref(false)

  /**
   * 統一的MCP回應處理
   */
  async function handleMcpResponse(response: any) {
    try {
      // 透過Tauri命令傳送回應并退出應用
      await invoke('send_mcp_response', { response })
      await invoke('exit_app')
    }
    catch (error) {
      console.error('MCP回應處理失敗:', error)
    }
  }

  /**
   * 統一的MCP取消處理
   */
  async function handleMcpCancel() {
    try {
      // 傳送取消訊息并退出應用
      await invoke('send_mcp_response', { response: 'CANCELLED' })
      await invoke('exit_app')
    }
    catch (error) {
      // 静默處理MCP取消錯誤
      console.error('MCP取消處理失敗:', error)
    }
  }

  /**
   * 顯示MCP弹窗
   */
  async function showMcpDialog(request: any) {
    // 獲取Telegram設定，檢查是否需要隐藏前端弹窗
    let shouldShowFrontendPopup = true
    try {
      const telegramConfig = await invoke('get_telegram_config')
      // 如果Telegram啟用且設定了隐藏前端弹窗，则不顯示前端弹窗
      if (telegramConfig && (telegramConfig as any).enabled && (telegramConfig as any).hide_frontend_popup) {
        shouldShowFrontendPopup = false
        console.log('🔕 根据Telegram設定，隐藏前端弹窗')
      }
    }
    catch (error) {
      console.error('獲取Telegram設定失敗:', error)
      // 設定獲取失敗時，保持預設行为（顯示弹窗）
    }

    // 根据設定决定是否顯示前端弹窗
    if (shouldShowFrontendPopup) {
      // 設定請求資料和顯示狀態
      mcpRequest.value = request
      showMcpPopup.value = true
    }
    else {
      console.log('🔕 跳過前端弹窗顯示，僅使用Telegram交互')
    }

    // 播放音訊通知（无论是否顯示弹窗都播放）
    try {
      await invoke('play_notification_sound')
    }
    catch (error) {
      console.error('播放音訊通知失敗:', error)
    }

    // 啟動Telegram同步（无论是否顯示弹窗都啟動）
    try {
      if (request?.message) {
        await invoke('start_telegram_sync', {
          message: request.message,
          predefinedOptions: request.predefined_options || [],
          isMarkdown: request.is_markdown || false,
        })
        console.log('✅ Telegram同步啟動成功')
      }
    }
    catch (error) {
      console.error('啟動Telegram同步失敗:', error)
    }
  }

  /**
   * 檢查MCP模式
   */
  async function checkMcpMode() {
    try {
      const args = await invoke('get_cli_args')

      if (args && (args as any).mcp_request) {
        // 讀取MCP請求檔案
        const content = await invoke('read_mcp_request', { filePath: (args as any).mcp_request })

        if (content) {
          await showMcpDialog(content)
        }
        return { isMcp: true, mcpContent: content }
      }
    }
    catch (error) {
      console.error('檢查MCP模式失敗:', error)
    }
    return { isMcp: false, mcpContent: null }
  }

  /**
   * 設定MCP事件監聽器
   */
  async function setupMcpEventListener() {
    try {
      await listen('mcp-request', (event) => {
        showMcpDialog(event.payload)
      })
    }
    catch (error) {
      console.error('設定MCP事件監聽器失敗:', error)
    }
  }

  return {
    mcpRequest,
    showMcpPopup,
    handleMcpResponse,
    handleMcpCancel,
    showMcpDialog,
    checkMcpMode,
    setupMcpEventListener,
  }
}
