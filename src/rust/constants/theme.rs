// 主題相关常量

/// 預設主題
pub const DEFAULT: &str = "dark";

/// 浅色主題
pub const LIGHT: &str = "light";

/// 深色主題
pub const DARK: &str = "dark";

/// 可用主題列表
pub const AVAILABLE_THEMES: &[&str] = &[LIGHT, DARK];

// 主題設定结构体
#[derive(Debug, Clone)]
pub struct ThemeConfig {
    pub default_theme: String,
    pub available_themes: Vec<String>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            default_theme: DEFAULT.to_string(),
            available_themes: AVAILABLE_THEMES.iter().map(|s| s.to_string()).collect(),
        }
    }
}

impl ThemeConfig {
    /// 驗證主題是否有效
    pub fn is_valid_theme(&self, theme: &str) -> bool {
        self.available_themes.contains(&theme.to_string())
    }

    /// 獲取預設主題
    pub fn get_default(&self) -> &str {
        &self.default_theme
    }

    /// 转换为 JSON 格式
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "default": self.default_theme,
            "available": self.available_themes
        })
    }
}

// 便捷函數
/// 獲取預設主題設定
pub fn get_default_theme_config() -> ThemeConfig {
    ThemeConfig::default()
}

/// 驗證主題是否有效
pub fn is_valid_theme(theme: &str) -> bool {
    AVAILABLE_THEMES.contains(&theme)
}

/// 獲取預設主題
pub fn get_default_theme() -> &'static str {
    DEFAULT
}
