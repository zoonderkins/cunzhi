use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, LogicalSize, Manager, State};

use super::settings::{AppConfig, AppState, default_shortcuts};

pub fn get_config_path(_app: &AppHandle) -> Result<PathBuf> {
    // 使用与独立設定相同的路径，确保一致性
    get_standalone_config_path()
}

pub async fn save_config(state: &State<'_, AppState>, app: &AppHandle) -> Result<()> {
    let config_path = get_config_path(app)?;

    // 确保目录存在
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let config = state
        .config
        .lock()
        .map_err(|e| anyhow::anyhow!("獲取設定失敗: {}", e))?;
    let config_json = serde_json::to_string_pretty(&*config)?;

    // 寫入檔案
    fs::write(&config_path, config_json)?;

    // 強制重新整理檔案系統快取
    if let Ok(file) = std::fs::OpenOptions::new().write(true).open(&config_path) {
        let _ = file.sync_all();
    }

    log::debug!("設定已儲存到: {:?}", config_path);

    Ok(())
}

/// Tauri應用专用的設定載入函數
pub async fn load_config(state: &State<'_, AppState>, app: &AppHandle) -> Result<()> {
    let config_path = get_config_path(app)?;

    if config_path.exists() {
        let config_json = fs::read_to_string(&config_path)?;
        let mut config: AppConfig = serde_json::from_str(&config_json)?;

        // 合并預設快捷鍵設定，确保新的預設快捷键被新增
        merge_default_shortcuts(&mut config);

        let mut config_guard = state
            .config
            .lock()
            .map_err(|e| anyhow::anyhow!("獲取設定锁失敗: {}", e))?;
        *config_guard = config;
    }

    Ok(())
}

pub async fn load_config_and_apply_window_settings(
    state: &State<'_, AppState>,
    app: &AppHandle,
) -> Result<()> {
    // 先載入設定
    load_config(state, app).await?;

    // 然后應用視窗設定
    let (always_on_top, window_config) = {
        let config = state
            .config
            .lock()
            .map_err(|e| anyhow::anyhow!("獲取設定失敗: {}", e))?;
        (
            config.ui_config.always_on_top,
            config.ui_config.window_config.clone(),
        )
    };

    // 應用到視窗
    if let Some(window) = app.get_webview_window("main") {
        // 應用置頂設定
        if let Err(e) = window.set_always_on_top(always_on_top) {
            log::warn!("設定視窗置顶失敗: {}", e);
        } else {
            log::info!("視窗置顶狀態已設定为: {} (設定載入時)", always_on_top);
        }

        // 應用視窗大小约束
        if let Err(e) = window.set_min_size(Some(LogicalSize::new(
            window_config.min_width,
            window_config.min_height,
        ))) {
            log::warn!("設定最小視窗大小失敗: {}", e);
        }

        if let Err(e) = window.set_max_size(Some(LogicalSize::new(
            window_config.max_width,
            window_config.max_height,
        ))) {
            log::warn!("設定最大視窗大小失敗: {}", e);
        }

        // 根据目前模式設定視窗大小
        let (target_width, target_height) = if window_config.fixed {
            // 固定模式：使用固定尺寸
            (window_config.fixed_width, window_config.fixed_height)
        } else {
            // 自由拉伸模式：使用自由拉伸尺寸
            (window_config.free_width, window_config.free_height)
        };

        // 應用視窗大小（移除偵錯訊息）
        if let Err(_e) = window.set_size(LogicalSize::new(target_width, target_height)) {
            // 静默處理視窗大小設定失敗
        }
    }

    Ok(())
}

/// 独立載入設定檔案（用于MCP服务器等独立程序）
pub fn load_standalone_config() -> Result<AppConfig> {
    let config_path = get_standalone_config_path()?;

    if config_path.exists() {
        let config_json = fs::read_to_string(config_path)?;
        let mut config: AppConfig = serde_json::from_str(&config_json)?;

        // 合并預設快捷鍵設定
        merge_default_shortcuts(&mut config);

        Ok(config)
    } else {
        // 如果設定檔案不存在，傳回預設設定
        Ok(AppConfig::default())
    }
}

// Telegram 功能已移除

/// 獲取独立設定檔案路径（不依赖Tauri）
fn get_standalone_config_path() -> Result<PathBuf> {
    // 使用標準的設定目录
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("無法獲取設定目录"))?
        .join("cunzhi");

    // 确保目录存在
    fs::create_dir_all(&config_dir)?;

    Ok(config_dir.join("config.json"))
}

/// 合并預設快捷鍵設定，确保新的預設快捷键被新增到现有設定中
fn merge_default_shortcuts(config: &mut AppConfig) {
    let default_shortcuts = default_shortcuts();

    // 遍历所有預設快捷键
    for (key, default_binding) in default_shortcuts {
        if !config.shortcut_config.shortcuts.contains_key(&key) {
            // 如果用户設定中不存在，则新增
            config.shortcut_config.shortcuts.insert(key, default_binding);
        } else if key == "enhance" {
            // 特殊處理：更新增強快捷鍵的預設值从 Shift+Enter 到 Ctrl+Shift+Enter
            if let Some(existing_binding) = config.shortcut_config.shortcuts.get(&key) {
                // 檢查是否是旧的預設值 (Shift+Enter)
                if existing_binding.key_combination.key == "Enter"
                    && !existing_binding.key_combination.ctrl
                    && existing_binding.key_combination.shift
                    && !existing_binding.key_combination.alt
                    && !existing_binding.key_combination.meta {
                    // 更新为新的預設值 (Ctrl+Shift+Enter)
                    config.shortcut_config.shortcuts.insert(key, default_binding);
                }
            }
        }
    }
}
