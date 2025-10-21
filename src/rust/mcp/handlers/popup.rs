use anyhow::Result;
use std::process::Command;
use std::fs;
use std::path::{Path, PathBuf};

use crate::mcp::types::PopupRequest;
use crate::log_debug;

/// RAII 临时文件自动清理器
struct TempFile(PathBuf);

impl TempFile {
    fn new(path: PathBuf) -> Self {
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        if self.0.exists() {
            match fs::remove_file(&self.0) {
                Ok(_) => log_debug!("临时文件已清理: {:?}", self.0),
                Err(e) => log_debug!("清理临时文件失败: {:?}, 错误: {}", self.0, e),
            }
        }
    }
}

/// 清理旧的 MCP 临时文件
///
/// 启动时调用，清理超过 1 小时的临时文件
pub fn cleanup_old_temp_files() {
    use std::time::{SystemTime, UNIX_EPOCH};

    let temp_dir = std::env::temp_dir();
    log_debug!("开始清理旧临时文件，目录: {:?}", temp_dir);

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let mut cleaned_count = 0;

    if let Ok(entries) = fs::read_dir(&temp_dir) {
        for entry in entries.flatten() {
            if let Ok(file_name) = entry.file_name().into_string() {
                // 只处理 MCP 请求临时文件
                if file_name.starts_with("mcp_request_") && file_name.ends_with(".json") {
                    // 检查文件修改时间
                    if let Ok(metadata) = entry.metadata() {
                        if let Ok(modified) = metadata.modified() {
                            if let Ok(modified_since_epoch) = modified.duration_since(UNIX_EPOCH) {
                                let age_secs = now.saturating_sub(modified_since_epoch.as_secs());

                                // 清理超过 1 小时（3600 秒）的文件
                                if age_secs > 3600 {
                                    if let Err(e) = fs::remove_file(entry.path()) {
                                        log_debug!("清理旧临时文件失败: {:?}, 错误: {}", entry.path(), e);
                                    } else {
                                        cleaned_count += 1;
                                        log_debug!("已清理旧临时文件: {:?} ({}小时前)", entry.path(), age_secs / 3600);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if cleaned_count > 0 {
        log_debug!("清理完成，共清理 {} 个旧临时文件", cleaned_count);
    }
}

/// 创建 Tauri 弹窗
///
/// 优先调用与 MCP 服务器同目录的 UI 命令，找不到时使用全局版本
pub fn create_tauri_popup(request: &PopupRequest) -> Result<String> {
    // 创建临时请求文件 - 跨平台适配
    let temp_dir = std::env::temp_dir();
    let temp_file_path = temp_dir.join(format!("mcp_request_{}.json", request.id));
    let request_json = serde_json::to_string_pretty(request)?;
    fs::write(&temp_file_path, request_json)?;

    // 使用 RAII 自动清理临时文件
    let temp_file = TempFile::new(temp_file_path);

    // 尝试找到等一下命令的路径
    let command_path = find_ui_command()?;

    // 调用等一下命令
    let output = Command::new(&command_path)
        .arg("--mcp-request")
        .arg(temp_file.path().to_string_lossy().to_string())
        .output()?;

    // temp_file 会在这里自动清理

    if output.status.success() {
        let response = String::from_utf8_lossy(&output.stdout);
        let response = response.trim();
        if response.is_empty() {
            Ok("用户取消了操作".to_string())
        } else {
            Ok(response.to_string())
        }
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("UI进程失败: {}", error);
    }
}

/// 查找等一下 UI 命令的路径
///
/// 按优先级查找：同目录 -> 全局版本 -> 开发环境
fn find_ui_command() -> Result<String> {
    use crate::log_important;

    // 1. 优先尝试与当前 MCP 服务器同目录的等一下命令
    if let Ok(current_exe) = std::env::current_exe() {
        log_debug!("当前可执行文件路径: {:?}", current_exe);

        if let Some(exe_dir) = current_exe.parent() {
            log_debug!("可执行文件目录: {:?}", exe_dir);

            let local_ui_path = exe_dir.join("等一下");
            log_debug!("查找本地 UI 命令: {:?}", local_ui_path);

            if local_ui_path.exists() {
                log_debug!("找到 等一下 文件，检查是否可执行...");
                if is_executable(&local_ui_path) {
                    log_important!(info, "使用同目录的 等一下 命令: {:?}", local_ui_path);
                    return Ok(local_ui_path.to_string_lossy().to_string());
                } else {
                    log_debug!("等一下 文件存在但不可执行");
                }
            } else {
                log_debug!("同目录下找不到 等一下 文件");
            }
        }
    }

    // 2. 尝试全局命令（最常见的部署方式）
    log_debug!("尝试查找全局 等一下 命令...");
    if test_command_available("等一下") {
        log_important!(info, "使用全局 等一下 命令");
        return Ok("等一下".to_string());
    }

    // 3. 如果都找不到，返回详细错误信息
    log_important!(error, "找不到等一下 UI 命令");
    anyhow::bail!(
        "找不到等一下 UI 命令。请确保：\n\
         1. 已编译项目：cargo build --release\n\
         2. 或已全局安装：./install.sh\n\
         3. 或等一下命令在同目录下"
    )
}

/// 测试命令是否可用
fn test_command_available(command: &str) -> bool {
    Command::new(command)
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// 检查文件是否可执行
fn is_executable(path: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        path.metadata()
            .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
    }

    #[cfg(windows)]
    {
        // Windows 上检查文件扩展名
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.eq_ignore_ascii_case("exe"))
            .unwrap_or(false)
    }
}
