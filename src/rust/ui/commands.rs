use crate::config::{save_config, load_config as load_config_from_file, AppState, ReplyConfig, WindowConfig, CustomPrompt, CustomPromptConfig, ShortcutConfig, ShortcutBinding, AppConfig};
use crate::constants::{window, ui, validation};
use crate::mcp::types::{build_continue_response, build_send_response, ImageAttachment, PopupRequest};
use crate::mcp::handlers::create_tauri_popup;
use tauri::{AppHandle, Manager, State};

#[tauri::command]
pub async fn get_app_info() -> Result<String, String> {
    Ok(format!("寸止 v{}", env!("CARGO_PKG_VERSION")))
}

#[tauri::command]
pub async fn get_always_on_top(state: State<'_, AppState>) -> Result<bool, String> {
    let config = state
        .config
        .lock()
        .map_err(|e| format!("獲取設定失敗: {}", e))?;
    Ok(config.ui_config.always_on_top)
}

#[tauri::command]
pub async fn set_always_on_top(
    enabled: bool,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    {
        let mut config = state
            .config
            .lock()
            .map_err(|e| format!("獲取設定失敗: {}", e))?;
        config.ui_config.always_on_top = enabled;
    }

    // 儲存設定到檔案
    save_config(&state, &app)
        .await
        .map_err(|e| format!("儲存設定失敗: {}", e))?;

    // 應用到当前視窗
    if let Some(window) = app.get_webview_window("main") {
        window
            .set_always_on_top(enabled)
            .map_err(|e| format!("設定視窗置顶失敗: {}", e))?;

        log::info!("用户切换視窗置顶狀態为: {} (已儲存設定)", enabled);
    }

    Ok(())
}

#[tauri::command]
pub async fn sync_window_state(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    // 根据設定同步視窗狀態
    let always_on_top = {
        let config = state
            .config
            .lock()
            .map_err(|e| format!("獲取設定失敗: {}", e))?;
        config.ui_config.always_on_top
    };

    // 應用到当前視窗
    if let Some(window) = app.get_webview_window("main") {
        window
            .set_always_on_top(always_on_top)
            .map_err(|e| format!("同步視窗狀態失敗: {}", e))?;
    }

    Ok(())
}

/// 載入設定（傳回完整設定對象）
#[tauri::command]
pub async fn load_config(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let config = state
        .config
        .lock()
        .map_err(|e| format!("獲取設定失敗: {}", e))?;

    serde_json::to_value(&*config)
        .map_err(|e| format!("序列化設定失敗: {}", e))
}

/// 儲存設定（接收完整設定對象）
#[tauri::command]
pub async fn save_config_cmd(
    config_value: serde_json::Value,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    // 解析設定
    let new_config: AppConfig = serde_json::from_value(config_value)
        .map_err(|e| format!("解析設定失敗: {}", e))?;

    // 更新內存中的設定
    {
        let mut config = state
            .config
            .lock()
            .map_err(|e| format!("獲取設定失敗: {}", e))?;
        *config = new_config;
    }

    // 儲存到檔案
    save_config(&state, &app)
        .await
        .map_err(|e| format!("儲存設定失敗: {}", e))?;

    Ok(())
}

/// 重新載入設定檔案到記憶體
#[tauri::command]
pub async fn reload_config(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    // 从檔案重新載入設定到記憶體
    load_config_from_file(&state, &app)
        .await
        .map_err(|e| format!("重新載入設定失敗: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn get_theme(state: State<'_, AppState>) -> Result<String, String> {
    let config = state
        .config
        .lock()
        .map_err(|e| format!("獲取設定失敗: {}", e))?;
    Ok(config.ui_config.theme.clone())
}

#[tauri::command]
pub async fn set_theme(
    theme: String,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    // 驗證主題值
    if !["light", "dark"].contains(&theme.as_str()) {
        return Err("无效的主題值，只支持 light、dark".to_string());
    }

    {
        let mut config = state
            .config
            .lock()
            .map_err(|e| format!("獲取設定失敗: {}", e))?;
        config.ui_config.theme = theme;
    }

    // 儲存設定到檔案
    save_config(&state, &app)
        .await
        .map_err(|e| format!("儲存設定失敗: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn get_window_config(state: State<'_, AppState>) -> Result<WindowConfig, String> {
    let config = state
        .config
        .lock()
        .map_err(|e| format!("獲取設定失敗: {}", e))?;
    Ok(config.ui_config.window_config.clone())
}

#[tauri::command]
pub async fn set_window_config(
    window_config: WindowConfig,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    {
        let mut config = state
            .config
            .lock()
            .map_err(|e| format!("獲取設定失敗: {}", e))?;
        config.ui_config.window_config = window_config;
    }

    // 儲存設定到檔案
    save_config(&state, &app)
        .await
        .map_err(|e| format!("儲存設定失敗: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn get_reply_config(state: State<'_, AppState>) -> Result<ReplyConfig, String> {
    let config = state
        .config
        .lock()
        .map_err(|e| format!("獲取設定失敗: {}", e))?;
    Ok(config.reply_config.clone())
}

#[tauri::command]
pub async fn set_reply_config(
    reply_config: ReplyConfig,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    {
        let mut config = state
            .config
            .lock()
            .map_err(|e| format!("獲取設定失敗: {}", e))?;
        config.reply_config = reply_config;
    }

    // 儲存設定到檔案
    save_config(&state, &app)
        .await
        .map_err(|e| format!("儲存設定失敗: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn get_window_settings(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let config = state
        .config
        .lock()
        .map_err(|e| format!("獲取設定失敗: {}", e))?;

    // 傳回視窗設定，包含两种模式的独立尺寸
    let window_settings = serde_json::json!({
        "fixed": config.ui_config.window_config.fixed,
        "current_width": config.ui_config.window_config.current_width(),
        "current_height": config.ui_config.window_config.current_height(),
        "fixed_width": config.ui_config.window_config.fixed_width,
        "fixed_height": config.ui_config.window_config.fixed_height,
        "free_width": config.ui_config.window_config.free_width,
        "free_height": config.ui_config.window_config.free_height
    });

    Ok(window_settings)
}

#[tauri::command]
pub async fn get_window_settings_for_mode(
    fixed: bool,
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let config = state
        .config
        .lock()
        .map_err(|e| format!("獲取設定失敗: {}", e))?;

    // 傳回指定模式的視窗設定
    let (width, height) = if fixed {
        (
            config.ui_config.window_config.fixed_width,
            config.ui_config.window_config.fixed_height,
        )
    } else {
        (
            config.ui_config.window_config.free_width,
            config.ui_config.window_config.free_height,
        )
    };

    let window_settings = serde_json::json!({
        "width": width,
        "height": height,
        "fixed": fixed
    });

    Ok(window_settings)
}

#[tauri::command]
pub async fn get_window_constraints_cmd() -> Result<serde_json::Value, String> {
    let constraints = window::get_default_constraints();
    let ui_timings = ui::get_default_ui_timings();

    let mut result = constraints.to_json();
    if let serde_json::Value::Object(ref mut map) = result {
        if let serde_json::Value::Object(ui_map) = ui_timings.to_json() {
            map.extend(ui_map);
        }
    }

    Ok(result)
}

#[tauri::command]
pub async fn get_current_window_size(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    if let Some(window) = app.get_webview_window("main") {
        // 檢查視窗是否最小化
        if let Ok(is_minimized) = window.is_minimized() {
            if is_minimized {
                return Err("視窗已最小化，跳過尺寸獲取".to_string());
            }
        }

        // 獲取逻辑尺寸而不是物理尺寸
        if let Ok(logical_size) = window.inner_size().map(|physical_size| {
            // 獲取缩放因子
            let scale_factor = window.scale_factor().unwrap_or(1.0);

            // 转换为逻辑尺寸
            let logical_width = physical_size.width as f64 / scale_factor;
            let logical_height = physical_size.height as f64 / scale_factor;

            tauri::LogicalSize::new(logical_width, logical_height)
        }) {
            let width = logical_size.width.round() as u32;
            let height = logical_size.height.round() as u32;

            // 驗證并调整尺寸到有效范围
            let (clamped_width, clamped_height) = crate::constants::window::clamp_window_size(width as f64, height as f64);
            let final_width = clamped_width as u32;
            let final_height = clamped_height as u32;

            if final_width != width || final_height != height {
                log::info!("視窗尺寸已调整: {}x{} -> {}x{}", width, height, final_width, final_height);
            }

            let window_size = serde_json::json!({
                "width": final_width,
                "height": final_height
            });
            return Ok(window_size);
        }
    }

    Err("无法獲取当前視窗大小".to_string())
}

#[tauri::command]
pub async fn set_window_settings(
    window_settings: serde_json::Value,
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    {
        let mut config = state
            .config
            .lock()
            .map_err(|e| format!("獲取設定失敗: {}", e))?;

        // 更新視窗設定
        if let Some(fixed) = window_settings.get("fixed").and_then(|v| v.as_bool()) {
            config.ui_config.window_config.fixed = fixed;
        }

        // 更新固定模式尺寸（新增尺寸驗證）
        if let Some(width) = window_settings.get("fixed_width").and_then(|v| v.as_f64()) {
            if let Some(height) = window_settings.get("fixed_height").and_then(|v| v.as_f64()) {
                if validation::is_valid_window_size(width, height) {
                    config.ui_config.window_config.fixed_width = width;
                    config.ui_config.window_config.fixed_height = height;
                }
            } else if width >= window::MIN_WIDTH {
                config.ui_config.window_config.fixed_width = width;
            }
        } else if let Some(height) = window_settings.get("fixed_height").and_then(|v| v.as_f64()) {
            if height >= window::MIN_HEIGHT {
                config.ui_config.window_config.fixed_height = height;
            }
        }

        // 更新自由拉伸模式尺寸（新增尺寸驗證）
        if let Some(width) = window_settings.get("free_width").and_then(|v| v.as_f64()) {
            if let Some(height) = window_settings.get("free_height").and_then(|v| v.as_f64()) {
                if validation::is_valid_window_size(width, height) {
                    config.ui_config.window_config.free_width = width;
                    config.ui_config.window_config.free_height = height;
                }
            } else if width >= window::MIN_WIDTH {
                config.ui_config.window_config.free_width = width;
            }
        } else if let Some(height) = window_settings.get("free_height").and_then(|v| v.as_f64()) {
            if height >= window::MIN_HEIGHT {
                config.ui_config.window_config.free_height = height;
            }
        }

        // 兼容旧的width/height參數，更新目前模式的尺寸（新增尺寸驗證）
        if let (Some(width), Some(height)) = (
            window_settings.get("width").and_then(|v| v.as_f64()),
            window_settings.get("height").and_then(|v| v.as_f64()),
        ) {
            if validation::is_valid_window_size(width, height) {
                config
                    .ui_config
                    .window_config
                    .update_current_size(width, height);
            }
        }
    }

    // 儲存設定到檔案
    save_config(&state, &app)
        .await
        .map_err(|e| format!("儲存設定失敗: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn send_mcp_response(
    response: serde_json::Value,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // 将响应序列化为JSON字符串
    let response_str =
        serde_json::to_string(&response).map_err(|e| format!("序列化响应失敗: {}", e))?;

    if response_str.trim().is_empty() {
        return Err("响应内容不能为空".to_string());
    }

    // 檢查是否为MCP模式
    let args: Vec<String> = std::env::args().collect();
    let is_mcp_mode = args.len() >= 3 && args[1] == "--mcp-request";

    if is_mcp_mode {
        // MCP模式：直接輸出到stdout（MCP协议要求）
        println!("{}", response_str);
        std::io::Write::flush(&mut std::io::stdout())
            .map_err(|e| format!("重新整理stdout失敗: {}", e))?;
    } else {
        // 通过channel发送响应（如果有的话）
        let sender = {
            let mut channel = state
                .response_channel
                .lock()
                .map_err(|e| format!("獲取响应通道失敗: {}", e))?;
            channel.take()
        };

        if let Some(sender) = sender {
            let _ = sender.send(response_str);
        }
    }

    Ok(())
}

#[tauri::command]
pub fn get_cli_args() -> Result<serde_json::Value, String> {
    let args: Vec<String> = std::env::args().collect();
    let mut result = serde_json::Map::new();

    // 檢查是否有 --mcp-request 參數
    if args.len() >= 3 && args[1] == "--mcp-request" {
        result.insert(
            "mcp_request".to_string(),
            serde_json::Value::String(args[2].clone()),
        );
    }

    Ok(serde_json::Value::Object(result))
}

#[tauri::command]
pub fn read_mcp_request(file_path: String) -> Result<serde_json::Value, String> {
    if !std::path::Path::new(&file_path).exists() {
        return Err(format!("檔案不存在: {}", file_path));
    }

    match std::fs::read_to_string(&file_path) {
        Ok(content) => {
            if content.trim().is_empty() {
                return Err("檔案内容为空".to_string());
            }
            match serde_json::from_str(&content) {
                Ok(json) => Ok(json),
                Err(e) => Err(format!("解析JSON失敗: {}", e)),
            }
        }
        Err(e) => Err(format!("讀取檔案失敗: {}", e)),
    }
}

#[tauri::command]
pub async fn select_image_files() -> Result<Vec<String>, String> {
    // 简化版本：傳回測試图片資料
    // 在实际應用中，这里应该呼叫系統檔案对话框
    let test_image_base64 = "data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTAwIiBoZWlnaHQ9IjEwMCIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KICA8cmVjdCB3aWR0aD0iMTAwIiBoZWlnaHQ9IjEwMCIgZmlsbD0iIzMzNzNkYyIvPgogIDx0ZXh0IHg9IjUwIiB5PSI1NSIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0IiBmaWxsPSJ3aGl0ZSIgdGV4dC1hbmNob3I9Im1pZGRsZSI+VGF1cmk8L3RleHQ+Cjwvc3ZnPg==";

    Ok(vec![test_image_base64.to_string()])
}

#[tauri::command]
pub async fn open_external_url(url: String) -> Result<(), String> {
    use std::process::Command;

    // 移除不重要的偵錯訊息

    // 根据操作系統選擇合适的命令
    let result = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "start", &url])
            .spawn()
    } else if cfg!(target_os = "macos") {
        Command::new("open")
            .arg(&url)
            .spawn()
    } else {
        // Linux 和其他 Unix 系統
        Command::new("xdg-open")
            .arg(&url)
            .spawn()
    };

    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("无法開啟連結: {}", e))
    }
}

#[tauri::command]
pub async fn exit_app(app: AppHandle) -> Result<(), String> {
    // 直接呼叫強制退出，用于程序内部的退出操作（如MCP响应后退出）
    crate::ui::exit::force_exit_app(app).await
}



/// 處理應用退出请求（用于前端退出快捷键）
#[tauri::command]
pub async fn handle_app_exit_request(app: AppHandle) -> Result<bool, String> {
    crate::ui::exit_handler::handle_exit_request_internal(app).await
}

/// 建構发送操作的MCP响应
#[tauri::command]
pub fn build_mcp_send_response(
    user_input: Option<String>,
    selected_options: Vec<String>,
    images: Vec<ImageAttachment>,
    request_id: Option<String>,
    source: String,
) -> Result<String, String> {
    Ok(build_send_response(
        user_input,
        selected_options,
        images,
        request_id,
        &source,
    ))
}

/// 建構繼續操作的MCP响应
#[tauri::command]
pub fn build_mcp_continue_response(
    request_id: Option<String>,
    source: String,
) -> Result<String, String> {
    Ok(build_continue_response(request_id, &source))
}

/// 建立測試popup視窗
#[tauri::command]
pub async fn create_test_popup(request: serde_json::Value) -> Result<String, String> {
    // 将JSON值转换为PopupRequest
    let popup_request: PopupRequest = serde_json::from_value(request)
        .map_err(|e| format!("解析请求參數失敗: {}", e))?;

    // 呼叫现有的popup建立函數
    match create_tauri_popup(&popup_request) {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("建立測試popup失敗: {}", e))
    }
}

// 自訂prompt相关命令

/// 獲取自訂prompt設定
#[tauri::command]
pub async fn get_custom_prompt_config(state: State<'_, AppState>) -> Result<CustomPromptConfig, String> {
    let config = state
        .config
        .lock()
        .map_err(|e| format!("獲取設定失敗: {}", e))?;
    Ok(config.custom_prompt_config.clone())
}

/// 新增自訂prompt
#[tauri::command]
pub async fn add_custom_prompt(
    prompt: CustomPrompt,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<(), String> {
    {
        let mut config = state
            .config
            .lock()
            .map_err(|e| format!("獲取設定失敗: {}", e))?;

        // 檢查是否超过最大数量限制
        if config.custom_prompt_config.prompts.len() >= config.custom_prompt_config.max_prompts as usize {
            return Err(format!("自訂prompt数量已达到上限: {}", config.custom_prompt_config.max_prompts));
        }

        // 檢查ID是否已存在
        if config.custom_prompt_config.prompts.iter().any(|p| p.id == prompt.id) {
            return Err("prompt ID已存在".to_string());
        }

        config.custom_prompt_config.prompts.push(prompt);
    }

    // 儲存設定到檔案
    save_config(&state, &app)
        .await
        .map_err(|e| format!("儲存設定失敗: {}", e))?;

    Ok(())
}

/// 更新自訂prompt
#[tauri::command]
pub async fn update_custom_prompt(
    prompt: CustomPrompt,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<(), String> {
    {
        let mut config = state
            .config
            .lock()
            .map_err(|e| format!("獲取設定失敗: {}", e))?;

        // 查找并更新prompt
        if let Some(existing_prompt) = config.custom_prompt_config.prompts.iter_mut().find(|p| p.id == prompt.id) {
            *existing_prompt = prompt;
        } else {
            return Err("未找到指定的prompt".to_string());
        }
    }

    // 儲存設定到檔案
    save_config(&state, &app)
        .await
        .map_err(|e| format!("儲存設定失敗: {}", e))?;

    Ok(())
}

/// 刪除自訂prompt
#[tauri::command]
pub async fn delete_custom_prompt(
    prompt_id: String,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<(), String> {
    {
        let mut config = state
            .config
            .lock()
            .map_err(|e| format!("獲取設定失敗: {}", e))?;

        // 查找并刪除prompt
        let initial_len = config.custom_prompt_config.prompts.len();
        config.custom_prompt_config.prompts.retain(|p| p.id != prompt_id);

        if config.custom_prompt_config.prompts.len() == initial_len {
            return Err("未找到指定的prompt".to_string());
        }
    }

    // 儲存設定到檔案
    save_config(&state, &app)
        .await
        .map_err(|e| format!("儲存設定失敗: {}", e))?;

    Ok(())
}

/// 設定自訂prompt启用狀態
#[tauri::command]
pub async fn set_custom_prompt_enabled(
    enabled: bool,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<(), String> {
    {
        let mut config = state
            .config
            .lock()
            .map_err(|e| format!("獲取設定失敗: {}", e))?;
        config.custom_prompt_config.enabled = enabled;
    }

    // 儲存設定到檔案
    save_config(&state, &app)
        .await
        .map_err(|e| format!("儲存設定失敗: {}", e))?;

    Ok(())
}

/// 更新自訂prompt排序
#[tauri::command]
pub async fn update_custom_prompt_order(
    prompt_ids: Vec<String>,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<(), String> {
    log::debug!("开始更新prompt排序，接收到的IDs: {:?}", prompt_ids);

    {
        let mut config = state
            .config
            .lock()
            .map_err(|e| format!("獲取設定失敗: {}", e))?;

        log::debug!("更新前的prompt顺序:");
        for prompt in &config.custom_prompt_config.prompts {
            log::debug!("  {} (sort_order: {})", prompt.name, prompt.sort_order);
        }

        // 根据新的顺序更新sort_order
        for (index, prompt_id) in prompt_ids.iter().enumerate() {
            if let Some(prompt) = config.custom_prompt_config.prompts.iter_mut().find(|p| p.id == *prompt_id) {
                let old_order = prompt.sort_order;
                prompt.sort_order = (index + 1) as i32;
                prompt.updated_at = chrono::Utc::now().to_rfc3339();
                log::debug!("更新prompt '{}': {} -> {}", prompt.name, old_order, prompt.sort_order);
            }
        }

        // 按sort_order排序
        config.custom_prompt_config.prompts.sort_by_key(|p| p.sort_order);

        log::debug!("更新后的prompt顺序:");
        for prompt in &config.custom_prompt_config.prompts {
            log::debug!("  {} (sort_order: {})", prompt.name, prompt.sort_order);
        }
    }

    log::debug!("开始儲存設定檔案...");
    let save_start = std::time::Instant::now();

    // 儲存設定到檔案
    save_config(&state, &app)
        .await
        .map_err(|e| format!("儲存設定失敗: {}", e))?;

    let save_duration = save_start.elapsed();
    log::debug!("設定儲存完成，耗时: {:?}", save_duration);

    Ok(())
}

/// 更新条件性prompt狀態
#[tauri::command]
pub async fn update_conditional_prompt_state(
    prompt_id: String,
    new_state: bool,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<(), String> {
    {
        let mut config = state
            .config
            .lock()
            .map_err(|e| format!("獲取設定失敗: {}", e))?;

        // 查找并更新指定prompt的current_state
        if let Some(prompt) = config.custom_prompt_config.prompts.iter_mut().find(|p| p.id == prompt_id) {
            prompt.current_state = new_state;
            prompt.updated_at = chrono::Utc::now().to_rfc3339();
        } else {
            return Err(format!("未找到ID为 {} 的prompt", prompt_id));
        }
    }

    // 儲存設定到檔案
    save_config(&state, &app)
        .await
        .map_err(|e| format!("儲存設定失敗: {}", e))?;

    Ok(())
}





/// 獲取設定檔案的真实路径
#[tauri::command]
pub async fn get_config_file_path(app: AppHandle) -> Result<String, String> {
    let config_path = crate::config::get_config_path(&app)
        .map_err(|e| format!("獲取設定檔案路径失敗: {}", e))?;

    // 獲取绝对路径
    let absolute_path = if config_path.is_absolute() {
        config_path
    } else {
        // 如果是相对路径，獲取当前工作目录并拼接
        std::env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
            .join(&config_path)
    };

    // 跨平台路径规范化
    let normalized_path = normalize_path_display(&absolute_path);

    Ok(normalized_path)
}

/// 跨平台路径显示规范化
fn normalize_path_display(path: &std::path::Path) -> String {
    // 如果檔案存在，尝试獲取规范路径
    let canonical_path = if path.exists() {
        match path.canonicalize() {
            Ok(canonical) => Some(canonical),
            Err(_) => None,
        }
    } else {
        None
    };

    let display_path = canonical_path.as_ref().map(|p| p.as_path()).unwrap_or(path);
    let path_str = display_path.to_string_lossy();

    // 處理不同平台的路径格式
    #[cfg(target_os = "windows")]
    {
        // Windows: 移除长路径前缀 \\?\
        if path_str.starts_with(r"\\?\") {
            path_str[4..].to_string()
        } else {
            path_str.to_string()
        }
    }

    #[cfg(target_os = "macos")]
    {
        // macOS: 處理可能的符号連結和特殊路径
        path_str.to_string()
    }

    #[cfg(target_os = "linux")]
    {
        // Linux: 標準Unix路径處理
        path_str.to_string()
    }

    #[cfg(target_os = "ios")]
    {
        // iOS: 类似macOS的處理
        path_str.to_string()
    }

    #[cfg(target_os = "android")]
    {
        // Android: 类似Linux的處理
        path_str.to_string()
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux", target_os = "ios", target_os = "android")))]
    {
        // 其他平台: 通用處理
        path_str.to_string()
    }
}

// 快捷键相关命令

/// 獲取快捷鍵設定
#[tauri::command]
pub async fn get_shortcut_config(state: State<'_, AppState>) -> Result<ShortcutConfig, String> {
    let config = state
        .config
        .lock()
        .map_err(|e| format!("獲取設定失敗: {}", e))?;
    Ok(config.shortcut_config.clone())
}

/// 更新快捷鍵綁定
#[tauri::command]
pub async fn update_shortcut_binding(
    shortcut_id: String,
    binding: ShortcutBinding,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<(), String> {
    {
        let mut config = state
            .config
            .lock()
            .map_err(|e| format!("獲取設定失敗: {}", e))?;

        // 更新指定的快捷鍵綁定
        config.shortcut_config.shortcuts.insert(shortcut_id, binding);
    }

    // 儲存設定到檔案
    save_config(&state, &app)
        .await
        .map_err(|e| format!("儲存設定失敗: {}", e))?;

    Ok(())
}



/// 重置快捷键为預設值
#[tauri::command]
pub async fn reset_shortcuts_to_default(
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<(), String> {
    {
        let mut config = state
            .config
            .lock()
            .map_err(|e| format!("獲取設定失敗: {}", e))?;
        config.shortcut_config = crate::config::default_shortcut_config();
    }

    // 儲存設定到檔案
    save_config(&state, &app)
        .await
        .map_err(|e| format!("儲存設定失敗: {}", e))?;

    Ok(())
}

/// 開啟 DevTools（調試用）
#[tauri::command]
pub async fn open_devtools(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.open_devtools();
        Ok(())
    } else {
        Err("找不到主視窗".to_string())
    }
}
