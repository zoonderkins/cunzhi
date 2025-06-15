use anyhow::Result;
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use teloxide::prelude::*;
use tokio::sync::Mutex;

use super::core::{handle_callback_query, handle_text_message, TelegramCore, TelegramEvent};

/// Telegram集成管理器
pub struct TelegramIntegration {
    core: TelegramCore,
    app_handle: AppHandle,
    /// 当前选中的选项
    selected_options: Arc<Mutex<Vec<String>>>,
    /// 用户输入文本
    user_input: Arc<Mutex<String>>,
    /// 操作消息ID，用于过滤后续消息
    operation_message_id: Arc<Mutex<Option<i32>>>,
    /// 停止信号发送器
    stop_sender: Option<tokio::sync::oneshot::Sender<()>>,
}

impl TelegramIntegration {
    /// 创建新的Telegram集成实例
    pub fn new(bot_token: String, chat_id: String, app_handle: AppHandle) -> Result<Self> {
        let core = TelegramCore::new(bot_token, chat_id)?;

        Ok(Self {
            core,
            app_handle,
            selected_options: Arc::new(Mutex::new(Vec::new())),
            user_input: Arc::new(Mutex::new(String::new())),
            operation_message_id: Arc::new(Mutex::new(None)),
            stop_sender: None,
        })
    }

    /// 发送MCP请求消息到Telegram
    pub async fn send_mcp_request(
        &mut self,
        message: &str,
        predefined_options: Vec<String>,
        is_markdown: bool,
        continue_reply_enabled: bool,
    ) -> Result<()> {
        // 初始化选中选项状态
        {
            let mut selected = self.selected_options.lock().await;
            selected.clear();
        }

        // 发送选项消息
        self.core
            .send_options_message(message, &predefined_options, is_markdown)
            .await?;

        // 发送操作消息
        let op_msg_id = self
            .core
            .send_operation_message(continue_reply_enabled)
            .await?;

        // 保存操作消息ID
        {
            let mut op_id = self.operation_message_id.lock().await;
            *op_id = Some(op_msg_id);
        }

        // 启动消息监听
        self.start_message_listener().await?;

        Ok(())
    }

    /// 发送选项消息（消息一）
    async fn send_options_message(
        &self,
        message: &str,
        predefined_options: &[String],
        is_markdown: bool,
    ) -> Result<()> {
        // 处理消息内容
        let processed_message = if is_markdown {
            process_telegram_markdown(message)
        } else {
            message.to_string()
        };

        // 创建inline keyboard
        let mut keyboard_rows = Vec::new();

        // 添加选项按钮（每行最多2个）
        for chunk in predefined_options.chunks(2) {
            let mut row = Vec::new();
            for option in chunk {
                let callback_data = format!("toggle:{}", option);
                row.push(InlineKeyboardButton::callback(
                    format!("☐ {}", option),
                    callback_data,
                ));
            }
            keyboard_rows.push(row);
        }

        let inline_keyboard = InlineKeyboardMarkup::new(keyboard_rows);

        // 发送消息
        let mut send_request = self
            .bot
            .send_message(self.chat_id, processed_message)
            .reply_markup(inline_keyboard);

        // 如果是Markdown，设置解析模式
        if is_markdown {
            send_request = send_request.parse_mode(ParseMode::MarkdownV2);
        }

        match send_request.await {
            Ok(_msg) => {
                // 消息发送成功
            }
            Err(e) => {
                return Err(anyhow::anyhow!("发送选项消息失败: {}", e));
            }
        }

        Ok(())
    }

    /// 发送操作消息（消息二）
    async fn send_operation_message(&self, continue_reply_enabled: bool) -> Result<()> {
        // 创建reply keyboard
        let mut keyboard_buttons = vec![KeyboardButton::new("↗️发送")];

        if continue_reply_enabled {
            keyboard_buttons.insert(0, KeyboardButton::new("⏩继续"));
        }

        let reply_keyboard = KeyboardMarkup::new(vec![keyboard_buttons])
            .resize_keyboard(true)
            .one_time_keyboard(false);

        // 发送操作消息
        let operation_message = "键盘上选择操作完成对话";

        match self
            .bot
            .send_message(self.chat_id, operation_message)
            .reply_markup(reply_keyboard)
            .await
        {
            Ok(msg) => {
                // 保存操作消息ID用于过滤
                {
                    let mut op_msg_id = self.operation_message_id.lock().await;
                    *op_msg_id = Some(msg.id.0);
                }
            }
            Err(e) => {
                return Err(anyhow::anyhow!("发送操作消息失败: {}", e));
            }
        }

        Ok(())
    }

    /// 启动消息监听
    async fn start_message_listener(&mut self) -> Result<()> {
        let bot = self.bot.clone();
        let chat_id = self.chat_id;
        let app_handle = self.app_handle.clone();
        let selected_options = self.selected_options.clone();
        let user_input = self.user_input.clone();
        let operation_message_id = self.operation_message_id.clone();

        let (stop_tx, mut stop_rx) = tokio::sync::oneshot::channel();
        self.stop_sender = Some(stop_tx);

        // 启动监听任务
        tokio::spawn(async move {
            let mut offset = 0i32;

            loop {
                tokio::select! {
                    _ = &mut stop_rx => {
                        break;
                    }
                    _ = tokio::time::sleep(tokio::time::Duration::from_millis(1000)) => {
                        // 轮询获取更新
                        match bot.get_updates().offset(offset).await {
                            Ok(updates) => {
                                for update in updates {
                                    offset = update.id.0 as i32 + 1;

                                    // 处理不同类型的更新
                                    match update.kind {
                                        teloxide::types::UpdateKind::CallbackQuery(callback_query) => {
                                            // 使用核心模块的处理函数
                                            match handle_callback_query(
                                                &bot,
                                                &callback_query,
                                                chat_id,
                                            ).await {
                                                Ok(Some(option)) => {

                                                    // 切换选项状态
                                                    let selected = {
                                                        let mut selected_opts = selected_options.lock().await;
                                                        if selected_opts.contains(&option) {
                                                            selected_opts.retain(|x| x != &option);
                                                            false
                                                        } else {
                                                            selected_opts.push(option.clone());
                                                            true
                                                        }
                                                    };

                                                    // 发送更新后的事件到前端
                                                    let event = TelegramEvent::OptionToggled {
                                                        option: option.clone(),
                                                        selected,
                                                    };

                                                    if let Err(e) = app_handle.emit("telegram-event", &event) {
                                                        eprintln!("Telegram事件发送失败: {}", e);
                                                    }
                                                }
                                                Ok(None) => {
                                                    // CallbackQuery 被过滤或忽略
                                                }
                                                Err(e) => {
                                                    eprintln!("CallbackQuery处理失败: {}", e);
                                                }
                                            }
                                        }
                                        teloxide::types::UpdateKind::Message(message) => {
                                            // 获取操作消息ID
                                            let op_msg_id = {
                                                let op_id = operation_message_id.lock().await;
                                                *op_id
                                            };

                                            // 使用核心模块的处理函数
                                            match handle_text_message(
                                                &message,
                                                chat_id,
                                                op_msg_id,
                                            ).await {
                                                Ok(Some(event)) => {
                                                    // 如果是文本更新，保存到用户输入
                                                    if let TelegramEvent::TextUpdated { text } = &event {
                                                        let mut input = user_input.lock().await;
                                                        *input = text.clone();
                                                    }

                                                    // 发送事件到前端
                                                    if let Err(e) = app_handle.emit("telegram-event", &event) {
                                                        eprintln!("Telegram文本事件发送失败: {}", e);
                                                    }
                                                }
                                                Ok(None) => {
                                                    // 文本消息被过滤或忽略
                                                }
                                                Err(e) => {
                                                    eprintln!("文本消息处理失败: {}", e);
                                                }
                                            }
                                        }
                                        _ => {
                                            // 忽略其他类型的更新
                                        }
                                    }
                                }
                            }
                            Err(_e) => {
                                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }

    /// 获取当前选中的选项
    pub async fn get_selected_options(&self) -> Vec<String> {
        let selected = self.selected_options.lock().await;
        selected.clone()
    }

    /// 获取用户输入的文本
    pub async fn get_user_input(&self) -> String {
        let input = self.user_input.lock().await;
        input.clone()
    }

    /// 停止Telegram集成
    pub async fn stop(&mut self) {
        if let Some(sender) = self.stop_sender.take() {
            let _ = sender.send(());
        }
    }
}

/// 处理callback query（inline keyboard点击）
async fn handle_callback_query(
    bot: Bot,
    q: CallbackQuery,
    chat_id: ChatId,
    app_handle: AppHandle,
    selected_options: Arc<Mutex<Vec<String>>>,
) -> ResponseResult<()> {
    // 检查是否是目标聊天
    if let Some(message) = &q.message {
        if message.chat.id != chat_id {
            return Ok(());
        }
    }

    if let Some(data) = q.data {
        if data.starts_with("toggle:") {
            let option = data.strip_prefix("toggle:").unwrap().to_string();

            // 切换选项状态
            let selected = {
                let mut selected_opts = selected_options.lock().await;
                if selected_opts.contains(&option) {
                    selected_opts.retain(|x| x != &option);
                    false
                } else {
                    selected_opts.push(option.clone());
                    true
                }
            };

            // 发送事件到前端
            let event = TelegramEvent::OptionToggled {
                option: option.clone(),
                selected,
            };
            let _ = app_handle.emit("telegram-event", &event);

            // 更新按钮文本
            if let Some(_message) = q.message {
                // 这里需要更新keyboard中对应按钮的文本
                // 由于teloxide的限制，我们简化处理，只回答callback query
            }
        }
    }

    // 回答callback query
    bot.answer_callback_query(q.id).await?;
    Ok(())
}

/// 处理文本消息
async fn handle_message(
    _bot: Bot,
    msg: Message,
    chat_id: ChatId,
    app_handle: AppHandle,
    user_input: Arc<Mutex<String>>,
    operation_message_id: Arc<Mutex<Option<i32>>>,
) -> ResponseResult<()> {
    // 检查是否是目标聊天
    if msg.chat.id != chat_id {
        return Ok(());
    }

    // 检查消息ID过滤
    let op_msg_id = {
        let op_id = operation_message_id.lock().await;
        *op_id
    };

    if let Some(op_id) = op_msg_id {
        if msg.id.0 <= op_id {
            return Ok(());
        }
    }

    if let Some(text) = msg.text() {
        match text {
            "⏩继续" => {
                let event = TelegramEvent::ContinuePressed;
                let _ = app_handle.emit("telegram-event", &event);
            }
            "↗️发送" => {
                let event = TelegramEvent::SendPressed;
                let _ = app_handle.emit("telegram-event", &event);
            }
            _ => {
                // 更新用户输入
                {
                    let mut input = user_input.lock().await;
                    *input = text.to_string();
                }

                // 发送事件到前端
                let event = TelegramEvent::TextUpdated {
                    text: text.to_string(),
                };
                let _ = app_handle.emit("telegram-event", &event);
            }
        }
    }

    Ok(())
}
