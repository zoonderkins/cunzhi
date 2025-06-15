use anyhow::Result;
use rmcp::{
    Error as McpError, ServerHandler, ServiceExt, RoleServer,
    model::*,
    transport::stdio,
    service::RequestContext,
    tool,
};
use std::collections::HashMap;

use super::tools::{InteractionTool, MemoryTool};
use super::types::{ZhiRequest, JiyiRequest};
use crate::config::{load_standalone_config, AppConfig};

#[derive(Clone)]
pub struct ZhiServer {
    enabled_tools: HashMap<String, bool>,
}

impl ZhiServer {
    pub fn new() -> Self {
        // 尝试加载配置，如果失败则使用默认配置
        let enabled_tools = match load_standalone_config() {
            Ok(config) => config.mcp_config.tools,
            Err(_) => {
                eprintln!("⚠️ 无法加载配置文件，使用默认工具配置");
                crate::config::default_mcp_tools()
            }
        };

        eprintln!("🔧 MCP工具配置: {:?}", enabled_tools);

        Self { enabled_tools }
    }

    /// 检查工具是否启用
    fn is_tool_enabled(&self, tool_name: &str) -> bool {
        self.enabled_tools.get(tool_name).copied().unwrap_or(true)
    }
}

#[tool(tool_box)]
impl ServerHandler for ZhiServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "Zhi-mcp".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            instructions: Some("Zhi 智能代码审查工具，支持交互式对话和记忆管理".to_string()),
        }
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<ServerInfo, McpError> {
        Ok(self.get_info())
    }
}

// 将工具方法委托给专门的工具结构体
#[tool(tool_box)]
impl ZhiServer {
    #[tool(description = "zhi 智能代码审查交互工具，支持预定义选项、自由文本输入和图片上传")]
    async fn zhi(
        &self,
        #[tool(aggr)] request: ZhiRequest,
    ) -> Result<CallToolResult, McpError> {
        // 寸止工具始终启用（必需工具）
        InteractionTool::zhi(request).await
    }

    #[tool(description = "ji 全局记忆管理工具，用于存储和管理重要的开发规范、用户偏好和最佳实践")]
    async fn ji(
        &self,
        #[tool(aggr)] request: JiyiRequest,
    ) -> Result<CallToolResult, McpError> {
        // 检查记忆管理工具是否启用
        if !self.is_tool_enabled("ji") {
            return Err(McpError::method_not_found(
                "记忆管理工具已被禁用".to_string(),
                None
            ));
        }

        MemoryTool::jiyi(request).await
    }
}

/// 启动MCP服务器
pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    // 创建并运行服务器
    let service = ZhiServer::new()
        .serve(stdio())
        .await
        .inspect_err(|e| {
            eprintln!("启动服务器失败: {}", e);
        })?;

    // 等待服务器关闭
    service.waiting().await?;
    Ok(())
}
