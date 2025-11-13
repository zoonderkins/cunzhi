use std::collections::HashMap;
use tauri::{AppHandle, State};
// use serde::{Deserialize, Serialize};

use crate::config::{AppState, save_config};
use crate::constants::mcp;

/// MCP工具設定
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct MCPToolConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub can_disable: bool,
    pub icon: String,
    pub icon_bg: String,
    pub dark_icon_bg: String,
}

/// 獲取MCP工具設定列表
#[tauri::command]
pub async fn get_mcp_tools_config(state: State<'_, AppState>) -> Result<Vec<MCPToolConfig>, String> {
    let config = state.config.lock().map_err(|e| format!("獲取設定失敗: {}", e))?;
    
    let mut tools = vec![
        MCPToolConfig {
            id: mcp::TOOL_ZHI.to_string(),
            name: "寸止".to_string(),
            description: "智慧程式碼審查互動工具，支援預定義選項、自由文字輸入和圖片上傳".to_string(),
            enabled: config.mcp_config.tools.get(mcp::TOOL_ZHI).copied().unwrap_or(true),
            can_disable: false, // 寸止工具是必需的
            icon: "i-carbon-chat".to_string(),
            icon_bg: "bg-blue-100 dark:bg-blue-900".to_string(),
            dark_icon_bg: "dark:bg-blue-800".to_string(),
        },
        MCPToolConfig {
            id: mcp::TOOL_JI.to_string(),
            name: "記憶管理".to_string(),
            description: "全域記憶管理工具，用於儲存和管理重要的開發規範、使用者偏好和最佳實務".to_string(),
            enabled: config.mcp_config.tools.get(mcp::TOOL_JI).copied().unwrap_or(true),
            can_disable: true,
            icon: "i-carbon-data-base".to_string(),
            icon_bg: "bg-green-100 dark:bg-green-900".to_string(),
            dark_icon_bg: "dark:bg-green-800".to_string(),
        },
    ];
    
    // 按啟用狀態排序，啟用的在前
    tools.sort_by(|a, b| b.enabled.cmp(&a.enabled));
    
    Ok(tools)
}

/// 設定MCP工具啟用狀態
#[tauri::command]
pub async fn set_mcp_tool_enabled(
    tool_id: String,
    enabled: bool,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<(), String> {
    {
        let mut config = state.config.lock().map_err(|e| format!("獲取設定失敗: {}", e))?;
        
        // 檢查工具是否可以禁用
        if tool_id == mcp::TOOL_ZHI && !enabled {
            return Err("寸止工具是必需的，無法禁用".to_string());
        }
        
        // 更新工具狀態
        config.mcp_config.tools.insert(tool_id.clone(), enabled);
    }
    
    // 儲存設定
    save_config(&state, &app).await
        .map_err(|e| format!("儲存設定失敗: {}", e))?;

    // 使用日誌記錄狀態變更（在 MCP 模式下会自動輸出到檔案）
    log::info!("MCP工具 {} 狀態已更新为: {}", tool_id, enabled);

    Ok(())
}

/// 獲取所有MCP工具狀態
#[tauri::command]
pub async fn get_mcp_tools_status(state: State<'_, AppState>) -> Result<HashMap<String, bool>, String> {
    let config = state.config.lock().map_err(|e| format!("獲取設定失敗: {}", e))?;
    Ok(config.mcp_config.tools.clone())
}

/// 重置MCP工具設定为預設值
#[tauri::command]
pub async fn reset_mcp_tools_config(
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<(), String> {
    {
        let mut config = state.config.lock().map_err(|e| format!("獲取設定失敗: {}", e))?;
        let default_config = mcp::get_default_mcp_config();
        config.mcp_config.tools.clear();
        for tool in &default_config.tools {
            config.mcp_config.tools.insert(tool.tool_id.clone(), tool.enabled);
        }
    }
    
    // 儲存設定
    save_config(&state, &app).await
        .map_err(|e| format!("儲存設定失敗: {}", e))?;

    // 使用日誌記錄設定重置（在 MCP 模式下会自動輸出到檔案）
    log::info!("MCP工具設定已重置为預設值");
    Ok(())
}
