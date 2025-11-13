use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Once;
use log::LevelFilter;
use env_logger::{Builder, Target};

static INIT: Once = Once::new();

/// 日誌設定
#[derive(Debug, Clone)]
pub struct LogConfig {
    /// 日誌级別
    pub level: LevelFilter,
    /// 日誌檔案路径（None 表示不輸出到檔案）
    pub file_path: Option<String>,
    /// 是否为 MCP 模式（MCP 模式下不輸出到 stderr）
    pub is_mcp_mode: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: LevelFilter::Warn,
            file_path: None,
            is_mcp_mode: false,
        }
    }
}

/// 初始化日誌系統
pub fn init_logger(config: LogConfig) -> Result<(), Box<dyn std::error::Error>> {
    INIT.call_once(|| {
        let mut builder = Builder::new();
        
        // 設定日誌级別
        builder.filter_level(config.level);
        
        // 設定日誌格式
        builder.format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] [{}] {}",
                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                record.level(),
                record.module_path().unwrap_or("unknown"),
                record.args()
            )
        });
        
        // 根据模式設定輸出目标
        if config.is_mcp_mode {
            // MCP 模式：只輸出到檔案，不輸出到 stderr
            if let Some(file_path) = &config.file_path {
                if let Ok(log_file) = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(file_path) 
                {
                    builder.target(Target::Pipe(Box::new(log_file)));
                } else {
                    // 如果檔案開啟失敗，禁用日誌輸出
                    builder.filter_level(LevelFilter::Off);
                }
            } else {
                // MCP 模式下没有指定檔案路径，禁用日誌輸出
                builder.filter_level(LevelFilter::Off);
            }
        } else {
            // 非 MCP 模式：輸出到 stderr
            builder.target(Target::Stderr);
        }
        
        builder.init();
    });
    
    Ok(())
}

/// 自動检測模式并初始化日誌系統
pub fn auto_init_logger() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let is_mcp_mode = args.len() >= 3 && args[1] == "--mcp-request";
    
    let config = if is_mcp_mode {
        // MCP 模式：輸出到檔案
        let log_file_path = env::var("MCP_LOG_FILE")
            .unwrap_or_else(|_| {
                let temp_dir = env::temp_dir();
                temp_dir.join("cunzhi-mcp.log").to_string_lossy().to_string()
            });
            
        LogConfig {
            level: env::var("RUST_LOG")
                .unwrap_or_else(|_| "warn".to_string())
                .parse::<LevelFilter>()
                .unwrap_or(LevelFilter::Warn),
            file_path: Some(log_file_path),
            is_mcp_mode: true,
        }
    } else {
        // GUI 模式：輸出到 stderr
        LogConfig {
            level: env::var("RUST_LOG")
                .unwrap_or_else(|_| "info".to_string())
                .parse::<LevelFilter>()
                .unwrap_or(LevelFilter::Info),
            file_path: None,
            is_mcp_mode: false,
        }
    };
    
    init_logger(config)
}

/// 便利宏：只在重要情况下記錄日誌
#[macro_export]
macro_rules! log_important {
    (error, $($arg:tt)*) => {
        log::error!($($arg)*)
    };
    (warn, $($arg:tt)*) => {
        log::warn!($($arg)*)
    };
    (info, $($arg:tt)*) => {
        log::info!($($arg)*)
    };
}

/// 便利宏：偵錯日誌（只在 debug 级別下輸出）
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        log::debug!($($arg)*)
    };
}

/// 便利宏：跟踪日誌（只在 trace 级別下輸出）
#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        log::trace!($($arg)*)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_log_config_default() {
        let config = LogConfig::default();
        assert_eq!(config.level, LevelFilter::Warn);
        assert_eq!(config.file_path, None);
        assert_eq!(config.is_mcp_mode, false);
    }
    
    #[test]
    fn test_mcp_mode_detection() {
        // 这个測試需要在實際環境中執行
        // 这里只是展示如何測試
    }
}
