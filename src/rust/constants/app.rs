// 應用程序相关常量

/// 應用程序名称
pub const NAME: &str = "寸止";

/// 應用程序英文名称
pub const NAME_EN: &str = "cunzhi";

/// 應用程序描述
pub const DESCRIPTION: &str = "智慧程式碼審查工具";

/// 應用程序版本（从 Cargo.toml 獲取）
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 應用程序作者
pub const AUTHOR: &str = "zoonderkins";

/// 應用程序主页
pub const HOMEPAGE: &str = "https://github.com/zoonderkins/cunzhi";

/// 應用程序仓函式庫
pub const REPOSITORY: &str = "https://github.com/zoonderkins/cunzhi";

/// 應用程序许可证
pub const LICENSE: &str = "MIT";

/// 設定檔案名
pub const CONFIG_FILE_NAME: &str = "config.json";

/// 日誌檔案名前缀
pub const LOG_FILE_PREFIX: &str = "cunzhi";

/// 應用程序标识符（用于系統集成）
pub const APP_IDENTIFIER: &str = "com.zoonderkins.cunzhi";

/// 用户代理字符串
pub const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

// 防误触退出相关常量
/// 退出確認時间視窗（秒）
pub const EXIT_CONFIRMATION_WINDOW_SECS: u64 = 3;

/// 需要的连续退出嘗試次数
pub const REQUIRED_EXIT_ATTEMPTS: u32 = 2;

// 應用程序訊息结构体
#[derive(Debug, Clone)]
pub struct AppInfo {
    pub name: String,
    pub name_en: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub homepage: String,
    pub repository: String,
    pub license: String,
    pub identifier: String,
    pub user_agent: String,
}

impl Default for AppInfo {
    fn default() -> Self {
        Self {
            name: NAME.to_string(),
            name_en: NAME_EN.to_string(),
            description: DESCRIPTION.to_string(),
            version: VERSION.to_string(),
            author: AUTHOR.to_string(),
            homepage: HOMEPAGE.to_string(),
            repository: REPOSITORY.to_string(),
            license: LICENSE.to_string(),
            identifier: APP_IDENTIFIER.to_string(),
            user_agent: USER_AGENT.to_string(),
        }
    }
}

impl AppInfo {
    /// 獲取完整的應用程序標題
    pub fn get_full_title(&self) -> String {
        format!("{} v{}", self.name, self.version)
    }

    /// 獲取視窗標題
    pub fn get_window_title(&self) -> String {
        self.name.clone()
    }

    /// 转换为 JSON 格式
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name,
            "name_en": self.name_en,
            "description": self.description,
            "version": self.version,
            "author": self.author,
            "homepage": self.homepage,
            "repository": self.repository,
            "license": self.license,
            "identifier": self.identifier,
            "user_agent": self.user_agent
        })
    }
}

// 便捷函數
/// 獲取應用程序訊息
pub fn get_app_info() -> AppInfo {
    AppInfo::default()
}

/// 獲取應用程序名称
pub fn get_app_name() -> &'static str {
    NAME
}

/// 獲取應用程序版本
pub fn get_app_version() -> &'static str {
    VERSION
}

/// 獲取用户代理字符串
pub fn get_user_agent() -> &'static str {
    USER_AGENT
}
