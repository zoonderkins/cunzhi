use anyhow::Result;
use chrono::Utc;
use std::fs;
use std::path::{Path, PathBuf};

use super::types::{MemoryEntry, MemoryCategory, MemoryMetadata};

/// 記憶管理器
pub struct MemoryManager {
    memory_dir: PathBuf,
    project_path: String,
}

impl MemoryManager {
    /// 建立新的記憶管理器
    pub fn new(project_path: &str) -> Result<Self> {
        // 规范化專案路径
        let normalized_path = Self::normalize_project_path(project_path)?;
        let memory_dir = normalized_path.join(".cunzhi-memory");

        // 建立記憶目录，如果失敗则说明專案不适合使用記憶功能
        fs::create_dir_all(&memory_dir)
            .map_err(|e| anyhow::anyhow!(
                "无法在git專案中建立記憶目录: {}\n錯誤: {}\n这可能是因为專案目录没有寫入權限。",
                memory_dir.display(),
                e
            ))?;

        let manager = Self {
            memory_dir,
            project_path: normalized_path.to_string_lossy().to_string(),
        };

        // 初始化記憶檔案结构
        manager.initialize_memory_structure()?;

        Ok(manager)
    }

    /// 规范化專案路径
    fn normalize_project_path(project_path: &str) -> Result<PathBuf> {
        // 使用增强的路径解码和规范化功能
        let normalized_path_str = crate::mcp::utils::decode_and_normalize_path(project_path)
            .map_err(|e| anyhow::anyhow!("路径格式錯誤: {}", e))?;

        let path = Path::new(&normalized_path_str);

        // 转换为绝对路径
        let absolute_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()?.join(path)
        };

        // 规范化路径（解析 . 和 .. 等）
        let canonical_path = absolute_path.canonicalize()
            .unwrap_or_else(|_| {
                // 如果 canonicalize 失敗，尝试手動规范化
                Self::manual_canonicalize(&absolute_path).unwrap_or(absolute_path)
            });

        // 驗證路径是否存在且为目录
        if !canonical_path.exists() {
            return Err(anyhow::anyhow!(
                "專案路径不存在: {}\n原始輸入: {}\n规范化后: {}",
                canonical_path.display(),
                project_path,
                normalized_path_str
            ));
        }

        if !canonical_path.is_dir() {
            return Err(anyhow::anyhow!("專案路径不是目录: {}", canonical_path.display()));
        }

        // 驗證是否为 git 根目录或其子目录
        if let Some(git_root) = Self::find_git_root(&canonical_path) {
            // 如果找到了 git 根目录，使用 git 根目录作为專案路径
            Ok(git_root)
        } else {
            Err(anyhow::anyhow!(
                "錯誤：提供的專案路径不在 git 仓函式庫中。\n路径: {}\n请确保在 git 根目录（包含 .git 檔案夹的目录）中呼叫此功能。",
                canonical_path.display()
            ))
        }
    }

    /// 手動规范化路径
    ///
    /// 當 canonicalize 失敗时的备用方案
    fn manual_canonicalize(path: &Path) -> Result<PathBuf> {
        let mut components = Vec::new();

        for component in path.components() {
            match component {
                std::path::Component::CurDir => {
                    // 忽略 "." 元件
                }
                std::path::Component::ParentDir => {
                    // 處理 ".." 元件
                    if !components.is_empty() {
                        components.pop();
                    }
                }
                _ => {
                    components.push(component);
                }
            }
        }

        let mut result = PathBuf::new();
        for component in components {
            result.push(component);
        }

        Ok(result)
    }

    /// 查找 git 根目录
    fn find_git_root(start_path: &Path) -> Option<PathBuf> {
        let mut current_path = start_path;

        loop {
            // 檢查当前目录是否包含 .git
            let git_path = current_path.join(".git");
            if git_path.exists() {
                return Some(current_path.to_path_buf());
            }

            // 向上一级目录查找
            match current_path.parent() {
                Some(parent) => current_path = parent,
                None => break, // 已经到达根目录
            }
        }

        None
    }

    /// 初始化記憶檔案结构
    fn initialize_memory_structure(&self) -> Result<()> {
        // 建立各类記憶檔案，使用新的结构化格式
        let categories = [
            MemoryCategory::Rule,
            MemoryCategory::Preference,
            MemoryCategory::Pattern,
            MemoryCategory::Context,
        ];

        for category in categories.iter() {
            let filename = match category {
                MemoryCategory::Rule => "rules.md",
                MemoryCategory::Preference => "preferences.md",
                MemoryCategory::Pattern => "patterns.md",
                MemoryCategory::Context => "context.md",
            };

            let file_path = self.memory_dir.join(filename);
            if !file_path.exists() {
                let header_content = self.get_category_header(category);
                fs::write(&file_path, header_content)?;
            }
        }

        // 建立或更新元資料
        self.update_metadata()?;

        Ok(())
    }

    /// 新增記憶条目
    pub fn add_memory(&self, content: &str, category: MemoryCategory) -> Result<String> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();

        let entry = MemoryEntry {
            id: id.clone(),
            content: content.to_string(),
            category,
            created_at: now,
            updated_at: now,
        };

        // 将記憶新增到对应的檔案中
        self.append_to_category_file(&entry)?;

        // 更新元資料
        self.update_metadata()?;

        Ok(id)
    }

    /// 獲取所有記憶
    pub fn get_all_memories(&self) -> Result<Vec<MemoryEntry>> {
        let mut memories = Vec::new();

        let categories = [
            (MemoryCategory::Rule, "rules.md"),
            (MemoryCategory::Preference, "preferences.md"),
            (MemoryCategory::Pattern, "patterns.md"),
            (MemoryCategory::Context, "context.md"),
        ];

        for (category, filename) in categories.iter() {
            let file_path = self.memory_dir.join(filename);
            if file_path.exists() {
                let content = fs::read_to_string(&file_path)?;
                let entries = self.parse_memory_file(&content, *category)?;
                memories.extend(entries);
            }
        }

        // 按更新时间排序
        memories.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

        Ok(memories)
    }

    /// 獲取指定分类的記憶
    pub fn get_memories_by_category(&self, category: MemoryCategory) -> Result<Vec<MemoryEntry>> {
        let filename = match category {
            MemoryCategory::Rule => "rules.md",
            MemoryCategory::Preference => "preferences.md",
            MemoryCategory::Pattern => "patterns.md",
            MemoryCategory::Context => "context.md",
        };

        let file_path = self.memory_dir.join(filename);
        if !file_path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&file_path)?;
        self.parse_memory_file(&content, category)
    }

    /// 将記憶条目新增到对应分类檔案
    fn append_to_category_file(&self, entry: &MemoryEntry) -> Result<()> {
        let filename = match entry.category {
            MemoryCategory::Rule => "rules.md",
            MemoryCategory::Preference => "preferences.md",
            MemoryCategory::Pattern => "patterns.md",
            MemoryCategory::Context => "context.md",
        };

        let file_path = self.memory_dir.join(filename);
        let mut content = if file_path.exists() {
            fs::read_to_string(&file_path)?
        } else {
            format!("# {}\n\n", self.get_category_title(&entry.category))
        };

        // 简化格式：一行一个記憶
        content.push_str(&format!("- {}\n", entry.content));

        fs::write(&file_path, content)?;
        Ok(())
    }

    /// 解析記憶檔案内容 - 简化版本
    fn parse_memory_file(&self, content: &str, category: MemoryCategory) -> Result<Vec<MemoryEntry>> {
        let mut memories = Vec::new();

        // 按列表项解析，每个 "- " 开头的行是一个記憶条目
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("- ") && line.len() > 2 {
                let content = line[2..].trim(); // 去掉 "- " 前缀
                if !content.is_empty() {
                    let entry = MemoryEntry {
                        id: uuid::Uuid::new_v4().to_string(),
                        content: content.to_string(),
                        category,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                    };

                    memories.push(entry);
                }
            }
        }

        Ok(memories)
    }

    /// 獲取分类标题
    fn get_category_title(&self, category: &MemoryCategory) -> &str {
        match category {
            MemoryCategory::Rule => "开发规范和规则",
            MemoryCategory::Preference => "用户偏好設定",
            MemoryCategory::Pattern => "常用模式和最佳实践",
            MemoryCategory::Context => "專案上下文訊息",
        }
    }

    /// 獲取分类檔案头部（简化版本）
    fn get_category_header(&self, category: &MemoryCategory) -> String {
        format!("# {}\n\n", self.get_category_title(category))
    }

    /// 更新元資料
    fn update_metadata(&self) -> Result<()> {
        let metadata = MemoryMetadata {
            project_path: self.project_path.clone(),
            last_organized: Utc::now(),
            total_entries: self.get_all_memories()?.len(),
            version: "1.0.0".to_string(),
        };

        let metadata_path = self.memory_dir.join("metadata.json");
        let metadata_json = serde_json::to_string_pretty(&metadata)?;
        fs::write(metadata_path, metadata_json)?;

        Ok(())
    }

    /// 獲取專案訊息供MCP呼叫方分析 - 压缩简化版本
    pub fn get_project_info(&self) -> Result<String> {
        // 汇总所有記憶规则并压缩
        let all_memories = self.get_all_memories()?;
        if all_memories.is_empty() {
            return Ok("📭 暂无專案記憶".to_string());
        }

        let mut compressed_info = Vec::new();

        // 按分类压缩汇总
        let categories = [
            (MemoryCategory::Rule, "规范"),
            (MemoryCategory::Preference, "偏好"),
            (MemoryCategory::Pattern, "模式"),
            (MemoryCategory::Context, "背景"),
        ];

        for (category, title) in categories.iter() {
            let memories = self.get_memories_by_category(*category)?;
            if !memories.is_empty() {
                let mut items = Vec::new();
                for memory in memories {
                    let content = memory.content.trim();
                    if !content.is_empty() {
                        // 去除多余空格和换行，压缩内容
                        let compressed_content = content
                            .split_whitespace()
                            .collect::<Vec<&str>>()
                            .join(" ");
                        items.push(compressed_content);
                    }
                }
                if !items.is_empty() {
                    compressed_info.push(format!("**{}**: {}", title, items.join("; ")));
                }
            }
        }

        if compressed_info.is_empty() {
            Ok("📭 暂无有效專案記憶".to_string())
        } else {
            Ok(format!("📚 專案記憶总览: {}", compressed_info.join(" | ")))
        }
    }
}
