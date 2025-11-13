//! 記憶管理工具模組
//!
//! 提供全局記憶管理功能，用于存储和管理重要的开发规范、用户偏好和最佳實務

pub mod manager;
pub mod types;
pub mod mcp;

// 重新匯出主要類型和功能
pub use manager::MemoryManager;
pub use types::{MemoryEntry, MemoryCategory, MemoryMetadata};
pub use mcp::MemoryTool;
