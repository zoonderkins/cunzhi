use crate::config::{AppState, load_config_and_apply_window_settings};
use crate::ui::{initialize_audio_asset_manager, setup_window_event_listeners};
use crate::ui::exit_handler::setup_exit_handlers;
use crate::log_important;
use tauri::{AppHandle, Manager};

/// 應用設定和初始化
pub async fn setup_application(app_handle: &AppHandle) -> Result<(), String> {
    let state = app_handle.state::<AppState>();

    // 載入設定並應用視窗設定
    if let Err(e) = load_config_and_apply_window_settings(&state, app_handle).await {
        log_important!(warn, "載入設定失敗: {}", e);
    }

    // 初始化音訊資源管理器
    if let Err(e) = initialize_audio_asset_manager(app_handle) {
        log_important!(warn, "初始化音訊資源管理器失敗: {}", e);
    }

    // 設定視窗事件監聽器
    setup_window_event_listeners(app_handle);

    // 設定退出處理器
    if let Err(e) = setup_exit_handlers(app_handle) {
        log_important!(warn, "設定退出處理器失敗: {}", e);
    }

    // 自動開啟 devtools 以便偵錯（暫時在 release 模式下也啟用）
    if let Some(window) = app_handle.get_webview_window("main") {
        log_important!(info, "✅ 找到主視窗，準備開啟 DevTools");

        // 獲取 webview URL 以便偵錯
        if let Ok(url) = window.url() {
            log_important!(info, "📍 Webview URL: {}", url);
        }

        // 開啟 DevTools
        window.open_devtools();
        log_important!(info, "🔧 DevTools 已開啟");
    } else {
        log_important!(warn, "❌ 找不到主視窗！");
    }

    Ok(())
}
