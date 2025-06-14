// MCP 服务器入口点
use cunzhi::mcp::run_server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_server().await
}
