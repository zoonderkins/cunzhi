<script setup lang="ts">
interface Props {
  name?: string
  mode?: 'out-in' | 'in-out' | 'default'
  duration?: number
  type?: 'fade' | 'slide' | 'scale' | 'blur'
}

const props = withDefaults(defineProps<Props>(), {
  name: 'popup',
  mode: 'out-in',
  duration: 300,
  type: 'fade',
})

// 导入computed
import { computed } from 'vue'

// 根据类型生成过渡类名
const _transitionClasses = computed(() => {
  const base = props.name
  return {
    enter: `${base}-enter`,
    enterActive: `${base}-enter-active`,
    enterTo: `${base}-enter-to`,
    leave: `${base}-leave`,
    leaveActive: `${base}-leave-active`,
    leaveTo: `${base}-leave-to`,
  }
})
</script>

<template>
  <Transition
    :name="name"
    :mode="mode"
    :duration="duration"
    appear
  >
    <slot />
  </Transition>
</template>

<style scoped>
/* 淡入淡出过渡 */
.popup-enter-active,
.popup-leave-active {
  transition: all 0.3s cubic-bezier(0.25, 0.8, 0.25, 1);
}

.popup-enter-from,
.popup-leave-to {
  opacity: 0;
  transform: scale(0.95);
}

.popup-enter-to,
.popup-leave-from {
  opacity: 1;
  transform: scale(1);
}

/* 滑动过渡 */
.slide-enter-active,
.slide-leave-active {
  transition: all 0.3s cubic-bezier(0.25, 0.8, 0.25, 1);
}

.slide-enter-from {
  opacity: 0;
  transform: translateY(-20px);
}

.slide-leave-to {
  opacity: 0;
  transform: translateY(20px);
}

.slide-enter-to,
.slide-leave-from {
  opacity: 1;
  transform: translateY(0);
}

/* 缩放过渡 */
.scale-enter-active,
.scale-leave-active {
  transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.scale-enter-from,
.scale-leave-to {
  opacity: 0;
  transform: scale(0.8);
}

.scale-enter-to,
.scale-leave-from {
  opacity: 1;
  transform: scale(1);
}

/* 模糊过渡 */
.blur-enter-active,
.blur-leave-active {
  transition: all 0.4s cubic-bezier(0.25, 0.8, 0.25, 1);
}

.blur-enter-from,
.blur-leave-to {
  opacity: 0;
  filter: blur(10px);
  transform: scale(1.05);
}

.blur-enter-to,
.blur-leave-from {
  opacity: 1;
  filter: blur(0px);
  transform: scale(1);
}

/* 主界面过渡 */
.main-layout-enter-active,
.main-layout-leave-active {
  transition: all 0.4s cubic-bezier(0.25, 0.8, 0.25, 1);
}

.main-layout-enter-from {
  opacity: 0;
  transform: translateY(30px) scale(0.95);
}

.main-layout-leave-to {
  opacity: 0;
  transform: translateY(-30px) scale(1.05);
}

.main-layout-enter-to,
.main-layout-leave-from {
  opacity: 1;
  transform: translateY(0) scale(1);
}

/* 内容切换过渡 */
.content-enter-active,
.content-leave-active {
  transition: all 0.25s ease-in-out;
}

.content-enter-from {
  opacity: 0;
  transform: translateX(20px);
}

.content-leave-to {
  opacity: 0;
  transform: translateX(-20px);
}

.content-enter-to,
.content-leave-from {
  opacity: 1;
  transform: translateX(0);
}

/* 骨架屏过渡 */
.skeleton-enter-active,
.skeleton-leave-active {
  transition: all 0.2s ease-in-out;
}

.skeleton-enter-from,
.skeleton-leave-to {
  opacity: 0;
}

.skeleton-enter-to,
.skeleton-leave-from {
  opacity: 1;
}

/* 响应式优化 */
@media (prefers-reduced-motion: reduce) {
  .popup-enter-active,
  .popup-leave-active,
  .slide-enter-active,
  .slide-leave-active,
  .scale-enter-active,
  .scale-leave-active,
  .blur-enter-active,
  .blur-leave-active,
  .main-layout-enter-active,
  .main-layout-leave-active,
  .content-enter-active,
  .content-leave-active,
  .skeleton-enter-active,
  .skeleton-leave-active {
    transition: none !important;
  }

  .popup-enter-from,
  .popup-leave-to,
  .slide-enter-from,
  .slide-leave-to,
  .scale-enter-from,
  .scale-leave-to,
  .blur-enter-from,
  .blur-leave-to,
  .main-layout-enter-from,
  .main-layout-leave-to,
  .content-enter-from,
  .content-leave-to {
    transform: none !important;
    filter: none !important;
  }
}
</style>
