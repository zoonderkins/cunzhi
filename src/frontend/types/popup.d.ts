// 弹窗系统类型定义

export interface McpRequest {
  id: string
  message: string
  predefined_options?: string[]
  is_markdown?: boolean
}

export interface PopupState {
  visible: boolean
  loading: boolean
  submitting: boolean
  currentTheme: string
  request: McpRequest | null
}

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

export interface PopupTransition {
  name: string
  duration: number
  enterFrom: string
  enterTo: string
  leaveFrom: string
  leaveTo: string
}

export interface PopupConfig {
  enableTransitions: boolean
  enableSkeletonLoader: boolean
  enableMainLayoutButton: boolean
  mockMode: boolean
}

// 弹窗管理器接口
export interface IPopupManager {
  state: PopupState
  config: PopupConfig
  show: (request: McpRequest) => void
  hide: () => void
  submit: (response: PopupResponse) => void
  cancel: () => void
  toggleTheme: () => void
  openMainLayout: () => void
}

// 事件类型
export type PopupEvent
  = | { type: 'show', payload: McpRequest }
    | { type: 'hide' }
    | { type: 'submit', payload: PopupResponse }
    | { type: 'cancel' }
    | { type: 'theme-change', payload: string }
    | { type: 'open-main-layout' }
