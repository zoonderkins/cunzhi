use crate::config::AppState;
use crate::log_important;
use tauri::{AppHandle, Manager};

/// 處理應用退出請求（从前端快捷键呼叫）
pub async fn handle_exit_request_internal(app_handle: AppHandle) -> Result<bool, String> {
    let state = app_handle.state::<AppState>();
    
    log_important!(info, "🔥 處理應用内退出請求");
    
    crate::ui::exit::handle_system_exit_request(
        state,
        &app_handle,
        false, // 非手動關閉
    ).await
}

/// 設定應用退出處理器（保留向后相容性）
pub fn setup_exit_handlers(_app_handle: &AppHandle) -> Result<(), String> {
    log_important!(info, "✅ 應用退出處理器已設定（前端快捷键處理）");
    Ok(())
}

/// 清理退出處理器（空操作，保持介面一致性）
pub fn cleanup_exit_handlers(_app_handle: &AppHandle) {
    log_important!(info, "應用退出處理器无需清理");
} 
