use crate::config::AppState;
use crate::constants::app::{EXIT_CONFIRMATION_WINDOW_SECS, REQUIRED_EXIT_ATTEMPTS};
use crate::log_important;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Manager, State, Emitter};

/// 檢查是否应该允许退出
/// 傳回 (should_exit, show_warning)
pub fn should_allow_exit(state: &State<AppState>) -> Result<(bool, bool), String> {
    let now = Instant::now();

    // 獲取當前退出嘗試计数和上次嘗試時间
    let (current_count, last_attempt) = {
        let count_guard = state.exit_attempt_count.lock()
            .map_err(|e| format!("獲取退出计数失敗: {}", e))?;
        let time_guard = state.last_exit_attempt.lock()
            .map_err(|e| format!("獲取退出時间失敗: {}", e))?;

        (*count_guard, *time_guard)
    };

    log_important!(info, "🔍 退出檢查 - 當前计数: {}, 要求计数: {}", current_count, REQUIRED_EXIT_ATTEMPTS);

    // 檢查時间視窗
    let within_time_window = if let Some(last_time) = last_attempt {
        let elapsed = now.duration_since(last_time);
        let within_window = elapsed <= Duration::from_secs(EXIT_CONFIRMATION_WINDOW_SECS);
        log_important!(info, "🔍 時间視窗檢查 - 距离上次: {:?}, 視窗期: {}秒, 在視窗内: {}",
                elapsed, EXIT_CONFIRMATION_WINDOW_SECS, within_window);
        within_window
    } else {
        log_important!(info, "🔍 首次退出嘗試");
        false
    };
    
    // 如果超出時间視窗，重置计数器并開始新的计数
    if !within_time_window {
        reset_exit_attempts(state)?;
        increment_exit_attempts(state, now)?;
        return Ok((false, true)); // 不退出，顯示警告
    }

    // 在時间視窗内，先增加计数，然后檢查是否达到要求
    increment_exit_attempts(state, now)?;
    let new_count = {
        let count_guard = state.exit_attempt_count.lock()
            .map_err(|e| format!("獲取退出计数失敗: {}", e))?;
        *count_guard
    };

    if new_count >= REQUIRED_EXIT_ATTEMPTS {
        // 达到要求的嘗試次数，允许退出
        reset_exit_attempts(state)?;
        Ok((true, false))
    } else {
        // 还未达到要求次数，顯示警告
        Ok((false, true))
    }
}

/// 重置退出嘗試计数器
fn reset_exit_attempts(state: &State<AppState>) -> Result<(), String> {
    {
        let mut count_guard = state.exit_attempt_count.lock()
            .map_err(|e| format!("重置退出计数失敗: {}", e))?;
        *count_guard = 0;
    }
    
    {
        let mut time_guard = state.last_exit_attempt.lock()
            .map_err(|e| format!("重置退出時间失敗: {}", e))?;
        *time_guard = None;
    }
    
    Ok(())
}

/// 增加退出嘗試计数
fn increment_exit_attempts(state: &State<AppState>, now: Instant) -> Result<(), String> {
    {
        let mut count_guard = state.exit_attempt_count.lock()
            .map_err(|e| format!("增加退出计数失敗: {}", e))?;
        *count_guard += 1;
    }
    
    {
        let mut time_guard = state.last_exit_attempt.lock()
            .map_err(|e| format!("更新退出時间失敗: {}", e))?;
        *time_guard = Some(now);
    }
    
    Ok(())
}

/// 處理系統退出請求（来自快捷键或視窗關閉按钮）
pub async fn handle_system_exit_request(
    state: State<'_, AppState>,
    app: &AppHandle,
    is_manual_close: bool,
) -> Result<bool, String> {
    // 如果是手動点击關閉按钮，直接退出
    if is_manual_close {
        perform_exit(app.clone()).await?;
        return Ok(true);
    }
    
    // 檢查是否应该允许退出
    let (should_exit, show_warning) = should_allow_exit(&state)?;
    
    if should_exit {
        perform_exit(app.clone()).await?;
        Ok(true)
    } else if show_warning {
        // 傳送警告消息到前端
        let warning_message = format!(
            "再次按下退出快捷键以確認退出 ({}秒内有效)",
            EXIT_CONFIRMATION_WINDOW_SECS
        );

        if let Some(window) = app.get_webview_window("main") {
            match window.emit("exit-warning", &warning_message) {
                Ok(_) => {
                    log_important!(info, "✅ 退出警告事件已傳送: {}", warning_message);
                }
                Err(e) => {
                    log_important!(error, "❌ 傳送退出警告事件失敗: {}", e);
                }
            }
        } else {
            log_important!(error, "❌ 無法獲取主視窗，無法傳送退出警告");
        }
        Ok(false)
    } else {
        Ok(false)
    }
}

/// 執行實際的退出操作
async fn perform_exit(app: AppHandle) -> Result<(), String> {
    // 關閉所有視窗
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.close();
    }
    
    // 短暂延迟后強制退出應用
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    app.exit(0);
    Ok(())
}

/// Tauri命令：強制退出應用（用于程序内部呼叫）
#[tauri::command]
pub async fn force_exit_app(app: AppHandle) -> Result<(), String> {
    perform_exit(app).await
}

/// Tauri命令：重置退出嘗試计数器
#[tauri::command]
pub async fn reset_exit_attempts_cmd(state: State<'_, AppState>) -> Result<(), String> {
    reset_exit_attempts(&state)
}
