//! 智能代码审查交互工具模組
//!
//! 提供智能代码审查交互功能，支持预定义選項、自由文本輸入和图片上传

pub mod mcp;

// 重新匯出主要類型和功能
pub use mcp::InteractionTool;
