use crate::config::AppState;
use crate::log_important;
use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState, ShortcutEvent};

/// 全局快捷键事件处理器
pub fn handle_global_shortcut(app: &tauri::AppHandle, shortcut: &Shortcut, event: ShortcutEvent) {
    if event.state() == ShortcutState::Pressed {
        let app_handle = app.clone();
        let shortcut_debug = format!("{:?}", shortcut);

        // 异步处理退出请求
        tauri::async_runtime::spawn(async move {
            let state = app_handle.state::<AppState>();

            log_important!(info, "🔥 全局退出快捷键触发: {}", shortcut_debug);

            match crate::ui::exit::handle_system_exit_request(
                state,
                &app_handle,
                false, // 快捷键触发
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
}

/// 设置全局快捷键监听器
pub fn setup_global_shortcuts(app_handle: &AppHandle) -> Result<(), String> {
    // 跨平台退出快捷键配置
    let exit_shortcuts = get_exit_shortcuts();

    for shortcut_config in exit_shortcuts {
        let description = shortcut_config.description.clone();

        // 注册快捷键
        match app_handle.global_shortcut().register(shortcut_config.shortcut) {
            Ok(_) => {
                log_important!(info, "✅ 全局快捷键已注册: {}", description);
            }
            Err(e) => {
                log_important!(warn, "❌ 注册全局快捷键失败 {}: {}", description, e);
                continue;
            }
        }
    }

    Ok(())
}

/// 快捷键配置结构
struct ShortcutConfig {
    shortcut: Shortcut,
    description: String,
}

/// 获取跨平台的退出快捷键配置
fn get_exit_shortcuts() -> Vec<ShortcutConfig> {
    let mut shortcuts = Vec::new();
    
    // macOS: Cmd+Q
    #[cfg(target_os = "macos")]
    shortcuts.push(ShortcutConfig {
        shortcut: Shortcut::new(Some(Modifiers::META), Code::KeyQ),
        description: "Cmd+Q (macOS)".to_string(),
    });
    
    // Windows: Alt+F4
    #[cfg(target_os = "windows")]
    shortcuts.push(ShortcutConfig {
        shortcut: Shortcut::new(Some(Modifiers::ALT), Code::F4),
        description: "Alt+F4 (Windows)".to_string(),
    });
    
    // Linux: Ctrl+Q
    #[cfg(target_os = "linux")]
    shortcuts.push(ShortcutConfig {
        shortcut: Shortcut::new(Some(Modifiers::CONTROL), Code::KeyQ),
        description: "Ctrl+Q (Linux)".to_string(),
    });
    
    // 通用快捷键：Ctrl+Shift+Q (作为备选)
    shortcuts.push(ShortcutConfig {
        shortcut: Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyQ),
        description: "Ctrl+Shift+Q (通用)".to_string(),
    });
    
    shortcuts
}

/// 清理全局快捷键
pub fn cleanup_global_shortcuts(app_handle: &AppHandle) {
    let global_shortcut = app_handle.global_shortcut();
    
    if let Err(e) = global_shortcut.unregister_all() {
        log_important!(warn, "清理全局快捷键失败: {}", e);
    } else {
        log_important!(info, "全局快捷键已清理");
    }
}
