use tauri::{AppHandle, Emitter};
use tauri_plugin_updater::UpdaterExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateInfo {
    pub available: bool,
    pub current_version: String,
    pub latest_version: String,
    pub release_notes: String,
    pub download_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateProgress {
    pub chunk_length: usize,
    pub content_length: Option<u64>,
    pub downloaded: u64,
    pub percentage: f64,
}

/// 检查是否有可用更新
#[tauri::command]
pub async fn check_for_updates(app: AppHandle) -> Result<UpdateInfo, String> {
    let updater = app.updater_builder().build()
        .map_err(|e| format!("创建更新器失败: {}", e))?;

    match updater.check().await {
        Ok(Some(update)) => {
            let current_version = app.package_info().version.to_string();
            let latest_version = update.version.clone();
            
            Ok(UpdateInfo {
                available: true,
                current_version,
                latest_version,
                release_notes: update.body.unwrap_or_default(),
                download_url: update.download_url.to_string(),
            })
        }
        Ok(None) => {
            let current_version = app.package_info().version.to_string();
            Ok(UpdateInfo {
                available: false,
                current_version: current_version.clone(),
                latest_version: current_version,
                release_notes: String::new(),
                download_url: String::new(),
            })
        }
        Err(e) => Err(format!("检查更新失败: {}", e)),
    }
}

/// 下载并安装更新
#[tauri::command]
pub async fn download_and_install_update(app: AppHandle) -> Result<(), String> {
    let updater = app.updater_builder().build()
        .map_err(|e| format!("创建更新器失败: {}", e))?;

    match updater.check().await {
        Ok(Some(update)) => {
            // 发送下载开始事件
            let _ = app.emit("update_download_started", ());

            // 下载更新
            let mut downloaded = 0u64;

            update
                .download_and_install(
                    |chunk_length, content_length| {
                        downloaded += chunk_length as u64;
                        let percentage = if let Some(total) = content_length {
                            (downloaded as f64 / total as f64) * 100.0
                        } else {
                            0.0
                        };

                        let progress = UpdateProgress {
                            chunk_length,
                            content_length,
                            downloaded,
                            percentage,
                        };

                        // 发送下载进度事件
                        let _ = app.emit("update_download_progress", &progress);
                    },
                    || {
                        // 发送安装开始事件
                        let _ = app.emit("update_install_started", ());
                    },
                )
                .await
                .map_err(|e| format!("下载或安装更新失败: {}", e))?;

            // 发送安装完成事件
            let _ = app.emit("update_install_finished", ());

            Ok(())
        }
        Ok(None) => Err("没有可用的更新".to_string()),
        Err(e) => Err(format!("检查更新失败: {}", e)),
    }
}

/// 获取当前应用版本
#[tauri::command]
pub async fn get_current_version(app: AppHandle) -> Result<String, String> {
    Ok(app.package_info().version.to_string())
}

/// 重启应用以完成更新
#[tauri::command]
pub async fn restart_app(app: AppHandle) -> Result<(), String> {
    app.restart();
}
