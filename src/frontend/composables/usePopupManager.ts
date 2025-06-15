import type { IPopupManager, McpRequest, PopupConfig, PopupEvent, PopupResponse, PopupState } from '../types/popup'
// 弹窗管理系统
import { reactive, ref } from 'vue'

export class PopupManager implements IPopupManager {
  public state: PopupState
  public config: PopupConfig
  private eventListeners: Map<string, Function[]> = new Map()

  constructor(config: Partial<PopupConfig> = {}) {
    this.state = reactive({
      visible: false,
      loading: false,
      submitting: false,
      currentTheme: 'dark',
      request: null,
    })

    this.config = {
      enableTransitions: true,
      enableSkeletonLoader: true,
      enableMainLayoutButton: true,
      mockMode: false,
      ...config,
    }
  }

  // 显示弹窗
  show(request: McpRequest): void {
    this.state.request = request
    this.state.loading = true
    this.state.visible = true

    // 模拟加载延迟
    if (this.config.mockMode) {
      setTimeout(() => {
        this.state.loading = false
      }, 500)
    }
    else {
      this.state.loading = false
    }

    this.emit({ type: 'show', payload: request })
  }

  // 隐藏弹窗
  hide(): void {
    this.state.visible = false
    this.state.request = null
    this.state.loading = false
    this.state.submitting = false
    this.emit({ type: 'hide' })
  }

  // 提交响应
  submit(response: PopupResponse): void {
    if (this.state.submitting)
      return

    this.state.submitting = true
    this.emit({ type: 'submit', payload: response })

    // 模拟提交延迟
    setTimeout(() => {
      this.state.submitting = false
      if (this.config.mockMode) {
        this.hide()
      }
    }, 1000)
  }

  // 取消操作
  cancel(): void {
    this.emit({ type: 'cancel' })
    this.hide()
  }

  // 切换主题
  toggleTheme(): void {
    const themes = ['light', 'dark']
    const currentIndex = themes.indexOf(this.state.currentTheme)
    const nextIndex = (currentIndex + 1) % themes.length
    const newTheme = themes[nextIndex]

    this.state.currentTheme = newTheme

    // 立即应用主题到DOM
    document.documentElement.classList.remove('light', 'dark')
    document.documentElement.classList.add(newTheme)

    this.emit({ type: 'theme-change', payload: newTheme })
  }

  // 打开主界面
  openMainLayout(): void {
    this.emit({ type: 'open-main-layout' })
  }

  // 设置主题
  setTheme(theme: string): void {
    this.state.currentTheme = theme
    document.documentElement.classList.remove('light', 'dark')
    document.documentElement.classList.add(theme)
  }

  // 事件系统
  on(event: string, callback: Function): void {
    if (!this.eventListeners.has(event)) {
      this.eventListeners.set(event, [])
    }
    this.eventListeners.get(event)!.push(callback)
  }

  off(event: string, callback: Function): void {
    const listeners = this.eventListeners.get(event)
    if (listeners) {
      const index = listeners.indexOf(callback)
      if (index > -1) {
        listeners.splice(index, 1)
      }
    }
  }

  private emit(event: PopupEvent): void {
    const listeners = this.eventListeners.get(event.type)
    if (listeners) {
      listeners.forEach(callback => callback(event))
    }
  }

  // 创建模拟数据
  static createMockRequest(type: 'basic' | 'options' | 'complex' = 'basic'): McpRequest {
    const mockRequests = {
      basic: {
        id: 'mock-basic',
        message: '这是一个基础的模拟请求，用于测试弹窗功能。',
        is_markdown: false,
      },
      options: {
        id: 'mock-options',
        message: '请选择您需要的操作：',
        predefined_options: ['选项一', '选项二', '选项三'],
        is_markdown: false,
      },
      complex: {
        id: 'mock-complex',
        message: `# 复杂模拟请求

这是一个包含 **Markdown** 格式的复杂请求。

## 功能特性
- 支持 Markdown 渲染
- 支持代码高亮
- 支持预定义选项

\`\`\`javascript
console.log('Hello, World!');
\`\`\`

请选择您的操作：`,
        predefined_options: ['继续开发', '提交代码', '运行测试'],
        is_markdown: true,
      },
    }

    return mockRequests[type]
  }
}

// 全局弹窗管理器实例
export const popupManager = new PopupManager()
