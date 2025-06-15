import type { McpRequest, PopupEvent, PopupResponse } from '../types/popup'
// 弹窗系统组合式API
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { PopupManager } from './usePopupManager'

export function usePopup(config?: {
  mockMode?: boolean
  enableTransitions?: boolean
  enableSkeletonLoader?: boolean
  enableMainLayoutButton?: boolean
}) {
  // 创建弹窗管理器实例
  const manager = new PopupManager(config)

  // 响应式状态
  const state = computed(() => manager.state)
  const isVisible = computed(() => manager.state.visible)
  const isLoading = computed(() => manager.state.loading)
  const isSubmitting = computed(() => manager.state.submitting)
  const currentTheme = computed(() => manager.state.currentTheme)
  const currentRequest = computed(() => manager.state.request)

  // 事件处理器
  const eventHandlers = ref<Map<string, Function[]>>(new Map())

  // 显示弹窗
  function showPopup(request: McpRequest) {
    manager.show(request)
  }

  // 隐藏弹窗
  function hidePopup() {
    manager.hide()
  }

  // 提交响应
  function submitResponse(response: PopupResponse) {
    manager.submit(response)
  }

  // 取消操作
  function cancelPopup() {
    manager.cancel()
  }

  // 切换主题
  function toggleTheme() {
    manager.toggleTheme()
  }

  // 设置主题
  function setTheme(theme: string) {
    manager.setTheme(theme)
  }

  // 打开主界面
  function openMainLayout() {
    manager.openMainLayout()
  }

  // 事件监听
  function onPopupEvent(event: string, callback: Function) {
    manager.on(event, callback)

    // 记录事件处理器以便清理
    if (!eventHandlers.value.has(event)) {
      eventHandlers.value.set(event, [])
    }
    eventHandlers.value.get(event)!.push(callback)
  }

  // 移除事件监听
  function offPopupEvent(event: string, callback: Function) {
    manager.off(event, callback)

    const handlers = eventHandlers.value.get(event)
    if (handlers) {
      const index = handlers.indexOf(callback)
      if (index > -1) {
        handlers.splice(index, 1)
      }
    }
  }

  // 创建模拟请求
  function createMockRequest(type: 'basic' | 'options' | 'complex' = 'basic') {
    return PopupManager.createMockRequest(type)
  }

  // 快捷方法：显示模拟弹窗
  function showMockPopup(type: 'basic' | 'options' | 'complex' = 'basic') {
    const mockRequest = createMockRequest(type)
    showPopup(mockRequest)
  }

  // 生命周期管理
  onUnmounted(() => {
    // 清理所有事件监听器
    eventHandlers.value.forEach((handlers, event) => {
      handlers.forEach((handler) => {
        manager.off(event, handler)
      })
    })
    eventHandlers.value.clear()
  })

  return {
    // 状态
    state,
    isVisible,
    isLoading,
    isSubmitting,
    currentTheme,
    currentRequest,

    // 方法
    showPopup,
    hidePopup,
    submitResponse,
    cancelPopup,
    toggleTheme,
    setTheme,
    openMainLayout,

    // 事件
    onPopupEvent,
    offPopupEvent,

    // 模拟数据
    createMockRequest,
    showMockPopup,

    // 管理器实例（高级用法）
    manager,
  }
}

// 全局弹窗实例（单例模式）
let globalPopupInstance: ReturnType<typeof usePopup> | null = null

export function useGlobalPopup(config?: Parameters<typeof usePopup>[0]) {
  if (!globalPopupInstance) {
    globalPopupInstance = usePopup(config)
  }
  return globalPopupInstance
}

// 弹窗工具函数
export const popupUtils = {
  // 快速显示确认弹窗
  confirm(message: string, options?: string[]) {
    const popup = useGlobalPopup()
    const request: McpRequest = {
      id: `confirm-${Date.now()}`,
      message,
      predefined_options: options || ['确认', '取消'],
      is_markdown: false,
    }
    popup.showPopup(request)
    return popup
  },

  // 快速显示信息弹窗
  info(message: string, isMarkdown = false) {
    const popup = useGlobalPopup()
    const request: McpRequest = {
      id: `info-${Date.now()}`,
      message,
      is_markdown: isMarkdown,
    }
    popup.showPopup(request)
    return popup
  },

  // 快速显示输入弹窗
  prompt(message: string, placeholder?: string) {
    const popup = useGlobalPopup()
    const request: McpRequest = {
      id: `prompt-${Date.now()}`,
      message: message + (placeholder ? `\n\n请输入${placeholder}：` : ''),
      is_markdown: false,
    }
    popup.showPopup(request)
    return popup
  },
}
