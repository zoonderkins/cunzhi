// 弹窗系統類型定义

export interface McpRequest {
  id: string
  message: string
  predefined_options?: string[]
  is_markdown?: boolean
}

// 自訂prompt類型定义
export interface CustomPrompt {
  id: string
  name: string
  content: string
  description?: string
  sort_order: number
  created_at: string
  updated_at: string
  type: 'normal' | 'conditional'
  // 條件性prompt專用欄位
  condition_text?: string // 條件描述文字
  template_true?: string // 開關為true時的範本
  template_false?: string // 開關為false時的範本
  current_state?: boolean // 目前開關狀態
}

// 自訂prompt設定
export interface CustomPromptConfig {
  prompts: CustomPrompt[]
  enabled: boolean
  maxPrompts: number
}

export interface PopupState {
  visible: boolean
  loading: boolean
  submitting: boolean
  currentTheme: string
  request: McpRequest | null
}

// 快捷键相关類型定义
export interface ShortcutConfig {
  shortcuts: Record<string, ShortcutBinding>
}

export interface ShortcutBinding {
  id: string
  name: string
  description: string
  action: string // "submit", "exit", "custom"
  key_combination: ShortcutKey
  enabled: boolean
  scope: string // "global", "popup", "input"
}

export interface ShortcutKey {
  key: string // 主键，如 "Enter", "Q", "F4"
  ctrl: boolean
  alt: boolean
  shift: boolean
  meta: boolean // macOS的Cmd键
}

// 新的结构化响应格式
export interface McpResponse {
  user_input: string | null
  selected_options: string[]
  images: ImageAttachment[]
  metadata: ResponseMetadata
}

export interface ImageAttachment {
  data: string
  media_type: string
  filename: string | null
}

export interface ResponseMetadata {
  timestamp: string | null
  request_id: string | null
  source: string | null
}

// 旧格式相容性支持
export interface PopupResponse {
  type: 'text' | 'image' | 'option'
  text?: string
  options?: string[]
  images?: ImageData[]
}

export interface ImageData {
  type: 'image'
  source: {
    type: 'base64'
    media_type: string
    data: string
  }
}

// 事件類型
export type PopupEvent
  = | { type: 'show', payload: McpRequest }
    | { type: 'hide' }
    | { type: 'submit', payload: PopupResponse }
    | { type: 'cancel' }
    | { type: 'theme-change', payload: string }
    | { type: 'open-main-layout' }
