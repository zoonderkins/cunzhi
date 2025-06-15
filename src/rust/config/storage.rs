use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, State, Manager, LogicalSize};

use super::settings::{AppConfig, AppState};

pub fn get_config_path(app: &AppHandle) -> Result<PathBuf> {
    let app_dir = app.path()
        .app_config_dir()
        .map_err(|e| anyhow::anyhow!("无法获取应用配置目录: {}", e))?;

    // 确保目录存在
    fs::create_dir_all(&app_dir)?;

    Ok(app_dir.join("config.json"))
}

pub async fn save_config(state: &State<'_, AppState>, app: &AppHandle) -> Result<()> {
    let config_path = get_config_path(app)?;
    
    // 确保目录存在
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    let config = state.config.lock().map_err(|e| anyhow::anyhow!("获取配置失败: {}", e))?;
    let config_json = serde_json::to_string_pretty(&*config)?;
    
    fs::write(config_path, config_json)?;
    
    Ok(())
}

pub async fn load_config(state: &State<'_, AppState>, app: &AppHandle) -> Result<()> {
    let config_path = get_config_path(app)?;

    if config_path.exists() {
        let config_json = fs::read_to_string(config_path)?;
        let config: AppConfig = serde_json::from_str(&config_json)?;

        let mut config_guard = state.config.lock()
            .map_err(|e| anyhow::anyhow!("获取配置锁失败: {}", e))?;
        *config_guard = config;
    }

    Ok(())
}

pub async fn load_config_and_apply_window_settings(state: &State<'_, AppState>, app: &AppHandle) -> Result<()> {
    // 先加载配置
    load_config(state, app).await?;

    // 然后应用窗口设置
    let (always_on_top, window_config) = {
        let config = state.config.lock()
            .map_err(|e| anyhow::anyhow!("获取配置失败: {}", e))?;
        (config.always_on_top, config.window_config.clone())
    };

    // 应用到窗口
    if let Some(window) = app.get_webview_window("main") {
        // 应用置顶设置
        let _ = window.set_always_on_top(always_on_top);

        // 应用窗口大小约束
        let _ = window.set_min_size(Some(LogicalSize::new(
            window_config.min_width,
            window_config.min_height,
        )));

        let _ = window.set_max_size(Some(LogicalSize::new(
            window_config.max_width,
            window_config.max_height,
        )));

        // 如果启用了自动调整大小，设置为合适的初始大小
        if window_config.auto_resize {
            let initial_width = window_config.min_width;
            let initial_height = (window_config.min_height + window_config.max_height) / 2.0;
            let _ = window.set_size(LogicalSize::new(initial_width, initial_height));
        }
    }

    Ok(())
}
