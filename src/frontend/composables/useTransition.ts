// 过渡动画组合式API
import { computed, onMounted, ref } from 'vue'
import { TRANSITION_DURATIONS, TRANSITION_EASINGS } from '../constants/ui'

export interface TransitionConfig {
  name: string
  duration: number
  type: 'fade' | 'slide' | 'scale' | 'blur'
  easing: string
  mode: 'out-in' | 'in-out' | 'default'
}

export function useTransition(config: Partial<TransitionConfig> = {}) {
  const defaultConfig: TransitionConfig = {
    name: 'popup',
    duration: TRANSITION_DURATIONS.normal,
    type: 'fade',
    easing: TRANSITION_EASINGS.cubic,
    mode: 'out-in',
  }

  const transitionConfig = ref<TransitionConfig>({ ...defaultConfig, ...config })
  const isTransitioning = ref(false)
  const prefersReducedMotion = ref(false)

  // 检测用户是否偏好减少动画
  onMounted(() => {
    const mediaQuery = window.matchMedia('(prefers-reduced-motion: reduce)')
    prefersReducedMotion.value = mediaQuery.matches

    mediaQuery.addEventListener('change', (e) => {
      prefersReducedMotion.value = e.matches
    })
  })

  // 计算过渡类名
  const transitionClasses = computed(() => {
    const base = transitionConfig.value.name
    return {
      enter: `${base}-enter`,
      enterActive: `${base}-enter-active`,
      enterTo: `${base}-enter-to`,
      leave: `${base}-leave`,
      leaveActive: `${base}-leave-active`,
      leaveTo: `${base}-leave-to`,
    }
  })

  // 计算过渡样式
  const transitionStyles = computed(() => {
    if (prefersReducedMotion.value) {
      return {
        transition: 'none !important',
        animation: 'none !important',
      }
    }

    return {
      transition: `all ${transitionConfig.value.duration}ms ${transitionConfig.value.easing}`,
    }
  })

  // 过渡事件处理
  function onBeforeEnter() {
    isTransitioning.value = true
  }

  function onAfterEnter() {
    isTransitioning.value = false
  }

  function onBeforeLeave() {
    isTransitioning.value = true
  }

  function onAfterLeave() {
    isTransitioning.value = false
  }

  // 更新配置
  function updateConfig(newConfig: Partial<TransitionConfig>) {
    transitionConfig.value = { ...transitionConfig.value, ...newConfig }
  }

  // 预设过渡配置
  const presets = {
    fade: {
      type: 'fade' as const,
      duration: TRANSITION_DURATIONS.normal,
      easing: TRANSITION_EASINGS.cubic,
    },
    slide: {
      type: 'slide' as const,
      duration: 350,
      easing: TRANSITION_EASINGS.cubic,
    },
    scale: {
      type: 'scale' as const,
      duration: 400,
      easing: TRANSITION_EASINGS.bounce,
    },
    blur: {
      type: 'blur' as const,
      duration: 450,
      easing: TRANSITION_EASINGS.cubic,
    },
    quick: {
      duration: TRANSITION_DURATIONS.quick,
      easing: TRANSITION_EASINGS.easeOut,
    },
    smooth: {
      duration: TRANSITION_DURATIONS.smooth,
      easing: TRANSITION_EASINGS.easeInOut,
    },
  }

  // 应用预设
  function applyPreset(preset: keyof typeof presets) {
    updateConfig(presets[preset])
  }

  return {
    // 状态
    transitionConfig,
    isTransitioning,
    prefersReducedMotion,

    // 计算属性
    transitionClasses,
    transitionStyles,

    // 方法
    updateConfig,
    applyPreset,

    // 事件处理器
    onBeforeEnter,
    onAfterEnter,
    onBeforeLeave,
    onAfterLeave,

    // 预设
    presets,
  }
}

// 全局过渡管理器
class TransitionManager {
  private transitions = new Map<string, ReturnType<typeof useTransition>>()

  register(name: string, config?: Partial<TransitionConfig>) {
    if (!this.transitions.has(name)) {
      this.transitions.set(name, useTransition({ ...config, name }))
    }
    return this.transitions.get(name)!
  }

  get(name: string) {
    return this.transitions.get(name)
  }

  remove(name: string) {
    this.transitions.delete(name)
  }

  clear() {
    this.transitions.clear()
  }
}

export const transitionManager = new TransitionManager()

// 预注册常用过渡
transitionManager.register('popup', { type: 'fade', duration: TRANSITION_DURATIONS.normal })
transitionManager.register('main-layout', { type: 'slide', duration: 400 })
transitionManager.register('content', { type: 'fade', duration: 250 })
transitionManager.register('skeleton', { type: 'fade', duration: TRANSITION_DURATIONS.quick })
