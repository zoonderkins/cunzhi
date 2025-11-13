use anyhow::Result;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{AppHandle, State, Manager};
use rodio::{Decoder, OutputStream, Sink};

use crate::config::{AppState, save_config};
use crate::log_important;
use super::audio_assets::{get_audio_asset_manager, AudioSource};

// 音訊播放控制器 - 只存储控制信号，不存储音訊流
pub struct AudioController {
    pub should_stop: Arc<AtomicBool>,
}

#[tauri::command]
pub async fn get_audio_notification_enabled(state: State<'_, AppState>) -> Result<bool, String> {
    let config = state.config.lock().map_err(|e| format!("獲取設定失敗: {}", e))?;
    Ok(config.audio_config.notification_enabled)
}

#[tauri::command]
pub async fn set_audio_notification_enabled(enabled: bool, state: State<'_, AppState>, app: tauri::AppHandle) -> Result<(), String> {
    // 如果是首次啟用音訊通知，先複製音訊檔案
    if enabled {
        if let Err(e) = ensure_audio_file_exists(&app).await {
            return Err(format!("准备音訊檔案失敗: {}", e));
        }
    }

    {
        let mut config = state.config.lock().map_err(|e| format!("獲取設定失敗: {}", e))?;
        config.audio_config.notification_enabled = enabled;
    }

    // 儲存設定到檔案
    save_config(&state, &app).await.map_err(|e| format!("儲存設定失敗: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn get_audio_url(state: State<'_, AppState>) -> Result<String, String> {
    let config = state.config.lock().map_err(|e| format!("獲取設定失敗: {}", e))?;
    Ok(config.audio_config.custom_url.clone())
}

#[tauri::command]
pub async fn set_audio_url(url: String, state: State<'_, AppState>, app: tauri::AppHandle) -> Result<(), String> {
    {
        let mut config = state.config.lock().map_err(|e| format!("獲取設定失敗: {}", e))?;
        config.audio_config.custom_url = url;
    }

    // 儲存設定到檔案
    save_config(&state, &app).await.map_err(|e| format!("儲存設定失敗: {}", e))?;
    Ok(())
}

#[tauri::command]
pub async fn play_notification_sound(state: State<'_, AppState>, app: tauri::AppHandle) -> Result<(), String> {
    // 檢查是否啟用音訊通知
    let (enabled, audio_url) = {
        let config = state.config.lock().map_err(|e| format!("獲取設定失敗: {}", e))?;
        (config.audio_config.notification_enabled, config.audio_config.custom_url.clone())
    };

    if !enabled {
        return Ok(());
    }

    // 非同步播放音訊，避免阻塞主執行緒
    tokio::spawn(async move {
        if let Err(e) = play_audio_file(&app, &audio_url).await {
            log_important!(warn, "播放音訊失敗: {}", e);
        }
    });

    Ok(())
}

#[tauri::command]
pub async fn test_audio_sound(state: State<'_, AppState>, app: tauri::AppHandle) -> Result<(), String> {
    // 獲取當前設定的音效URL
    let audio_url = {
        let config = state.config.lock().map_err(|e| format!("獲取設定失敗: {}", e))?;
        config.audio_config.custom_url.clone()
    };

    // 同步測試音訊播放，确保能捕获錯誤
    match play_audio_file(&app, &audio_url).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("音效測試失敗: {}", e))
    }
}

#[tauri::command]
pub async fn stop_audio_sound(app: tauri::AppHandle) -> Result<(), String> {
    // 設定停止信号
    if let Some(audio_controller) = app.try_state::<AudioController>() {
        audio_controller.should_stop.store(true, Ordering::Relaxed);
    }
    Ok(())
}

pub async fn play_audio_file(app: &AppHandle, audio_url: &str) -> Result<()> {
    // 重置停止信号
    if let Some(audio_controller) = app.try_state::<AudioController>() {
        audio_controller.should_stop.store(false, Ordering::Relaxed);
    }

    let audio_source = {
        let manager = get_audio_asset_manager();
        let manager = manager.lock().map_err(|e| anyhow::anyhow!("獲取管理器锁失敗: {}", e))?;
        manager.parse_audio_url(app, audio_url)?
    };

    match audio_source {
        AudioSource::Url(url) => {
            // 網路音訊檔案
            play_audio_from_url(app, &url).await
        }
        AudioSource::File(path) => {
            // 本地檔案路径
            if path.exists() {
                let app_handle = app.clone();
                tokio::task::spawn_blocking(move || {
                    play_audio_sync_with_controller(&path, &app_handle)
                }).await
                .map_err(|e| anyhow::anyhow!("音訊播放任务失敗: {}", e))?
            } else {
                Err(anyhow::anyhow!("音訊檔案不存在: {:?}", path))
            }
        }
        AudioSource::Asset(asset_id) => {
            // 内置音訊资源
            let audio_path = {
                let manager = get_audio_asset_manager();
                let manager = manager.lock().map_err(|e| anyhow::anyhow!("獲取管理器锁失敗: {}", e))?;
                manager.ensure_audio_exists(app, &asset_id)?
            };
            let app_handle = app.clone();
            tokio::task::spawn_blocking(move || {
                play_audio_sync_with_controller(&audio_path, &app_handle)
            }).await
            .map_err(|e| anyhow::anyhow!("音訊播放任务失敗: {}", e))?
        }
    }
}

async fn play_audio_from_url(app: &AppHandle, url: &str) -> Result<()> {
    // 下载音訊檔案到临時目录
    let response = reqwest::get(url).await
        .map_err(|e| anyhow::anyhow!("下载音訊檔案失敗: {}", e))?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("下载音訊檔案失敗，狀態码: {}", response.status()));
    }

    let bytes = response.bytes().await
        .map_err(|e| anyhow::anyhow!("讀取音訊資料失敗: {}", e))?;

    // 将bytes转换为Vec<u8>以便移动到執行緒中
    let bytes_vec = bytes.to_vec();
    let app_handle = app.clone();

    // 使用 tokio::task::spawn_blocking 来等待音訊播放完成
    tokio::task::spawn_blocking(move || {
        play_audio_from_bytes_with_controller(bytes_vec, &app_handle)
    }).await
    .map_err(|e| anyhow::anyhow!("音訊播放任务失敗: {}", e))?
}

fn play_audio_from_bytes_with_controller(bytes: Vec<u8>, app: &AppHandle) -> Result<()> {
    // 建立音訊輸出流
    let (_stream, stream_handle) = OutputStream::try_default()
        .map_err(|e| anyhow::anyhow!("建立音訊輸出流失敗: {}", e))?;

    // 建立音訊播放器
    let sink = Sink::try_new(&stream_handle)
        .map_err(|e| anyhow::anyhow!("建立音訊播放器失敗: {}", e))?;

    // 从字節資料解码音訊
    let cursor = std::io::Cursor::new(bytes);
    let source = Decoder::new(cursor)
        .map_err(|e| anyhow::anyhow!("解码音訊資料失敗: {}", e))?;

    // 播放音訊
    sink.append(source);

    // 檢查停止信号并播放
    if let Some(audio_controller) = app.try_state::<AudioController>() {
        while !sink.empty() {
            if audio_controller.should_stop.load(Ordering::Relaxed) {
                sink.stop();
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    } else {
        sink.sleep_until_end();
    }

    Ok(())
}



fn play_audio_sync_with_controller(audio_path: &PathBuf, app: &AppHandle) -> Result<()> {
    // 建立音訊輸出流
    let (_stream, stream_handle) = OutputStream::try_default()
        .map_err(|e| anyhow::anyhow!("建立音訊輸出流失敗: {}", e))?;

    // 建立音訊播放器
    let sink = Sink::try_new(&stream_handle)
        .map_err(|e| anyhow::anyhow!("建立音訊播放器失敗: {}", e))?;

    // 讀取音訊檔案
    let file = std::fs::File::open(audio_path)
        .map_err(|e| anyhow::anyhow!("開啟音訊檔案失敗: {}", e))?;
    let buf_reader = BufReader::new(file);

    // 解码音訊
    let source = Decoder::new(buf_reader)
        .map_err(|e| anyhow::anyhow!("解码音訊檔案失敗: {}", e))?;

    // 播放音訊
    sink.append(source);

    // 檢查停止信号并播放
    if let Some(audio_controller) = app.try_state::<AudioController>() {
        while !sink.empty() {
            if audio_controller.should_stop.load(Ordering::Relaxed) {
                sink.stop();
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    } else {
        // 如果没有控制器，使用原来的方式
        sink.sleep_until_end();
    }

    Ok(())
}



/// 确保預設音訊檔案存在，如果不存在则从资源目录複製
pub async fn ensure_audio_file_exists(app: &AppHandle) -> Result<()> {
    let manager = get_audio_asset_manager();
    let manager = manager.lock().map_err(|e| anyhow::anyhow!("獲取管理器锁失敗: {}", e))?;

    // 确保第一个可用的音訊资源存在
    let all_assets = manager.get_all_assets();
    if let Some(first_asset) = all_assets.first() {
        manager.ensure_audio_exists(app, &first_asset.id)?;
    } else {
        return Err(anyhow::anyhow!("没有可用的音訊资源"));
    }

    Ok(())
}


