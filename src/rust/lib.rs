pub mod config;
pub mod constants;
pub mod mcp;
pub mod telegram;
pub mod ui;
pub mod utils;

// 避免重名导出，使用限定导出
pub use config::*;
pub use utils::*;

// 这些模块有重名项，使用模块限定访问
// pub use constants::*;  // 与其他模块有重名：AudioConfig, audio, window, get_app_info
// pub use mcp::*;        // 与其他模块有重名：commands
// pub use ui::*;         // 与其他模块有重名：commands, audio, window, get_app_info

// 选择性导出常用项，避免冲突
pub use constants::{app, theme, validation, network, telegram as telegram_constants};
pub use mcp::{server, tools, types, handlers, utils as mcp_utils};
pub use ui::{window as ui_window, audio as ui_audio, audio_assets, updater};
