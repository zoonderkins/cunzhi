use cunzhi::config::{AppState, load_config_and_apply_window_settings};
use anyhow::Result;
use tauri::Manager;

// 重新导出所有命令函数
pub use cunzhi::ui::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            get_app_info,
            get_always_on_top,
            set_always_on_top,
            sync_window_state,
            get_audio_notification_enabled,
            set_audio_notification_enabled,
            get_audio_url,
            set_audio_url,
            play_notification_sound,
            test_audio_sound,
            get_theme,
            set_theme,
            get_window_config,
            set_window_config,
            get_reply_config,
            set_reply_config,
            apply_window_constraints,
            send_mcp_response,
            get_cli_args,
            read_mcp_request,
            exit_app
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();

            // 在setup中直接调用，不使用spawn
            tauri::async_runtime::block_on(async {
                let state = app.state::<AppState>();
                if let Err(e) = load_config_and_apply_window_settings(&state, &app_handle).await {
                    eprintln!("加载配置失败: {}", e);
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() -> Result<()> {
    // 检查程序是如何被调用的
    let args: Vec<String> = std::env::args().collect();

    // 处理命令行参数
    if args.len() >= 3 && args[1] == "--mcp-request" {
        // MCP 请求模式：启动 GUI 处理弹窗
        run();
    } else if args.len() >= 2 && (args[1] == "--help" || args[1] == "-h") {
        // 显示帮助信息
        print_help();
    } else if args.len() >= 2 && (args[1] == "--version" || args[1] == "-v") {
        // 显示版本信息
        print_version();
    } else {
        // 正常启动 GUI 应用（设置界面）
        run();
    }

    Ok(())
}

/// 显示帮助信息
fn print_help() {
    println!("寸止 - 智能代码审查工具");
    println!();
    println!("用法:");
    println!("  等一下                    启动设置界面");
    println!("  等一下 --mcp-request <文件>  处理 MCP 请求");
    println!("  等一下 --help             显示此帮助信息");
    println!("  等一下 --version          显示版本信息");
}

/// 显示版本信息
fn print_version() {
    println!("寸止 v{}", env!("CARGO_PKG_VERSION"));
}
