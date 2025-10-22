pub mod app;
pub mod config;
pub mod constants;
pub mod mcp;
pub mod ui;
pub mod utils;

// 避免重名匯出，使用限定匯出
pub use config::*;
pub use utils::*;

// 選擇性匯出常用項，避免衝突
pub use constants::{app as app_constants, theme, validation, network};
pub use mcp::{server, tools, types, handlers, utils as mcp_utils};
pub use ui::{window as ui_window, audio as ui_audio, audio_assets, updater};
