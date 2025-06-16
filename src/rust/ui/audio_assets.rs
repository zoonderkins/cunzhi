use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use tauri::{AppHandle, Manager};
use rust_embed::RustEmbed;

/// 内嵌音效资源 - 自动包含整个sounds目录
#[derive(RustEmbed)]
#[folder = "src/rust/assets/resources/"]
struct EmbeddedAudio;

/// 音频资源信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioAsset {
    pub id: String,
    pub name: String,
    pub filename: String,
}

/// 音频资源管理器
pub struct AudioAssetManager {
    assets: HashMap<String, AudioAsset>,
}

impl AudioAssetManager {
    /// 创建新的音频资源管理器
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
        }
    }

    /// 从应用中加载音频资源
    pub fn load_from_app(&mut self, app: &AppHandle) -> Result<()> {
        // 优先加载内嵌音频资源
        if let Ok(()) = self.load_embedded_audio() {
            eprintln!("✅ 从内嵌资源加载了 {} 个音频资源", self.assets.len());
            return Ok(());
        }

        // 回退到文件系统加载（开发环境）
        eprintln!("⚠️ 内嵌资源加载失败，尝试从开发环境文件系统加载...");
        self.scan_audio_directory(app)
    }

    /// 加载内嵌音频资源
    fn load_embedded_audio(&mut self) -> Result<()> {
        eprintln!("📁 使用文件名约定自动扫描内嵌音频文件...");

        // 动态扫描所有内嵌的音频文件
        for file_path in EmbeddedAudio::iter() {
            let filename = file_path.as_ref();
            if self.is_audio_file(filename) {
                let asset = self.create_asset_from_filename(filename);
                eprintln!("  ✅ 发现音频文件: {} -> ID: {}, 名称: {}", filename, asset.id, asset.name);
                self.assets.insert(asset.id.clone(), asset);
            }
        }

        if self.assets.is_empty() {
            return Err(anyhow::anyhow!("没有找到任何内嵌音频资源。请确保 src/rust/assets/resources/ 目录中包含音频文件。"));
        }

        Ok(())
    }



    /// 扫描音频目录（仅开发环境使用）
    fn scan_audio_directory(&mut self, _app: &AppHandle) -> Result<()> {
        // 仅在开发环境扫描文件系统
        let sounds_dir = std::env::current_dir()
            .unwrap_or_default()
            .join("src/rust/assets/resources");

        if !sounds_dir.exists() {
            return Err(anyhow::anyhow!("开发环境音频目录不存在: {:?}", sounds_dir));
        }

        let entries = fs::read_dir(&sounds_dir)
            .map_err(|e| anyhow::anyhow!("读取音频目录失败: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| anyhow::anyhow!("读取目录项失败: {}", e))?;
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

        eprintln!("✅ 扫描音频目录，发现 {} 个音频文件", self.assets.len());
        Ok(())
    }

    /// 判断是否为音频文件
    fn is_audio_file(&self, filename: &str) -> bool {
        let audio_extensions = ["mp3", "wav", "ogg", "m4a", "aac", "flac"];
        if let Some(ext) = filename.split('.').last() {
            audio_extensions.contains(&ext.to_lowercase().as_str())
        } else {
            false
        }
    }

    /// 从文件名创建音频资源
    /// 支持格式：
    /// - filename.mp3 -> ID: filename, 名称: filename
    /// - filename[友好名称].mp3 -> ID: filename, 名称: 友好名称
    /// - filename[].mp3 -> ID: filename, 名称: filename (空方括号时使用文件名)
    fn create_asset_from_filename(&self, filename: &str) -> AudioAsset {
        let name_without_ext = filename.split('.').next().unwrap_or(filename);

        // 解析文件名约定：filename[友好名称]
        let (id, name) = if let Some(bracket_start) = name_without_ext.find('[') {
            if let Some(bracket_end) = name_without_ext.find(']') {
                let base_name = name_without_ext[..bracket_start].trim();
                let friendly_name = name_without_ext[bracket_start + 1..bracket_end].trim();

                // 如果base_name为空，使用整个文件名
                let id = if base_name.is_empty() {
                    name_without_ext.to_lowercase().replace(' ', "_").replace(['[', ']'], "")
                } else {
                    base_name.to_lowercase().replace(' ', "_")
                };

                // 如果友好名称为空，使用base_name或文件名
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
                // 只有左方括号，格式错误，使用整个名称
                eprintln!("⚠️ 音频文件名格式错误（缺少右方括号）: {}", filename);
                let id = name_without_ext.to_lowercase().replace(' ', "_").replace('[', "");
                (id, name_without_ext.replace('[', ""))
            }
        } else {
            // 没有方括号，使用文件名作为ID和名称
            let id = name_without_ext.to_lowercase().replace(' ', "_");
            (id, name_without_ext.to_string())
        };

        AudioAsset {
            id,
            name,
            filename: filename.to_string(),
        }
    }

    /// 获取所有音频资源
    pub fn get_all_assets(&self) -> Vec<&AudioAsset> {
        self.assets.values().collect()
    }

    /// 根据ID获取音频资源
    pub fn get_asset_by_id(&self, id: &str) -> Option<&AudioAsset> {
        self.assets.get(id)
    }

    /// 获取音频文件的完整路径（已废弃，使用ensure_audio_exists代替）
    pub fn get_audio_path(&self, app: &AppHandle, asset_id: &str) -> Result<PathBuf> {
        // 直接调用ensure_audio_exists，它会处理内嵌资源
        self.ensure_audio_exists(app, asset_id)
    }

    /// 确保音频文件存在，如果不存在则从内嵌资源或资源目录复制到用户配置目录
    pub fn ensure_audio_exists(&self, app: &AppHandle, asset_id: &str) -> Result<PathBuf> {
        let asset = self.get_asset_by_id(asset_id)
            .ok_or_else(|| anyhow::anyhow!("未找到音频资源: {}", asset_id))?;

        let config_dir = app.path().app_config_dir()
            .map_err(|e| anyhow::anyhow!("无法获取应用配置目录: {}", e))?;

        let sounds_dir = config_dir.join("sounds");
        let target_path = sounds_dir.join(&asset.filename);

        // 如果文件已存在，直接返回
        if target_path.exists() {
            return Ok(target_path);
        }

        // 创建sounds目录
        std::fs::create_dir_all(&sounds_dir)
            .map_err(|e| anyhow::anyhow!("创建sounds目录失败: {}", e))?;

        // 优先从内嵌资源复制
        if let Some(embedded_file) = EmbeddedAudio::get(&asset.filename) {
            std::fs::write(&target_path, embedded_file.data.as_ref())
                .map_err(|e| anyhow::anyhow!("写入内嵌音频文件失败: {}", e))?;

            eprintln!("✅ 音频文件已从内嵌资源复制: {} -> {:?}", asset.name, target_path);
            return Ok(target_path);
        }

        // 回退到文件系统复制（仅开发环境）
        let dev_source_path = std::env::current_dir()
            .unwrap_or_default()
            .join("src/rust/assets/resources")
            .join(&asset.filename);

        if dev_source_path.exists() {
            std::fs::copy(&dev_source_path, &target_path)
                .map_err(|e| anyhow::anyhow!("复制音频文件失败: {}", e))?;

            eprintln!("✅ 音频文件已从开发环境复制: {} -> {:?}", asset.name, target_path);
            return Ok(target_path);
        }

        // 如果内嵌资源和文件系统都没有找到，返回错误
        Err(anyhow::anyhow!(
            "音频资源 '{}' 不存在。请确保音频文件已正确内嵌到二进制中，或在开发环境中存在于 src/rust/assets/resources/ 目录。",
            asset_id
        ))
    }



    /// 解析音频URL，支持资源ID、文件路径和网络URL
    pub fn parse_audio_url(&self, _app: &AppHandle, audio_url: &str) -> Result<AudioSource> {
        if audio_url.is_empty() {
            // 如果为空，使用第一个可用的音频资源作为默认
            let all_assets = self.get_all_assets();
            if let Some(first_asset) = all_assets.first() {
                return Ok(AudioSource::Asset(first_asset.id.clone()));
            } else {
                return Err(anyhow::anyhow!("没有可用的音频资源"));
            }
        }

        // 检查是否是网络URL
        if audio_url.starts_with("http://") || audio_url.starts_with("https://") {
            return Ok(AudioSource::Url(audio_url.to_string()));
        }

        // 检查是否是资源ID
        if self.assets.contains_key(audio_url) {
            return Ok(AudioSource::Asset(audio_url.to_string()));
        }

        // 尝试通过文件名匹配（去掉扩展名）
        let filename_without_ext = audio_url.split('.').next().unwrap_or(audio_url);
        if self.assets.contains_key(filename_without_ext) {
            return Ok(AudioSource::Asset(filename_without_ext.to_string()));
        }

        // 检查是否是文件路径
        let path = PathBuf::from(audio_url);
        if path.exists() {
            return Ok(AudioSource::File(path));
        }

        // 默认返回文件路径（可能是相对路径）
        Ok(AudioSource::File(path))
    }
}

/// 音频源类型
#[derive(Debug, Clone)]
pub enum AudioSource {
    Asset(String),      // 内置资源ID
    File(PathBuf),      // 本地文件路径
    Url(String),        // 网络URL
}

impl Default for AudioAssetManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 全局音频资源管理器实例
static AUDIO_ASSET_MANAGER: std::sync::OnceLock<std::sync::Mutex<AudioAssetManager>> = std::sync::OnceLock::new();

/// 获取全局音频资源管理器
pub fn get_audio_asset_manager() -> &'static std::sync::Mutex<AudioAssetManager> {
    AUDIO_ASSET_MANAGER.get_or_init(|| std::sync::Mutex::new(AudioAssetManager::new()))
}

/// 初始化音频资源管理器
pub fn initialize_audio_asset_manager(app: &AppHandle) -> Result<()> {
    let manager = get_audio_asset_manager();
    let mut manager = manager.lock().map_err(|e| anyhow::anyhow!("获取管理器锁失败: {}", e))?;
    manager.load_from_app(app)
}

/// 获取所有可用的音频资源（用于前端）
#[tauri::command]
pub async fn get_available_audio_assets() -> Result<Vec<AudioAsset>, String> {
    let manager = get_audio_asset_manager();
    let manager = manager.lock().map_err(|e| format!("获取管理器锁失败: {}", e))?;
    let all_assets = manager.get_all_assets();
    Ok(all_assets.into_iter().cloned().collect())
}

/// 刷新音频资源（重新扫描目录）
#[tauri::command]
pub async fn refresh_audio_assets(app: tauri::AppHandle) -> Result<Vec<AudioAsset>, String> {
    initialize_audio_asset_manager(&app).map_err(|e| format!("刷新音频资源失败: {}", e))?;
    get_available_audio_assets().await
}
