use crate::config::AppState;
use crate::log_important;
use tauri::{AppHandle, Manager, WindowEvent};

/// 设置窗口事件监听器
pub fn setup_window_event_listeners(app_handle: &AppHandle) {
    if let Some(window) = app_handle.get_webview_window("main") {
        let app_handle_clone = app_handle.clone();
        
        window.on_window_event(move |event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                // 阻止默认的关闭行为
                api.prevent_close();
                
                let app_handle = app_handle_clone.clone();
                
                // 异步处理退出请求
                tauri::async_runtime::spawn(async move {
                    let state = app_handle.state::<AppState>();
                    
                    // 这里假设是系统快捷键触发的关闭（非手动点击）
                    // 在实际应用中，可以通过其他方式区分关闭来源
                    match crate::ui::exit::handle_system_exit_request(
                        state,
                        &app_handle,
                        false, // 假设是快捷键触发
                    ).await {
                        Ok(exited) => {
                            if !exited {
                                log_important!(info, "退出被阻止，等待二次确认");
                            }
                        }
                        Err(e) => {
                            log_important!(error, "处理退出请求失败: {}", e);
                        }
                    }
                });
            }
        });
    }
}
