// MCP 服务器入口点
use cunzhi::{mcp::run_server, utils::auto_init_logger, log_important};
use cunzhi::mcp::handlers::cleanup_old_temp_files;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 自動初始化日誌系統
    auto_init_logger()?;

    log_important!(info, "啟動 MCP 服务器");

    // 清理旧的临時檔案
    cleanup_old_temp_files();

    run_server().await
}
