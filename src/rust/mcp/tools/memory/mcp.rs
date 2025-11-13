use anyhow::Result;
use rmcp::{ErrorData as McpError, model::*};

use super::{MemoryManager, MemoryCategory};
use crate::mcp::{JiyiRequest, utils::{validate_project_path, project_path_error}};

/// 全局記憶管理工具
///
/// 用于存储和管理重要的开发规范、用户偏好和最佳實務
#[derive(Clone)]
pub struct MemoryTool;

impl MemoryTool {
    pub async fn jiyi(
        request: JiyiRequest,
    ) -> Result<CallToolResult, McpError> {
        // 使用增强的路径驗證功能
        if let Err(e) = validate_project_path(&request.project_path) {
            return Err(project_path_error(format!(
                "路径驗證失敗: {}\n原始路径: {}\n請檢查路径格式是否正确，特別是 Windows 路径应使用正确的盘符格式（如 C:\\path）",
                e,
                request.project_path
            )).into());
        }

        let manager = MemoryManager::new(&request.project_path)
            .map_err(|e| McpError::internal_error(format!("建立記憶管理器失敗: {}", e), None))?;

        let result = match request.action.as_str() {
            "記憶" => {
                if request.content.trim().is_empty() {
                    return Err(McpError::invalid_params("缺少記憶內容".to_string(), None));
                }

                let category = match request.category.as_str() {
                    "rule" => MemoryCategory::Rule,
                    "preference" => MemoryCategory::Preference,
                    "pattern" => MemoryCategory::Pattern,
                    "context" => MemoryCategory::Context,
                    _ => MemoryCategory::Context,
                };

                let id = manager.add_memory(&request.content, category)
                    .map_err(|e| McpError::internal_error(format!("新增記憶失敗: {}", e), None))?;

                format!("✅ 記憶已新增，ID: {}\n📝 內容: {}\n📂 分類: {:?}", id, request.content, category)
            }
            "回忆" | "回憶" => {
                manager.get_project_info()
                    .map_err(|e| McpError::internal_error(format!("獲取專案訊息失敗: {}", e), None))?
            }
            _ => {
                return Err(McpError::invalid_params(
                    format!("未知的操作類型: {}（支援：記憶/記憶、回忆/回憶）", request.action),
                    None
                ));
            }
        };

        Ok(CallToolResult::success(vec![Content::text(result)]))
    }
}
