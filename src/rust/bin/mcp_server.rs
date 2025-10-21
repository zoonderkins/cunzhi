// MCP 服务器入口点
use cunzhi::{mcp::run_server, utils::auto_init_logger, log_important};
use cunzhi::mcp::handlers::cleanup_old_temp_files;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 自动初始化日志系统
    auto_init_logger()?;

    log_important!(info, "启动 MCP 服务器");

    // 清理旧的临时文件
    cleanup_old_temp_files();

    run_server().await
}
