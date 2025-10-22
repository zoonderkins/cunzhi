use crate::app::builder::run_tauri_app;
use anyhow::Result;

/// 處理命令列參數
pub fn handle_cli_args() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        // 無參數：正常啟動GUI
        1 => {
            run_tauri_app();
        }
        // 單參數：幫助或版本
        2 => {
            match args[1].as_str() {
                "--help" | "-h" => print_help(),
                "--version" | "-v" => print_version(),
                _ => {
                    eprintln!("未知參數: {}", args[1]);
                    print_help();
                    std::process::exit(1);
                }
            }
        }
        // 多參數：MCP請求模式
        _ => {
            if args[1] == "--mcp-request" && args.len() >= 3 {
                handle_mcp_request(&args[2])?;
            } else {
                eprintln!("無效的命令列參數");
                print_help();
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

/// 處理MCP請求
fn handle_mcp_request(_request_file: &str) -> Result<()> {
    // Telegram 功能已移除，直接啟動 GUI 處理 MCP 請求
    run_tauri_app();
    Ok(())
}

/// 顯示幫助訊息
fn print_help() {
    println!("寸止 - 智慧程式碼審查工具");
    println!();
    println!("用法:");
    println!("  等一下                    啟動設定介面");
    println!("  等一下 --mcp-request <檔案>  處理 MCP 請求");
    println!("  等一下 --help             顯示此幫助訊息");
    println!("  等一下 --version          顯示版本訊息");
}

/// 顯示版本訊息
fn print_version() {
    println!("寸止 v{}", env!("CARGO_PKG_VERSION"));
}
