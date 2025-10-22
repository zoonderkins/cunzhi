use tauri::{State, Manager};
use crate::config::{AppState, save_config};
use crate::constants::window;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowSizeUpdate {
    pub width: f64,
    pub height: f64,
    pub fixed: bool,
}

#[tauri::command]
pub async fn apply_window_constraints(state: State<'_, AppState>, app: tauri::AppHandle) -> Result<(), String> {
    let (window_config, always_on_top) = {
        let config = state.config.lock().map_err(|e| format!("獲取設定失敗: {}", e))?;
        (config.ui_config.window_config.clone(), config.ui_config.always_on_top)
    };

    if let Some(window) = app.get_webview_window("main") {
        // 設定視窗约束
        if let Err(e) = window.set_min_size(Some(tauri::LogicalSize::new(
            window_config.min_width,
            window_config.min_height,
        ))) {
            return Err(format!("設定最小視窗大小失敗: {}", e));
        }

        if let Err(e) = window.set_max_size(Some(tauri::LogicalSize::new(
            window_config.max_width,
            window_config.max_height,
        ))) {
            return Err(format!("設定最大視窗大小失敗: {}", e));
        }

        // 如果启用了自動调整大小，設定为合适的初始大小
        if window_config.auto_resize {
            let initial_width = window_config.min_width;
            let initial_height = (window_config.min_height + window_config.max_height) / 2.0;
            
            if let Err(e) = window.set_size(tauri::LogicalSize::new(initial_width, initial_height)) {
                return Err(format!("設定視窗大小失敗: {}", e));
            }
        }

        // 确保置顶狀態在應用視窗约束后仍然有效
        if let Err(e) = window.set_always_on_top(always_on_top) {
            log::warn!("應用視窗约束后重新設定置顶狀態失敗: {}", e);
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn update_window_size(size_update: WindowSizeUpdate, state: State<'_, AppState>, app: tauri::AppHandle) -> Result<(), String> {
    // 更新設定
    {
        let mut config = state.config.lock().map_err(|e| format!("獲取設定失敗: {}", e))?;

        // 更新模式設定
        config.ui_config.window_config.fixed = size_update.fixed;

        // 更新目前模式的尺寸
        config.ui_config.window_config.update_current_size(size_update.width, size_update.height);

        if size_update.fixed {
            // 固定模式：設定最大和最小尺寸为相同值
            config.ui_config.window_config.max_width = size_update.width;
            config.ui_config.window_config.max_height = size_update.height;
            config.ui_config.window_config.min_width = size_update.width;
            config.ui_config.window_config.min_height = size_update.height;
            config.ui_config.window_config.auto_resize = false;
        } else {
            // 自由拉伸模式：設定合理的最小值和限制的最大值
            config.ui_config.window_config.min_width = window::MIN_WIDTH;
            config.ui_config.window_config.min_height = window::MIN_HEIGHT;
            config.ui_config.window_config.max_width = window::MAX_WIDTH;
            config.ui_config.window_config.max_height = window::MAX_HEIGHT;
            config.ui_config.window_config.auto_resize = window::DEFAULT_AUTO_RESIZE;
        }
    }

    // 儲存設定
    save_config(&state, &app).await.map_err(|e| format!("儲存設定失敗: {}", e))?;

    // 獲取置顶狀態
    let always_on_top = {
        let config = state.config.lock().map_err(|e| format!("獲取設定失敗: {}", e))?;
        config.ui_config.always_on_top
    };

    // 應用到当前視窗
    if let Some(window) = app.get_webview_window("main") {
        if size_update.fixed {
            // 固定模式：設定精确的視窗大小和约束
            if let Err(e) = window.set_size(tauri::LogicalSize::new(size_update.width, size_update.height)) {
                return Err(format!("設定視窗大小失敗: {}", e));
            }

            if let Err(e) = window.set_min_size(Some(tauri::LogicalSize::new(size_update.width, size_update.height))) {
                return Err(format!("設定最小視窗大小失敗: {}", e));
            }

            if let Err(e) = window.set_max_size(Some(tauri::LogicalSize::new(size_update.width, size_update.height))) {
                return Err(format!("設定最大視窗大小失敗: {}", e));
            }

            log::debug!("視窗已設定为固定大小: {}x{}", size_update.width, size_update.height);
        } else {
            // 自由拉伸模式：設定合理的约束范围
            if let Err(e) = window.set_min_size(Some(tauri::LogicalSize::new(window::MIN_WIDTH, window::MIN_HEIGHT))) {
                return Err(format!("設定最小視窗大小失敗: {}", e));
            }

            if let Err(e) = window.set_max_size(Some(tauri::LogicalSize::new(window::MAX_WIDTH, window::MAX_HEIGHT))) {
                return Err(format!("設定最大視窗大小失敗: {}", e));
            }

            // 設定为預設大小
            if let Err(e) = window.set_size(tauri::LogicalSize::new(size_update.width, size_update.height)) {
                return Err(format!("設定視窗大小失敗: {}", e));
            }

            log::debug!("視窗已設定为自由拉伸模式，預設大小: {}x{}", size_update.width, size_update.height);
        }

        // 重新應用置顶狀態，确保視窗大小变更不会影响置頂設定
        if let Err(e) = window.set_always_on_top(always_on_top) {
            log::warn!("重新應用置顶狀態失敗: {}", e);
        } else {
            log::debug!("置顶狀態已重新應用: {}", always_on_top);
        }
    }

    Ok(())
}
