use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use crate::constants::{window, theme, audio, mcp, font};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    #[serde(default = "default_ui_config")]
    pub ui_config: UiConfig, // UI相關設定（主題、視窗、置頂等）
    #[serde(default = "default_audio_config")]
    pub audio_config: AudioConfig, // 音訊相關設定
    #[serde(default = "default_reply_config")]
    pub reply_config: ReplyConfig, // 繼續回覆設定
    #[serde(default = "default_mcp_config")]
    pub mcp_config: McpConfig, // MCP工具設定
    #[serde(default = "default_custom_prompt_config")]
    pub custom_prompt_config: CustomPromptConfig, // 自訂prompt設定
    #[serde(default = "default_shortcut_config")]
    pub shortcut_config: ShortcutConfig, // 自訂快捷鍵設定
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UiConfig {
    // 主題設定
    #[serde(default = "default_theme")]
    pub theme: String, // "light", "dark"

    // 語言設定
    #[serde(default = "default_language")]
    pub language: String, // "zh-CN", "zh-TW"

    // 字型設定
    #[serde(default = "default_font_config")]
    pub font_config: FontConfig,

    // 視窗設定
    #[serde(default = "default_window_config")]
    pub window_config: WindowConfig,

    // 置頂設定
    #[serde(default = "default_always_on_top")]
    pub always_on_top: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FontConfig {
    // 字型系列
    #[serde(default = "default_font_family")]
    pub font_family: String, // "inter", "jetbrains-mono", "system", "custom"

    // 字型大小
    #[serde(default = "default_font_size")]
    pub font_size: String, // "small", "medium", "large"

    // 自訂字型系列（當 font_family 为 "custom" 時使用）
    #[serde(default = "default_custom_font_family")]
    pub custom_font_family: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowConfig {
    // 視窗約束設定
    #[serde(default = "default_auto_resize")]
    pub auto_resize: bool,
    #[serde(default = "default_max_width")]
    pub max_width: f64,
    #[serde(default = "default_max_height")]
    pub max_height: f64,
    #[serde(default = "default_min_width")]
    pub min_width: f64,
    #[serde(default = "default_min_height")]
    pub min_height: f64,

    // 目前模式
    #[serde(default = "default_window_fixed")]
    pub fixed: bool,

    // 固定模式的尺寸設定
    #[serde(default = "default_fixed_width")]
    pub fixed_width: f64,
    #[serde(default = "default_fixed_height")]
    pub fixed_height: f64,

    // 自由拉伸模式的尺寸設定
    #[serde(default = "default_free_width")]
    pub free_width: f64,
    #[serde(default = "default_free_height")]
    pub free_height: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioConfig {
    #[serde(default = "default_audio_notification_enabled")]
    pub notification_enabled: bool,
    #[serde(default = "default_audio_url")]
    pub custom_url: String, // 自訂音效URL
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReplyConfig {
    #[serde(default = "default_enable_continue_reply")]
    pub enable_continue_reply: bool,
    #[serde(default = "default_auto_continue_threshold")]
    pub auto_continue_threshold: u32, // 字元數閾值
    #[serde(default = "default_continue_prompt")]
    pub continue_prompt: String, // 繼續回覆的提示詞
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct McpConfig {
    #[serde(default = "default_mcp_tools")]
    pub tools: HashMap<String, bool>, // MCP工具啟用狀態
}

// 自訂prompt結構
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomPrompt {
    pub id: String,
    pub name: String,
    pub content: String,
    pub description: Option<String>,
    pub sort_order: i32,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default = "default_prompt_type")]
    pub r#type: String, // "normal" | "conditional"
    // 條件性prompt專用欄位
    pub condition_text: Option<String>,    // 條件描述文字
    pub template_true: Option<String>,     // 開關為true時的範本
    pub template_false: Option<String>,    // 開關為false時的範本
    #[serde(default = "default_prompt_state")]
    pub current_state: bool,               // 目前開關狀態（原default_state）
}

// 自訂prompt設定
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomPromptConfig {
    #[serde(default = "default_custom_prompts")]
    pub prompts: Vec<CustomPrompt>,
    #[serde(default = "default_custom_prompt_enabled")]
    pub enabled: bool,
    #[serde(default = "default_custom_prompt_max_prompts")]
    pub max_prompts: u32,
}

// 快捷鍵設定
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShortcutConfig {
    #[serde(default = "default_shortcuts")]
    pub shortcuts: HashMap<String, ShortcutBinding>,
}

// 快捷鍵綁定
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShortcutBinding {
    pub id: String,
    pub name: String,
    pub description: String,
    pub action: String, // "submit", "exit", "custom"
    pub key_combination: ShortcutKey,
    pub enabled: bool,
    pub scope: String, // "global", "popup", "input"
}

// 快捷鍵組合
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShortcutKey {
    pub key: String, // 主键，如 "Enter", "Q", "F4"
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub meta: bool, // macOS的Cmd键
}

// Telegram 功能已移除

#[derive(Debug)]
pub struct AppState {
    pub config: Mutex<AppConfig>,
    pub response_channel: Mutex<Option<tokio::sync::oneshot::Sender<String>>>,
    // 防誤觸退出機制
    pub exit_attempt_count: Mutex<u32>,
    pub last_exit_attempt: Mutex<Option<std::time::Instant>>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            ui_config: default_ui_config(),
            audio_config: default_audio_config(),
            reply_config: default_reply_config(),
            mcp_config: default_mcp_config(),
            custom_prompt_config: default_custom_prompt_config(),
            shortcut_config: default_shortcut_config(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            config: Mutex::new(AppConfig::default()),
            response_channel: Mutex::new(None),
            exit_attempt_count: Mutex::new(0),
            last_exit_attempt: Mutex::new(None),
        }
    }
}

// 預設值函數
pub fn default_ui_config() -> UiConfig {
    UiConfig {
        theme: default_theme(),
        language: default_language(),
        font_config: default_font_config(),
        window_config: default_window_config(),
        always_on_top: default_always_on_top(),
    }
}

pub fn default_audio_config() -> AudioConfig {
    AudioConfig {
        notification_enabled: default_audio_notification_enabled(),
        custom_url: default_audio_url(),
    }
}

pub fn default_mcp_config() -> McpConfig {
    McpConfig {
        tools: default_mcp_tools(),
    }
}

// Telegram 功能已移除

pub fn default_custom_prompt_config() -> CustomPromptConfig {
    CustomPromptConfig {
        prompts: default_custom_prompts(),
        enabled: default_custom_prompt_enabled(),
        max_prompts: default_custom_prompt_max_prompts(),
    }
}

pub fn default_always_on_top() -> bool {
    window::DEFAULT_ALWAYS_ON_TOP
}

pub fn default_audio_notification_enabled() -> bool {
    audio::DEFAULT_NOTIFICATION_ENABLED
}

pub fn default_theme() -> String {
    theme::DEFAULT.to_string()
}

pub fn default_language() -> String {
    "zh-TW".to_string()
}

pub fn default_audio_url() -> String {
    audio::DEFAULT_URL.to_string()
}

pub fn default_window_config() -> WindowConfig {
    WindowConfig {
        auto_resize: window::DEFAULT_AUTO_RESIZE,
        max_width: window::MAX_WIDTH,
        max_height: window::MAX_HEIGHT,
        min_width: window::MIN_WIDTH,
        min_height: window::MIN_HEIGHT,
        fixed: window::DEFAULT_FIXED_MODE,
        fixed_width: window::DEFAULT_WIDTH,
        fixed_height: window::DEFAULT_HEIGHT,
        free_width: window::DEFAULT_WIDTH,
        free_height: window::DEFAULT_HEIGHT,
    }
}

pub fn default_reply_config() -> ReplyConfig {
    ReplyConfig {
        enable_continue_reply: mcp::DEFAULT_CONTINUE_REPLY_ENABLED,
        auto_continue_threshold: mcp::DEFAULT_AUTO_CONTINUE_THRESHOLD,
        continue_prompt: mcp::DEFAULT_CONTINUE_PROMPT.to_string(),
    }
}

pub fn default_auto_resize() -> bool {
    true
}

pub fn default_max_width() -> f64 {
    window::MAX_WIDTH
}

pub fn default_max_height() -> f64 {
    window::MAX_HEIGHT
}

pub fn default_min_width() -> f64 {
    window::MIN_WIDTH
}

pub fn default_min_height() -> f64 {
    window::MIN_HEIGHT
}

pub fn default_enable_continue_reply() -> bool {
    mcp::DEFAULT_CONTINUE_REPLY_ENABLED
}

pub fn default_auto_continue_threshold() -> u32 {
    mcp::DEFAULT_AUTO_CONTINUE_THRESHOLD
}

pub fn default_continue_prompt() -> String {
    mcp::DEFAULT_CONTINUE_PROMPT.to_string()
}

pub fn default_mcp_tools() -> HashMap<String, bool> {
    let mut tools = HashMap::new();
    tools.insert(mcp::TOOL_ZHI.to_string(), true); // 寸止工具預設啟用
    tools.insert(mcp::TOOL_JI.to_string(), true); // 記憶管理工具預設啟用
    tools
}

pub fn default_window_width() -> f64 {
    window::DEFAULT_WIDTH
}

pub fn default_window_height() -> f64 {
    window::DEFAULT_HEIGHT
}

pub fn default_window_fixed() -> bool {
    window::DEFAULT_FIXED_MODE
}

pub fn default_fixed_width() -> f64 {
    window::DEFAULT_WIDTH
}

pub fn default_fixed_height() -> f64 {
    window::DEFAULT_HEIGHT
}

pub fn default_free_width() -> f64 {
    window::DEFAULT_WIDTH
}

pub fn default_free_height() -> f64 {
    window::DEFAULT_HEIGHT
}

// Telegram 功能已移除

impl WindowConfig {
    // 獲取目前模式的宽度
    pub fn current_width(&self) -> f64 {
        if self.fixed {
            self.fixed_width
        } else {
            self.free_width
        }
    }

    // 獲取目前模式的高度
    pub fn current_height(&self) -> f64 {
        if self.fixed {
            self.fixed_height
        } else {
            self.free_height
        }
    }

    // 更新目前模式的尺寸
    pub fn update_current_size(&mut self, width: f64, height: f64) {
        if self.fixed {
            self.fixed_width = width;
            self.fixed_height = height;
        } else {
            self.free_width = width;
            self.free_height = height;
        }
    }
}

// 字型設定預設值函數
pub fn default_font_config() -> FontConfig {
    FontConfig {
        font_family: default_font_family(),
        font_size: default_font_size(),
        custom_font_family: default_custom_font_family(),
    }
}

pub fn default_font_family() -> String {
    font::DEFAULT_FONT_FAMILY.to_string()
}

pub fn default_font_size() -> String {
    font::DEFAULT_FONT_SIZE.to_string()
}

pub fn default_custom_font_family() -> String {
    font::DEFAULT_CUSTOM_FONT_FAMILY.to_string()
}

pub fn default_prompt_type() -> String {
    "normal".to_string()
}

pub fn default_prompt_state() -> bool {
    false
}



// 自訂prompt預設值函數
pub fn default_custom_prompts() -> Vec<CustomPrompt> {
    vec![
        CustomPrompt {
            id: "default_1".to_string(),
            name: "✅Done".to_string(),
            content: "結束當前對話".to_string(),
            description: Some("請求AI結束工作".to_string()),
            sort_order: 1,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            r#type: "normal".to_string(),
            condition_text: None,
            template_true: None,
            template_false: None,
            current_state: false,
        },
        CustomPrompt {
            id: "default_2".to_string(),
            name: "🧹Clear".to_string(),
            content: "".to_string(),
            description: Some("清空輸入框內容".to_string()),
            sort_order: 2,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            r#type: "normal".to_string(),
            condition_text: None,
            template_true: None,
            template_false: None,
            current_state: false,
        },
        CustomPrompt {
            id: "default_3".to_string(),
            name: "✨New Issue".to_string(),
            content: "ok，完美，新的需求or问题，".to_string(),
            description: Some("准备新的需求or问题".to_string()),
            sort_order: 3,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            r#type: "normal".to_string(),
            condition_text: None,
            template_true: None,
            template_false: None,
            current_state: false,
        },
        CustomPrompt {
            id: "default_4".to_string(),
            name: "🧠Remember".to_string(),
            content: "請记住，".to_string(),
            description: Some("寸止的另一个工具，請记住".to_string()),
            sort_order: 4,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            r#type: "normal".to_string(),
            condition_text: None,
            template_true: None,
            template_false: None,
            current_state: false,
        },
        CustomPrompt {
            id: "default_5".to_string(),
            name: "📝Summary And Restart".to_string(),
            content: "本次對話的上下文已经太长了，我打算关掉并重新开一个新的会话。你有什么想对你的继任者说的，以便它能更好的理解你當前的工作并顺利繼續？".to_string(),
            description: Some("總結-开新会话".to_string()),
            sort_order: 5,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            r#type: "normal".to_string(),
            condition_text: None,
            template_true: None,
            template_false: None,
            current_state: false,
        },
        CustomPrompt {
            id: "default_6".to_string(),
            name: "🔍Review And Plan".to_string(),
            content: "請執行以下專案进度檢查和规划任务：\n\n1. **專案进度分析**：\n   - 查看當前代码函式庫狀態，分析已完成的功能模組\n   - 识別已完成、進行中和待開始的功能点\n\n2. **里程碑确定**：\n   - 基于當前进度和剩余工作量，定義清晰的里程碑節点\n   - 为每个里程碑設定具体的完成標準和時间预期\n   - 优先考虑核心任务管理功能的里程碑\n\n3. **文档更新**（注意：僅更新现有文档，不建立新文档）：\n   - 更新專案规划文档中的进度狀態\n   - 修正任何与實際實作不符的技术方案描述\n   - 确保文档反映當前的技术栈和架构决策\n\n4. **下一步工作规划**：\n   - 基于用户偏好（系統化开发方法、前端优先、分步骤反馈）制定具体的下一阶段工作计划\n   - 识別关键路径上的阻塞点和依赖关系\n   - 提供3-5个具体的下一步行动项，按优先级排序\n\n5. **反馈收集**：\n   - 在完成分析后，使用寸止工具收集用户对进度评估和下一步计划的反馈\n   - 提供多个可选的发展方向供用户選擇".to_string(),
            description: Some("專案进度檢查和规划任务".to_string()),
            sort_order: 6,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            r#type: "normal".to_string(),
            condition_text: None,
            template_true: None,
            template_false: None,
            current_state: false,
        },
        CustomPrompt {
            id: "default_7".to_string(),
            name: "是否生成總結性Markdown文档".to_string(),
            content: "".to_string(),
            description: Some("是否生成總結性Markdown文档".to_string()),
            sort_order: 7,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            r#type: "conditional".to_string(),
            condition_text: Some("是否生成總結性Markdown文档".to_string()),
            template_true: Some("✔️請记住，帮我生成總結性Markdown文档".to_string()),
            template_false: Some("❌請记住，不要生成總結性Markdown文档".to_string()),
            current_state: false,
        },
        CustomPrompt {
            id: "default_8".to_string(),
            name: "是否生成測試脚本".to_string(),
            content: "".to_string(),
            description: Some("是否生成測試脚本".to_string()),
            sort_order: 8,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            r#type: "conditional".to_string(),
            condition_text: Some("是否生成測試脚本".to_string()),
            template_true: Some("✔️請记住，帮我生成測試脚本".to_string()),
            template_false: Some("❌請记住，不要生成測試脚本".to_string()),
            current_state: false,
        },
        CustomPrompt {
            id: "default_9".to_string(),
            name: "是否主动編譯".to_string(),
            content: "".to_string(),
            description: Some("是否主动編譯".to_string()),
            sort_order: 9,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            r#type: "conditional".to_string(),
            condition_text: Some("是否主动編譯".to_string()),
            template_true: Some("✔️請记住，帮我編譯".to_string()),
            template_false: Some("❌請记住，不要編譯，用户自己編譯".to_string()),
            current_state: false,
        },
        CustomPrompt {
            id: "default_10".to_string(),
            name: "是否主动執行".to_string(),
            content: "".to_string(),
            description: Some("是否主动執行".to_string()),
            sort_order: 10,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            r#type: "conditional".to_string(),
            condition_text: Some("是否主动執行".to_string()),
            template_true: Some("✔️請记住，帮我執行".to_string()),
            template_false: Some("❌請记住，不要執行，用户自己執行".to_string()),
            current_state: false,
        },
    ]
}

pub fn default_custom_prompt_enabled() -> bool {
    true
}

pub fn default_custom_prompt_max_prompts() -> u32 {
    50
}

// 快捷键預設值函數
pub fn default_shortcut_config() -> ShortcutConfig {
    ShortcutConfig {
        shortcuts: default_shortcuts(),
    }
}

pub fn default_shortcuts() -> HashMap<String, ShortcutBinding> {
    let mut shortcuts = HashMap::new();

    // 快速發送快捷鍵
    shortcuts.insert("quick_submit".to_string(), ShortcutBinding {
        id: "quick_submit".to_string(),
        name: "快速傳送".to_string(),
        description: "快速提交當前輸入內容".to_string(),
        action: "submit".to_string(),
        key_combination: ShortcutKey {
            key: "Enter".to_string(),
            ctrl: true,
            alt: false,
            shift: false,
            meta: false,
        },
        enabled: true,
        scope: "popup".to_string(),
    });

    // 增強快捷鍵
    shortcuts.insert("enhance".to_string(), ShortcutBinding {
        id: "enhance".to_string(),
        name: "增强".to_string(),
        description: "增强當前輸入內容".to_string(),
        action: "enhance".to_string(),
        key_combination: ShortcutKey {
            key: "Enter".to_string(),
            ctrl: true,
            alt: false,
            shift: true,
            meta: false,
        },
        enabled: true,
        scope: "popup".to_string(),
    });

    // 繼續快捷鍵
    shortcuts.insert("continue".to_string(), ShortcutBinding {
        id: "continue".to_string(),
        name: "繼續".to_string(),
        description: "繼續對話".to_string(),
        action: "continue".to_string(),
        key_combination: ShortcutKey {
            key: "Enter".to_string(),
            ctrl: false,
            alt: true,
            shift: false,
            meta: false,
        },
        enabled: true,
        scope: "popup".to_string(),
    });

    shortcuts
}


