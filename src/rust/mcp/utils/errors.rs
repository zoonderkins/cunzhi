/// MCP 錯誤處理工具模組
/// 
/// 提供統一的錯誤處理和转换功能

use rmcp::ErrorData as McpError;

/// MCP 錯誤類型枚举
#[derive(Debug, thiserror::Error)]
pub enum McpToolError {
    #[error("專案路径錯誤: {0}")]
    ProjectPath(String),
    
    #[error("弹窗建立失敗: {0}")]
    PopupCreation(String),
    
    #[error("回應解析失敗: {0}")]
    ResponseParsing(String),
    
    #[error("記憶管理錯誤: {0}")]
    Memory(String),
    
    #[error("IO 錯誤: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON 序列化錯誤: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("通用錯誤: {0}")]
    Generic(#[from] anyhow::Error),
}

impl From<McpToolError> for McpError {
    fn from(error: McpToolError) -> Self {
        match error {
            McpToolError::ProjectPath(msg) => {
                McpError::invalid_params(msg, None)
            }
            McpToolError::PopupCreation(msg) | 
            McpToolError::ResponseParsing(msg) | 
            McpToolError::Memory(msg) => {
                McpError::internal_error(msg, None)
            }
            McpToolError::Io(e) => {
                McpError::internal_error(format!("IO 錯誤: {}", e), None)
            }
            McpToolError::Json(e) => {
                McpError::internal_error(format!("JSON 錯誤: {}", e), None)
            }
            McpToolError::Generic(e) => {
                McpError::internal_error(e.to_string(), None)
            }
        }
    }
}

/// 建立專案路径錯誤
pub fn project_path_error(msg: impl Into<String>) -> McpToolError {
    McpToolError::ProjectPath(msg.into())
}

/// 建立弹窗錯誤
pub fn popup_error(msg: impl Into<String>) -> McpToolError {
    McpToolError::PopupCreation(msg.into())
}

/// 建立回應解析錯誤
pub fn response_error(msg: impl Into<String>) -> McpToolError {
    McpToolError::ResponseParsing(msg.into())
}

/// 建立記憶管理錯誤
pub fn memory_error(msg: impl Into<String>) -> McpToolError {
    McpToolError::Memory(msg.into())
}
