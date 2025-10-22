use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use tauri::{AppHandle, Manager};
use rust_embed::RustEmbed;

/// 内嵌音效资源 - 自動包含整个sounds目录
#[derive(RustEmbed)]
#[folder = "src/rust/assets/resources/"]
struct EmbeddedAudio;

/// 音訊资源訊息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioAsset {
    pub id: String,
    pub name: String,
    pub filename: String,
}

/// 音訊资源管理器
pub struct AudioAssetManager {
    assets: HashMap<String, AudioAsset>,
}

impl AudioAssetManager {
    /// 建立新的音訊资源管理器
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
        }
    }

    /// 从應用中載入音訊资源
    pub fn load_from_app(&mut self, app: &AppHandle) -> Result<()> {
        // 优先載入内嵌音訊资源
        if let Ok(()) = self.load_embedded_audio() {
            eprintln!("✅ 从内嵌资源載入了 {} 个音訊资源", self.assets.len());
            return Ok(());
        }

        // 回退到檔案系統載入（开发环境）
        eprintln!("⚠️ 内嵌资源載入失敗，尝试从开发环境檔案系統載入...");
        self.scan_audio_directory(app)
    }

    /// 載入内嵌音訊资源
    fn load_embedded_audio(&mut self) -> Result<()> {
        // 动态扫描所有内嵌的音訊檔案
        for file_path in EmbeddedAudio::iter() {
            let filename = file_path.as_ref();
            if self.is_audio_file(filename) {
                let asset = self.create_asset_from_filename(filename);
                self.assets.insert(asset.id.clone(), asset);
            }
        }

        if self.assets.is_empty() {
            return Err(anyhow::anyhow!("没有找到任何内嵌音訊资源。请确保 src/rust/assets/resources/ 目录中包含音訊檔案。"));
        }

        Ok(())
    }



    /// 扫描音訊目录（仅开发环境使用）
    fn scan_audio_directory(&mut self, _app: &AppHandle) -> Result<()> {
        // 仅在开发环境扫描檔案系統
        let sounds_dir = std::env::current_dir()
            .unwrap_or_default()
            .join("src/rust/assets/resources");

        if !sounds_dir.exists() {
            return Err(anyhow::anyhow!("开发环境音訊目录不存在: {:?}", sounds_dir));
        }

        let entries = fs::read_dir(&sounds_dir)
            .map_err(|e| anyhow::anyhow!("讀取音訊目录失敗: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| anyhow::anyhow!("讀取目录项失敗: {}", e))?;
            let path = entry.path();

            if path.is_file() {
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    if self.is_audio_file(filename) {
                        let asset = self.create_asset_from_filename(filename);
                        self.assets.insert(asset.id.clone(), asset);
                    }
                }
            }
        }

        eprintln!("✅ 扫描音訊目录，发现 {} 个音訊檔案", self.assets.len());
        Ok(())
    }

    /// 判断是否为音訊檔案
    fn is_audio_file(&self, filename: &str) -> bool {
        let audio_extensions = ["mp3", "wav", "ogg", "m4a", "aac", "flac"];
        if let Some(ext) = filename.split('.').next_back() {
            audio_extensions.contains(&ext.to_lowercase().as_str())
        } else {
            false
        }
    }

    /// 从檔案名建立音訊资源
    /// 支持格式：
    /// - filename.mp3 -> ID: filename, 名称: filename
    /// - filename[友好名称].mp3 -> ID: filename, 名称: 友好名称
    /// - filename[].mp3 -> ID: filename, 名称: filename (空方括号時使用檔案名)
    fn create_asset_from_filename(&self, filename: &str) -> AudioAsset {
        let name_without_ext = filename.split('.').next().unwrap_or(filename);

        // 解析檔案名约定：filename[友好名称]
        let (id, name) = if let Some(bracket_start) = name_without_ext.find('[') {
            if let Some(bracket_end) = name_without_ext.find(']') {
                let base_name = name_without_ext[..bracket_start].trim();
                let friendly_name = name_without_ext[bracket_start + 1..bracket_end].trim();

                // 如果base_name为空，使用整个檔案名
                let id = if base_name.is_empty() {
                    name_without_ext.to_lowercase().replace(' ', "_").replace(['[', ']'], "")
                } else {
                    base_name.to_lowercase().replace(' ', "_")
                };

                // 如果友好名称为空，使用base_name或檔案名
                let display_name = if friendly_name.is_empty() {
                    if base_name.is_empty() {
                        name_without_ext.replace(['[', ']'], "")
                    } else {
                        base_name.to_string()
                    }
                } else {
                    friendly_name.to_string()
                };

                (id, display_name)
            } else {
                // 只有左方括号，格式錯誤，使用整个名称
                log::warn!("音訊檔案名格式錯誤（缺少右方括号）: {}", filename);
                let id = name_without_ext.to_lowercase().replace(' ', "_").replace('[', "");
                (id, name_without_ext.replace('[', ""))
            }
        } else {
            // 没有方括号，使用檔案名作为ID和名称
            let id = name_without_ext.to_lowercase().replace(' ', "_");
            (id, name_without_ext.to_string())
        };

        AudioAsset {
            id,
            name,
            filename: filename.to_string(),
        }
    }

    /// 獲取所有音訊资源
    pub fn get_all_assets(&self) -> Vec<&AudioAsset> {
        self.assets.values().collect()
    }

    /// 根据ID獲取音訊资源
    pub fn get_asset_by_id(&self, id: &str) -> Option<&AudioAsset> {
        self.assets.get(id)
    }

    /// 獲取音訊檔案的完整路径（已废弃，使用ensure_audio_exists代替）
    pub fn get_audio_path(&self, app: &AppHandle, asset_id: &str) -> Result<PathBuf> {
        // 直接呼叫ensure_audio_exists，它会處理内嵌资源
        self.ensure_audio_exists(app, asset_id)
    }

    /// 确保音訊檔案存在，如果不存在则从内嵌资源或资源目录複製到用户設定目录
    pub fn ensure_audio_exists(&self, app: &AppHandle, asset_id: &str) -> Result<PathBuf> {
        let asset = self.get_asset_by_id(asset_id)
            .ok_or_else(|| anyhow::anyhow!("未找到音訊资源: {}", asset_id))?;

        let config_dir = app.path().app_config_dir()
            .map_err(|e| anyhow::anyhow!("无法獲取應用設定目录: {}", e))?;

        let sounds_dir = config_dir.join("sounds");
        let target_path = sounds_dir.join(&asset.filename);

        // 如果檔案已存在，直接傳回
        if target_path.exists() {
            return Ok(target_path);
        }

        // 建立sounds目录
        std::fs::create_dir_all(&sounds_dir)
            .map_err(|e| anyhow::anyhow!("建立sounds目录失敗: {}", e))?;

        // 优先从内嵌资源複製
        if let Some(embedded_file) = EmbeddedAudio::get(&asset.filename) {
            std::fs::write(&target_path, embedded_file.data.as_ref())
                .map_err(|e| anyhow::anyhow!("寫入内嵌音訊檔案失敗: {}", e))?;

            return Ok(target_path);
        }

        // 回退到檔案系統複製（仅开发环境）
        let dev_source_path = std::env::current_dir()
            .unwrap_or_default()
            .join("src/rust/assets/resources")
            .join(&asset.filename);

        if dev_source_path.exists() {
            std::fs::copy(&dev_source_path, &target_path)
                .map_err(|e| anyhow::anyhow!("複製音訊檔案失敗: {}", e))?;

            return Ok(target_path);
        }

        // 如果内嵌资源和檔案系統都没有找到，傳回錯誤
        Err(anyhow::anyhow!(
            "音訊资源 '{}' 不存在。请确保音訊檔案已正确内嵌到二进制中，或在开发环境中存在于 src/rust/assets/resources/ 目录。",
            asset_id
        ))
    }



    /// 解析音訊URL，支持资源ID、檔案路径和網路URL
    pub fn parse_audio_url(&self, _app: &AppHandle, audio_url: &str) -> Result<AudioSource> {
        if audio_url.is_empty() {
            // 如果为空，使用第一个可用的音訊资源作为預設
            let all_assets = self.get_all_assets();
            if let Some(first_asset) = all_assets.first() {
                return Ok(AudioSource::Asset(first_asset.id.clone()));
            } else {
                return Err(anyhow::anyhow!("没有可用的音訊资源"));
            }
        }

        // 檢查是否是網路URL
        if audio_url.starts_with("http://") || audio_url.starts_with("https://") {
            return Ok(AudioSource::Url(audio_url.to_string()));
        }

        // 檢查是否是资源ID
        if self.assets.contains_key(audio_url) {
            return Ok(AudioSource::Asset(audio_url.to_string()));
        }

        // 尝试通过檔案名匹配（去掉擴充名）
        let filename_without_ext = audio_url.split('.').next().unwrap_or(audio_url);
        if self.assets.contains_key(filename_without_ext) {
            return Ok(AudioSource::Asset(filename_without_ext.to_string()));
        }

        // 檢查是否是檔案路径
        let path = PathBuf::from(audio_url);
        if path.exists() {
            return Ok(AudioSource::File(path));
        }

        // 預設傳回檔案路径（可能是相对路径）
        Ok(AudioSource::File(path))
    }
}

/// 音訊源類型
#[derive(Debug, Clone)]
pub enum AudioSource {
    Asset(String),      // 内置资源ID
    File(PathBuf),      // 本地檔案路径
    Url(String),        // 網路URL
}

impl Default for AudioAssetManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 全局音訊资源管理器实例
static AUDIO_ASSET_MANAGER: std::sync::OnceLock<std::sync::Mutex<AudioAssetManager>> = std::sync::OnceLock::new();

/// 獲取全局音訊资源管理器
pub fn get_audio_asset_manager() -> &'static std::sync::Mutex<AudioAssetManager> {
    AUDIO_ASSET_MANAGER.get_or_init(|| std::sync::Mutex::new(AudioAssetManager::new()))
}

/// 初始化音訊資源管理器
pub fn initialize_audio_asset_manager(app: &AppHandle) -> Result<()> {
    let manager = get_audio_asset_manager();
    let mut manager = manager.lock().map_err(|e| anyhow::anyhow!("獲取管理器锁失敗: {}", e))?;
    manager.load_from_app(app)
}

/// 獲取所有可用的音訊资源（用于前端）
#[tauri::command]
pub async fn get_available_audio_assets() -> Result<Vec<AudioAsset>, String> {
    let manager = get_audio_asset_manager();
    let manager = manager.lock().map_err(|e| format!("獲取管理器锁失敗: {}", e))?;
    let all_assets = manager.get_all_assets();
    Ok(all_assets.into_iter().cloned().collect())
}

/// 重新整理音訊资源（重新扫描目录）
#[tauri::command]
pub async fn refresh_audio_assets(app: tauri::AppHandle) -> Result<Vec<AudioAsset>, String> {
    initialize_audio_asset_manager(&app).map_err(|e| format!("重新整理音訊资源失敗: {}", e))?;
    get_available_audio_assets().await
}
