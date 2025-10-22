use anyhow::Result;
use rmcp::{
    ErrorData as McpError, ServerHandler, ServiceExt, RoleServer,
    model::*,
    transport::stdio,
    service::RequestContext,
};
use std::collections::HashMap;

use super::tools::{InteractionTool, MemoryTool};
use super::types::{ZhiRequest, JiyiRequest};
use crate::config::load_standalone_config;
use crate::{log_important, log_debug};

#[derive(Clone)]
pub struct ZhiServer {
    enabled_tools: HashMap<String, bool>,
}

impl Default for ZhiServer {
    fn default() -> Self {
        Self::new()
    }
}

impl ZhiServer {
    pub fn new() -> Self {
        // 尝试載入設定，如果失敗则使用預設設定
        let enabled_tools = match load_standalone_config() {
            Ok(config) => config.mcp_config.tools,
            Err(e) => {
                log_important!(warn, "无法載入設定檔案，使用預設工具設定: {}", e);
                crate::config::default_mcp_tools()
            }
        };

        Self { enabled_tools }
    }

    /// 檢查工具是否启用 - 动态讀取最新設定
    fn is_tool_enabled(&self, tool_name: &str) -> bool {
        // 每次都重新讀取設定，确保獲取最新狀態
        match load_standalone_config() {
            Ok(config) => {
                let enabled = config.mcp_config.tools.get(tool_name).copied().unwrap_or(true);
                log_debug!("工具 {} 当前狀態: {}", tool_name, enabled);
                enabled
            }
            Err(e) => {
                log_important!(warn, "讀取設定失敗，使用快取狀態: {}", e);
                // 如果讀取失敗，使用快取的設定
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
                title: Some("寸止 MCP 伺服器".to_string()),
                website_url: Some("https://github.com/zoonderkins/cunzhi".to_string()),
                icons: None,
            },
            instructions: Some("Zhi 智慧程式碼審查工具，支持交互式对话和記憶管理".to_string()),
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
                    "description": "预定义的選項列表（可选）"
                },
                "is_markdown": {
                    "type": "boolean",
                    "description": "消息是否为Markdown格式，預設为true"
                }
            },
            "required": ["message"]
        });

        if let serde_json::Value::Object(schema_map) = zhi_schema {
            tools.push(Tool {
                name: Cow::Borrowed("zhi"),
                title: Some("寸止互動工具".to_string()),
                description: Some(Cow::Borrowed("智能代码审查交互工具，支持预定义選項、自由文本輸入和图片上传")),
                input_schema: Arc::new(schema_map),
                output_schema: None,
                icons: None,
                annotations: None,
            });
        }

        // 記憶管理工具 - 仅在启用时新增
        if self.is_tool_enabled("ji") {
            let ji_schema = serde_json::json!({
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "description": "操作類型：記憶(新增記憶), 回忆(獲取專案訊息)"
                    },
                    "project_path": {
                        "type": "string",
                        "description": "專案路径（必需）"
                    },
                    "content": {
                        "type": "string",
                        "description": "記憶内容（記憶操作时必需）"
                    },
                    "category": {
                        "type": "string",
                        "description": "記憶分类：rule(规范规则), preference(用户偏好), pattern(最佳实践), context(專案上下文)"
                    }
                },
                "required": ["action", "project_path"]
            });

            if let serde_json::Value::Object(schema_map) = ji_schema {
                tools.push(Tool {
                    name: Cow::Borrowed("ji"),
                    title: Some("記憶管理工具".to_string()),
                    description: Some(Cow::Borrowed("全局記憶管理工具，用于存储和管理重要的开发规范、用户偏好和最佳实践")),
                    input_schema: Arc::new(schema_map),
                    output_schema: None,
                    icons: None,
                    annotations: None,
                });
            }
        }

        log_debug!("傳回给客户端的工具列表: {:?}", tools.iter().map(|t| &t.name).collect::<Vec<_>>());

        Ok(ListToolsResult {
            tools,
            next_cursor: None,
        })
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        log_debug!("收到工具呼叫请求: {}", request.name);

        match request.name.as_ref() {
            "zhi" => {
                // 解析请求參數
                let arguments_value = request.arguments
                    .map(serde_json::Value::Object)
                    .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));

                let zhi_request: ZhiRequest = serde_json::from_value(arguments_value)
                    .map_err(|e| McpError::invalid_params(format!("參數解析失敗: {}", e), None))?;

                // 呼叫寸止工具
                InteractionTool::zhi(zhi_request).await
            }
            "ji" => {
                // 檢查記憶管理工具是否启用
                if !self.is_tool_enabled("ji") {
                    return Err(McpError::internal_error(
                        "記憶管理工具已被禁用".to_string(),
                        None
                    ));
                }

                // 解析请求參數
                let arguments_value = request.arguments
                    .map(serde_json::Value::Object)
                    .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));

                let ji_request: JiyiRequest = serde_json::from_value(arguments_value)
                    .map_err(|e| McpError::invalid_params(format!("參數解析失敗: {}", e), None))?;

                // 呼叫記憶工具
                MemoryTool::jiyi(ji_request).await
            }
            _ => {
                Err(McpError::invalid_request(
                    format!("未知的工具: {}", request.name),
                    None
                ))
            }
        }
    }
}



/// 啟動MCP服务器
pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    // 建立并執行服务器
    let service = ZhiServer::new()
        .serve(stdio())
        .await
        .inspect_err(|e| {
            log_important!(error, "啟動服务器失敗: {}", e);
        })?;

    // 等待服务器關閉
    service.waiting().await?;
    Ok(())
}
