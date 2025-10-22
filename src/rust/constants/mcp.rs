// MCP 工具相关常量

/// 寸止工具标识符
pub const TOOL_ZHI: &str = "zhi";

/// 記憶管理工具标识符
pub const TOOL_JI: &str = "ji";

/// 預設启用的工具列表
pub const DEFAULT_ENABLED_TOOLS: &[&str] = &[TOOL_ZHI, TOOL_JI];

/// 繼續回复預設启用狀態
pub const DEFAULT_CONTINUE_REPLY_ENABLED: bool = true;

/// 預設自動繼續阈值
pub const DEFAULT_AUTO_CONTINUE_THRESHOLD: u32 = 1000;

/// 預設繼續提示词
pub const DEFAULT_CONTINUE_PROMPT: &str = "请按照最佳实践繼續";

/// MCP 请求超时时间 (ms)
pub const REQUEST_TIMEOUT_MS: u64 = 30000;

/// MCP 重试次数
pub const MAX_RETRY_COUNT: u32 = 3;

// MCP 工具設定结构体
#[derive(Debug, Clone)]
pub struct McpToolConfig {
    pub tool_id: String,
    pub enabled: bool,
    pub can_disable: bool,
}

impl McpToolConfig {
    pub fn new(tool_id: &str, enabled: bool, can_disable: bool) -> Self {
        Self {
            tool_id: tool_id.to_string(),
            enabled,
            can_disable,
        }
    }
}

// MCP 設定结构体
#[derive(Debug, Clone)]
pub struct McpConfig {
    pub tools: Vec<McpToolConfig>,
    pub continue_reply_enabled: bool,
    pub auto_continue_threshold: u32,
    pub continue_prompt: String,
    pub request_timeout_ms: u64,
    pub max_retry_count: u32,
}

impl Default for McpConfig {
    fn default() -> Self {
        Self {
            tools: vec![
                McpToolConfig::new(TOOL_ZHI, true, false), // 寸止工具不可禁用
                McpToolConfig::new(TOOL_JI, true, true),   // 記憶管理工具可禁用
            ],
            continue_reply_enabled: DEFAULT_CONTINUE_REPLY_ENABLED,
            auto_continue_threshold: DEFAULT_AUTO_CONTINUE_THRESHOLD,
            continue_prompt: DEFAULT_CONTINUE_PROMPT.to_string(),
            request_timeout_ms: REQUEST_TIMEOUT_MS,
            max_retry_count: MAX_RETRY_COUNT,
        }
    }
}

impl McpConfig {
    /// 獲取工具設定
    pub fn get_tool_config(&self, tool_id: &str) -> Option<&McpToolConfig> {
        self.tools.iter().find(|tool| tool.tool_id == tool_id)
    }

    /// 檢查工具是否启用
    pub fn is_tool_enabled(&self, tool_id: &str) -> bool {
        self.get_tool_config(tool_id)
            .map(|tool| tool.enabled)
            .unwrap_or(false)
    }

    /// 設定工具启用狀態
    pub fn set_tool_enabled(&mut self, tool_id: &str, enabled: bool) -> bool {
        if let Some(tool) = self.tools.iter_mut().find(|tool| tool.tool_id == tool_id) {
            if tool.can_disable || enabled {
                tool.enabled = enabled;
                return true;
            }
        }
        false
    }

    /// 转换为 JSON 格式
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "tools": self.tools.iter().map(|tool| {
                serde_json::json!({
                    "id": tool.tool_id,
                    "enabled": tool.enabled,
                    "can_disable": tool.can_disable
                })
            }).collect::<Vec<_>>(),
            "continue_reply_enabled": self.continue_reply_enabled,
            "auto_continue_threshold": self.auto_continue_threshold,
            "continue_prompt": self.continue_prompt,
            "request_timeout_ms": self.request_timeout_ms,
            "max_retry_count": self.max_retry_count
        })
    }
}

// 便捷函數
/// 獲取預設 MCP 設定
pub fn get_default_mcp_config() -> McpConfig {
    McpConfig::default()
}

/// 檢查是否为有效的工具 ID
pub fn is_valid_tool_id(tool_id: &str) -> bool {
    matches!(tool_id, TOOL_ZHI | TOOL_JI)
}
