// 应用状态管理器
import { computed, reactive } from 'vue'

export type AppView = 'main-layout' | 'popup' | 'loading'

export interface AppState {
  currentView: AppView
  previousView: AppView | null
  isTransitioning: boolean
  mcpRequest: any
  showMcpPopup: boolean
}

export class AppStateManager {
  public state: AppState
  private transitionTimeout: number | null = null

  constructor() {
    this.state = reactive({
      currentView: 'main-layout' as AppView,
      previousView: null,
      isTransitioning: false,
      mcpRequest: null,
      showMcpPopup: false,
    })
  }

  // 切换到主界面
  switchToMainLayout(withTransition = true) {
    if (this.state.currentView === 'main-layout')
      return

    this.state.previousView = this.state.currentView

    if (withTransition) {
      this.state.isTransitioning = true
      this.state.currentView = 'loading'

      // 模拟加载时间
      this.transitionTimeout = window.setTimeout(() => {
        this.state.currentView = 'main-layout'
        this.state.showMcpPopup = false
        this.state.mcpRequest = null
        this.state.isTransitioning = false
      }, 300)
    }
    else {
      this.state.currentView = 'main-layout'
      this.state.showMcpPopup = false
      this.state.mcpRequest = null
    }
  }

  // 切换到弹窗
  switchToPopup(request: any, withTransition = true) {
    if (this.state.currentView === 'popup' && this.state.mcpRequest?.id === request?.id)
      return

    this.state.previousView = this.state.currentView
    this.state.mcpRequest = request

    if (withTransition) {
      this.state.isTransitioning = true
      this.state.currentView = 'loading'

      // 模拟加载时间
      this.transitionTimeout = window.setTimeout(() => {
        this.state.currentView = 'popup'
        this.state.showMcpPopup = true
        this.state.isTransitioning = false
      }, 300)
    }
    else {
      this.state.currentView = 'popup'
      this.state.showMcpPopup = true
    }
  }

  // 返回上一个视图
  goBack(withTransition = true) {
    if (this.state.previousView) {
      const targetView = this.state.previousView
      this.state.previousView = this.state.currentView

      if (targetView === 'main-layout') {
        this.switchToMainLayout(withTransition)
      }
      else if (targetView === 'popup' && this.state.mcpRequest) {
        this.switchToPopup(this.state.mcpRequest, withTransition)
      }
    }
  }

  // 清理资源
  cleanup() {
    if (this.transitionTimeout) {
      clearTimeout(this.transitionTimeout)
      this.transitionTimeout = null
    }
  }

  // 获取当前视图状态
  get isMainLayout() {
    return this.state.currentView === 'main-layout'
  }

  get isPopup() {
    return this.state.currentView === 'popup'
  }

  get isLoading() {
    return this.state.currentView === 'loading' || this.state.isTransitioning
  }

  get canGoBack() {
    return this.state.previousView !== null
  }
}

// 全局应用状态管理器实例
export const appStateManager = new AppStateManager()

// 组合式API
export function useAppState() {
  const state = computed(() => appStateManager.state)

  return {
    state,
    isMainLayout: computed(() => appStateManager.isMainLayout),
    isPopup: computed(() => appStateManager.isPopup),
    isLoading: computed(() => appStateManager.isLoading),
    canGoBack: computed(() => appStateManager.canGoBack),

    switchToMainLayout: (withTransition = true) => appStateManager.switchToMainLayout(withTransition),
    switchToPopup: (request: any, withTransition = true) => appStateManager.switchToPopup(request, withTransition),
    goBack: (withTransition = true) => appStateManager.goBack(withTransition),
    cleanup: () => appStateManager.cleanup(),

    manager: appStateManager,
  }
}

// 应用视图组件映射
export const viewComponents = {
  'main-layout': () => import('../components/layout/MainLayout.vue'),
  'popup': () => import('../components/popup/NewMcpPopup.vue'),
  'loading': () => import('../components/common/SkeletonLoader.vue'),
}

// 视图过渡配置
export const viewTransitions = {
  'main-layout': {
    name: 'main-layout',
    duration: 400,
    type: 'slide',
  },
  'popup': {
    name: 'popup',
    duration: 300,
    type: 'fade',
  },
  'loading': {
    name: 'skeleton',
    duration: 200,
    type: 'fade',
  },
} as const
