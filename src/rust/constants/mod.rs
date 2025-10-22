// 常量模組
// 将所有常量按功能模組化管理，便于維護和擴充

pub mod app;
pub mod audio;
pub mod font;
pub mod mcp;
pub mod network;
pub mod theme;
pub mod ui;
pub mod validation;
pub mod window;

// 重新匯出所有常量，方便使用
// 注意：为了避免命名冲突，某些模組使用限定匯出
pub use app::*;
pub use audio::*;
pub use theme::*;
pub use ui::*;
pub use validation::*;
pub use window::*;

// 这些模組有重复的常量名，使用模組限定访问
// pub use mcp::*;
// pub use network::*;
