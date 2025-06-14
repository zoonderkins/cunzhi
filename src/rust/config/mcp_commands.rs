use std::collections::HashMap;
use tauri::{AppHandle, State};
use serde::{Deserialize, Serialize};

use crate::config::{AppState, save_config};

/// MCP工具配置结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct McpToolConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub can_disable: bool,
    pub icon: String,
    pub icon_bg: String,
    pub dark_icon_bg: String,
}

/// 获取MCP工具配置
#[tauri::command]
pub async fn get_mcp_tools_config(state: State<'_, AppState>) -> Result<Vec<McpToolConfig>, String> {
    let config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
    
    // 定义默认工具配置
    let default_tools = vec![
        McpToolConfig {
            id: "zhi".to_string(),
            name: "Zhi 智能审查工具".to_string(),
            description: "智能代码审查交互工具（寸止）".to_string(),
            enabled: config.mcp_tools.get("zhi").copied().unwrap_or(true),
            can_disable: false,
            icon: "i-carbon-chat".to_string(),
            icon_bg: "bg-blue-100".to_string(),
            dark_icon_bg: "dark:bg-blue-900".to_string(),
        },
        McpToolConfig {
            id: "memory".to_string(),
            name: "记忆管理工具".to_string(),
            description: "智能记忆存储和检索系统".to_string(),
            enabled: config.mcp_tools.get("memory").copied().unwrap_or(true),
            can_disable: true,
            icon: "i-carbon-data-base".to_string(),
            icon_bg: "bg-purple-100".to_string(),
            dark_icon_bg: "dark:bg-purple-900".to_string(),
        },
    ];
    
    Ok(default_tools)
}

/// 设置MCP工具状态
#[tauri::command]
pub async fn set_mcp_tool_enabled(
    tool_id: String,
    enabled: bool,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<bool, String> {
    {
        let mut config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
        
        // 检查工具是否可以禁用
        if tool_id == "zhi" && !enabled {
            return Err("寸止工具是必需的，无法禁用".to_string());
        }
        
        // 更新工具状态
        config.mcp_tools.insert(tool_id.clone(), enabled);
    }
    
    // 保存配置
    save_config(&state, &app).await
        .map_err(|e| format!("保存配置失败: {}", e))?;
    
    println!("✅ MCP工具 {} 状态已更新为: {}", tool_id, enabled);
    
    // 返回是否需要重启
    Ok(true)
}

/// 获取所有MCP工具状态
#[tauri::command]
pub async fn get_mcp_tools_status(state: State<'_, AppState>) -> Result<HashMap<String, bool>, String> {
    let config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
    Ok(config.mcp_tools.clone())
}

/// 重置MCP工具配置为默认值
#[tauri::command]
pub async fn reset_mcp_tools_config(
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<(), String> {
    {
        let mut config = state.config.lock().map_err(|e| format!("获取配置失败: {}", e))?;
        config.mcp_tools = crate::config::default_mcp_tools();
    }
    
    // 保存配置
    save_config(&state, &app).await
        .map_err(|e| format!("保存配置失败: {}", e))?;
    
    println!("✅ MCP工具配置已重置为默认值");
    Ok(())
}

/// 重启应用
#[tauri::command]
pub async fn restart_application(app: AppHandle) -> Result<(), String> {
    println!("🔄 正在重启应用...");
    
    // 获取当前可执行文件路径
    let current_exe = std::env::current_exe()
        .map_err(|e| format!("获取当前可执行文件路径失败: {}", e))?;
    
    // 启动新进程
    std::process::Command::new(&current_exe)
        .spawn()
        .map_err(|e| format!("启动新进程失败: {}", e))?;
    
    // 延迟一下再退出，确保新进程启动
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    // 退出当前进程
    app.exit(0);
    
    Ok(())
}
