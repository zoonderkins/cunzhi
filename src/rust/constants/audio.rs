// 音訊相关常量

/// 預設音訊 URL（空字符串表示使用内置音效）
pub const DEFAULT_URL: &str = "";

/// 預設音訊通知啟用狀態
pub const DEFAULT_NOTIFICATION_ENABLED: bool = false;

/// 音訊檔案支持的格式
pub const SUPPORTED_FORMATS: &[&str] = &["mp3", "wav", "ogg", "m4a"];

/// 預設音量 (0.0 - 1.0)
pub const DEFAULT_VOLUME: f32 = 0.8;

/// 最大音訊檔案大小 (bytes) - 10MB
pub const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;

// 音訊設定结构体
#[derive(Debug, Clone)]
pub struct AudioConfig {
    pub default_url: String,
    pub notification_enabled: bool,
    pub supported_formats: Vec<String>,
    pub default_volume: f32,
    pub max_file_size: u64,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            default_url: DEFAULT_URL.to_string(),
            notification_enabled: DEFAULT_NOTIFICATION_ENABLED,
            supported_formats: SUPPORTED_FORMATS.iter().map(|s| s.to_string()).collect(),
            default_volume: DEFAULT_VOLUME,
            max_file_size: MAX_FILE_SIZE,
        }
    }
}

impl AudioConfig {
    /// 驗證音訊格式是否支持
    pub fn is_supported_format(&self, format: &str) -> bool {
        self.supported_formats.contains(&format.to_lowercase())
    }

    /// 驗證音訊檔案大小是否有效
    pub fn is_valid_file_size(&self, size: u64) -> bool {
        size <= self.max_file_size
    }

    /// 驗證音量是否有效
    pub fn is_valid_volume(&self, volume: f32) -> bool {
        (0.0..=1.0).contains(&volume)
    }

    /// 转换为 JSON 格式
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "default_url": self.default_url,
            "notification_enabled": self.notification_enabled,
            "supported_formats": self.supported_formats,
            "default_volume": self.default_volume,
            "max_file_size": self.max_file_size
        })
    }
}

// 便捷函數
/// 獲取預設音訊設定
pub fn get_default_audio_config() -> AudioConfig {
    AudioConfig::default()
}

/// 驗證音訊格式是否支持
pub fn is_supported_audio_format(format: &str) -> bool {
    SUPPORTED_FORMATS.contains(&format.to_lowercase().as_str())
}

/// 驗證音訊檔案大小是否有效
pub fn is_valid_audio_file_size(size: u64) -> bool {
    size <= MAX_FILE_SIZE
}
