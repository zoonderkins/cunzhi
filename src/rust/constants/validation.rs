// 驗證相关常量和函數

use super::window;
use super::audio;
use super::theme;
use super::network;

// 通用驗證函數

/// 驗證字符串是否为空
pub fn is_empty_string(s: &str) -> bool {
    s.trim().is_empty()
}

/// 驗證字符串长度是否在范围内
pub fn is_valid_string_length(s: &str, min: usize, max: usize) -> bool {
    let len = s.len();
    len >= min && len <= max
}

/// 驗證数值是否在范围内
pub fn is_in_range<T: PartialOrd>(value: T, min: T, max: T) -> bool {
    value >= min && value <= max
}

/// 驗證 URL 格式是否有效
pub fn is_valid_url(url: &str) -> bool {
    if url.is_empty() {
        return true; // 空 URL 被认为是有效的（使用預設值）
    }
    
    url.starts_with("http://") || url.starts_with("https://") || url.starts_with("file://")
}

/// 驗證檔案路径是否有效
pub fn is_valid_file_path(path: &str) -> bool {
    !path.is_empty() && !path.contains('\0')
}

/// 驗證端口号是否有效
pub fn is_valid_port(port: u16) -> bool {
    port > 0
}

// 視窗驗證函數
pub use window::is_valid_window_size;

/// 驗證視窗位置是否有效
pub fn is_valid_window_position(x: i32, y: i32) -> bool {
    // 允许负值，因为多顯示器環境下可能有负坐标
    (-10000..=10000).contains(&x) && (-10000..=10000).contains(&y)
}

// 音訊驗證函數
pub use audio::{is_supported_audio_format, is_valid_audio_file_size};

/// 驗證音量是否有效
pub fn is_valid_volume(volume: f32) -> bool {
    is_in_range(volume, 0.0, 1.0)
}

// 主題驗證函數
pub use theme::is_valid_theme;

// Telegram 功能已移除

// 網路驗證函數

/// 驗證超時時间是否有效
pub fn is_valid_timeout(timeout_ms: u64) -> bool {
    is_in_range(timeout_ms, 100, 300000) // 100ms 到 5分钟
}

/// 驗證重試次数是否有效
pub fn is_valid_retry_count(count: u32) -> bool {
    count <= network::MAX_RETRY_COUNT
}

// 設定驗證结构体
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: String) {
        self.is_valid = false;
        self.errors.push(error);
    }

    pub fn merge(&mut self, other: ValidationResult) {
        if !other.is_valid {
            self.is_valid = false;
            self.errors.extend(other.errors);
        }
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

// 综合驗證函數

/// 驗證視窗設定
pub fn validate_window_config(
    width: f64,
    height: f64,
    x: Option<i32>,
    y: Option<i32>,
) -> ValidationResult {
    let mut result = ValidationResult::new();

    if !is_valid_window_size(width, height) {
        result.add_error(format!("无效的視窗尺寸: {}x{}", width, height));
    }

    if let (Some(x), Some(y)) = (x, y) {
        if !is_valid_window_position(x, y) {
            result.add_error(format!("无效的視窗位置: ({}, {})", x, y));
        }
    }

    result
}

/// 驗證音訊設定
pub fn validate_audio_config(url: &str, volume: f32, enabled: bool) -> ValidationResult {
    let mut result = ValidationResult::new();

    if enabled && !url.is_empty() && !is_valid_url(url) {
        result.add_error(format!("无效的音訊 URL: {}", url));
    }

    if !is_valid_volume(volume) {
        result.add_error(format!("无效的音量值: {}", volume));
    }

    result
}

/// 驗證網路設定
pub fn validate_network_config(
    timeout_ms: u64,
    retry_count: u32,
    retry_interval_ms: u64,
) -> ValidationResult {
    let mut result = ValidationResult::new();

    if !is_valid_timeout(timeout_ms) {
        result.add_error(format!("无效的超時時间: {}ms", timeout_ms));
    }

    if !is_valid_retry_count(retry_count) {
        result.add_error(format!("无效的重試次数: {}", retry_count));
    }

    if !is_in_range(retry_interval_ms, network::MIN_RETRY_INTERVAL_MS, network::MAX_RETRY_INTERVAL_MS) {
        result.add_error(format!("无效的重試间隔: {}ms", retry_interval_ms));
    }

    result
}
