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
use crate::config::load_standalone_config;

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

    /// 检查工具是否启用 - 动态读取最新配置
    fn is_tool_enabled(&self, tool_name: &str) -> bool {
        // 每次都重新读取配置，确保获取最新状态
        match load_standalone_config() {
            Ok(config) => {
                let enabled = config.mcp_config.tools.get(tool_name).copied().unwrap_or(true);
                eprintln!("🔧 工具 {} 当前状态: {}", tool_name, enabled);
                enabled
            }
            Err(e) => {
                eprintln!("⚠️ 读取配置失败，使用缓存状态: {}", e);
                // 如果读取失败，使用缓存的配置
                self.enabled_tools.get(tool_name).copied().unwrap_or(true)
            }
        }
    }
}

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

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParam>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        use std::sync::Arc;
        use std::borrow::Cow;

        let mut tools = Vec::new();

        // 寸止工具始终可用（必需工具）
        let zhi_schema = serde_json::json!({
            "type": "object",
            "properties": {
                "message": {
                    "type": "string",
                    "description": "要显示给用户的消息"
                },
                "predefined_options": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "预定义的选项列表（可选）"
                },
                "is_markdown": {
                    "type": "boolean",
                    "description": "消息是否为Markdown格式，默认为true"
                }
            },
            "required": ["message"]
        });

        if let serde_json::Value::Object(schema_map) = zhi_schema {
            tools.push(Tool {
                name: Cow::Borrowed("zhi"),
                description: Some(Cow::Borrowed("智能代码审查交互工具，支持预定义选项、自由文本输入和图片上传")),
                input_schema: Arc::new(schema_map),
                annotations: None,
            });
        }

        // 记忆管理工具 - 仅在启用时添加
        if self.is_tool_enabled("ji") {
            let ji_schema = serde_json::json!({
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "description": "操作类型：记忆(添加记忆), 回忆(获取项目信息)"
                    },
                    "project_path": {
                        "type": "string",
                        "description": "项目路径（必需）"
                    },
                    "content": {
                        "type": "string",
                        "description": "记忆内容（记忆操作时必需）"
                    },
                    "category": {
                        "type": "string",
                        "description": "记忆分类：rule(规范规则), preference(用户偏好), pattern(最佳实践), context(项目上下文)"
                    }
                },
                "required": ["action", "project_path"]
            });

            if let serde_json::Value::Object(schema_map) = ji_schema {
                tools.push(Tool {
                    name: Cow::Borrowed("ji"),
                    description: Some(Cow::Borrowed("全局记忆管理工具，用于存储和管理重要的开发规范、用户偏好和最佳实践")),
                    input_schema: Arc::new(schema_map),
                    annotations: None,
                });
            }
        }

        eprintln!("🔧 返回给客户端的工具列表: {:?}", tools.iter().map(|t| &t.name).collect::<Vec<_>>());

        Ok(ListToolsResult {
            tools,
            next_cursor: None,
        })
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
            return Err(McpError::internal_error(
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
