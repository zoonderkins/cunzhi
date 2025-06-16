use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, State, Manager, LogicalSize};

use super::settings::{AppConfig, AppState};

pub fn get_config_path(_app: &AppHandle) -> Result<PathBuf> {
    // 使用与独立配置相同的路径，确保一致性
    get_standalone_config_path()
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

/// Tauri应用专用的配置加载函数
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
        (config.ui_config.always_on_top, config.ui_config.window_config.clone())
    };

    // 应用到窗口
    if let Some(window) = app.get_webview_window("main") {
        // 应用置顶设置
        if let Err(e) = window.set_always_on_top(always_on_top) {
            eprintln!("⚠️ 设置窗口置顶失败: {}", e);
        } else {
            println!("✅ 窗口置顶状态已设置为: {}", always_on_top);
        }

        // 应用窗口大小约束
        if let Err(e) = window.set_min_size(Some(LogicalSize::new(
            window_config.min_width,
            window_config.min_height,
        ))) {
            eprintln!("⚠️ 设置最小窗口大小失败: {}", e);
        }

        if let Err(e) = window.set_max_size(Some(LogicalSize::new(
            window_config.max_width,
            window_config.max_height,
        ))) {
            eprintln!("⚠️ 设置最大窗口大小失败: {}", e);
        }

        // 根据当前模式设置窗口大小
        let (target_width, target_height) = if window_config.fixed {
            // 固定模式：使用固定尺寸
            (window_config.fixed_width, window_config.fixed_height)
        } else {
            // 自由拉伸模式：使用自由拉伸尺寸
            (window_config.free_width, window_config.free_height)
        };

        // 应用窗口大小
        println!("🔍 应用窗口大小调试信息:");
        println!("   目标逻辑尺寸: {}x{}", target_width, target_height);

        if let Err(e) = window.set_size(LogicalSize::new(target_width, target_height)) {
            eprintln!("⚠️ 设置窗口大小失败: {}", e);
        } else {
            let mode = if window_config.fixed { "固定" } else { "自由拉伸" };
            println!("✅ 窗口大小已设置为{}模式: {}x{}", mode, target_width, target_height);

            // 验证设置是否生效
            if let Ok(actual_size) = window.inner_size() {
                if let Ok(scale_factor) = window.scale_factor() {
                    let actual_logical_width = actual_size.width as f64 / scale_factor;
                    let actual_logical_height = actual_size.height as f64 / scale_factor;
                    println!("   实际物理尺寸: {}x{}", actual_size.width, actual_size.height);
                    println!("   实际逻辑尺寸: {}x{}", actual_logical_width.round(), actual_logical_height.round());
                }
            }
        }
    }

    Ok(())
}

/// 独立加载配置文件（用于MCP服务器等独立进程）
pub fn load_standalone_config() -> Result<AppConfig> {
    let config_path = get_standalone_config_path()?;

    if config_path.exists() {
        let config_json = fs::read_to_string(config_path)?;
        let config: AppConfig = serde_json::from_str(&config_json)?;
        Ok(config)
    } else {
        // 如果配置文件不存在，返回默认配置
        Ok(AppConfig::default())
    }
}

/// 获取独立配置文件路径（不依赖Tauri）
fn get_standalone_config_path() -> Result<PathBuf> {
    // 使用标准的配置目录
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("无法获取配置目录"))?
        .join("cunzhi");

    // 确保目录存在
    fs::create_dir_all(&config_dir)?;

    Ok(config_dir.join("config.json"))
}
