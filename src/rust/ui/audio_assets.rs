use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use tauri::{AppHandle, Manager};

/// 音频资源信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioAsset {
    pub id: String,
    pub name: String,
    pub filename: String,
}

/// 音频元数据配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioMetadata {
    pub version: String,
    pub description: String,
    pub sounds: HashMap<String, AudioAssetInfo>,
}

/// 音频资源详细信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioAssetInfo {
    pub id: String,
    pub name: String,
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
        // 先尝试从元数据文件加载
        if let Ok(()) = self.load_from_metadata(app) {
            return Ok(());
        }

        // 如果元数据文件不存在，回退到扫描目录
        self.scan_audio_directory(app)
    }

    /// 从元数据文件加载音频资源
    fn load_from_metadata(&mut self, app: &AppHandle) -> Result<()> {
        // 尝试多个可能的元数据文件路径
        let possible_paths = vec![
            // 开发环境路径
            std::env::current_dir().unwrap_or_default().join("src/assets/sounds/metadata.json"),
            // 生产环境路径
            app.path().resource_dir().unwrap_or_default().join("src/assets/sounds/metadata.json"),
        ];

        let mut metadata_path = None;
        for path in &possible_paths {
            if path.exists() {
                metadata_path = Some(path.clone());
                break;
            }
        }

        let metadata_path = metadata_path.ok_or_else(|| {
            anyhow::anyhow!("元数据文件不存在，尝试的路径: {:?}",
                possible_paths.iter().map(|p| p.to_string_lossy()).collect::<Vec<_>>())
        })?;

        let metadata_content = fs::read_to_string(&metadata_path)
            .map_err(|e| anyhow::anyhow!("读取元数据文件失败: {}", e))?;

        let metadata: AudioMetadata = serde_json::from_str(&metadata_content)
            .map_err(|e| anyhow::anyhow!("解析元数据文件失败: {}", e))?;

        // 转换为 AudioAsset
        for (filename, asset_info) in metadata.sounds {
            let asset = AudioAsset {
                id: asset_info.id,
                name: asset_info.name,
                filename,
            };
            self.assets.insert(asset.id.clone(), asset);
        }

        println!("✅ 从元数据文件加载了 {} 个音频资源", self.assets.len());
        Ok(())
    }

    /// 扫描音频目录
    fn scan_audio_directory(&mut self, app: &AppHandle) -> Result<()> {
        // 尝试多个可能的音频目录路径
        let possible_paths = vec![
            // 开发环境路径
            std::env::current_dir().unwrap_or_default().join("src/assets/sounds"),
            // 生产环境路径
            app.path().resource_dir().unwrap_or_default().join("src/assets/sounds"),
            // 备用路径
            app.path().app_data_dir().unwrap_or_default().join("sounds"),
        ];

        let mut sounds_dir = None;
        for path in &possible_paths {
            if path.exists() {
                sounds_dir = Some(path.clone());
                break;
            }
        }

        let sounds_dir = sounds_dir.ok_or_else(|| {
            anyhow::anyhow!("无法找到音频目录，尝试的路径: {:?}",
                possible_paths.iter().map(|p| p.to_string_lossy()).collect::<Vec<_>>())
        })?;

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

        println!("✅ 扫描音频目录，发现 {} 个音频文件", self.assets.len());
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
    fn create_asset_from_filename(&self, filename: &str) -> AudioAsset {
        let name_without_ext = filename.split('.').next().unwrap_or(filename);

        // 根据文件名推断信息
        let (id, name) = match name_without_ext {
            "ji" => ("ikun".to_string(), "iKun".to_string()),
            "a" => ("elegant".to_string(), "销魂".to_string()),
            "gaowan" => ("gaowan".to_string(), "睾丸了".to_string()),
            "100w" => ("100w".to_string(), "100万".to_string()),
            _ => {
                let id = name_without_ext.to_lowercase().replace(' ', "_");
                let name = name_without_ext.to_string();
                (id, name)
            }
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

    /// 获取音频文件的完整路径
    pub fn get_audio_path(&self, app: &AppHandle, asset_id: &str) -> Result<PathBuf> {
        let asset = self.get_asset_by_id(asset_id)
            .ok_or_else(|| anyhow::anyhow!("未找到音频资源: {}", asset_id))?;

        // 首先尝试从用户配置目录获取
        let config_dir = app.path().app_config_dir()
            .map_err(|e| anyhow::anyhow!("无法获取应用配置目录: {}", e))?;
        
        let user_audio_path = config_dir.join("sounds").join(&asset.filename);
        if user_audio_path.exists() {
            return Ok(user_audio_path);
        }

        // 如果用户配置目录中没有，尝试从多个可能的资源目录获取
        let possible_resource_paths = vec![
            // 开发环境路径
            std::env::current_dir().unwrap_or_default().join("src/assets/sounds").join(&asset.filename),
            // 生产环境路径
            app.path().resource_dir().unwrap_or_default().join("src/assets/sounds").join(&asset.filename),
        ];

        for resource_audio_path in possible_resource_paths {
            if resource_audio_path.exists() {
                return Ok(resource_audio_path);
            }
        }

        Err(anyhow::anyhow!("音频文件不存在: {}", asset.filename))
    }

    /// 确保音频文件存在，如果不存在则从资源目录复制到用户配置目录
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

        // 尝试多个可能的源文件路径
        let possible_source_paths = vec![
            // 开发环境路径
            std::env::current_dir().unwrap_or_default().join("src/assets/sounds").join(&asset.filename),
            // 生产环境路径
            app.path().resource_dir().unwrap_or_default().join("src/assets/sounds").join(&asset.filename),
        ];

        let mut source_path = None;
        for path in &possible_source_paths {
            if path.exists() {
                source_path = Some(path.clone());
                break;
            }
        }

        let source_path = source_path.ok_or_else(|| {
            anyhow::anyhow!("源音频文件不存在，尝试的路径: {:?}",
                possible_source_paths.iter().map(|p| p.to_string_lossy()).collect::<Vec<_>>())
        })?;

        std::fs::copy(&source_path, &target_path)
            .map_err(|e| anyhow::anyhow!("复制音频文件失败: {}", e))?;

        println!("✅ 音频文件已复制: {} -> {:?}", asset.name, target_path);
        Ok(target_path)
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

        // 兼容旧格式
        let legacy_mapping = [
            ("ji", "ikun"),
            ("a", "elegant"),
            ("ji.mp3", "ikun"),
            ("a.mp3", "elegant"),
            ("gaowan.mp3", "gaowan"),
            ("100w.m4a", "100w"),
            ("src/assets/sounds/ji.mp3", "ikun"),
            ("src/assets/sounds/a.mp3", "elegant"),
            ("src/assets/sounds/gaowan.mp3", "gaowan"),
            ("src/assets/sounds/100w.m4a", "100w"),
        ];

        for (old_format, new_id) in &legacy_mapping {
            if audio_url == *old_format {
                return Ok(AudioSource::Asset(new_id.to_string()));
            }
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
