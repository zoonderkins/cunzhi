<script setup lang="ts">
import ThemeIcon from '../common/ThemeIcon.vue'

interface Props {
  currentTheme?: string
  loading?: boolean
}

interface Emits {
  themeChange: []
  openMainLayout: []
}

withDefaults(defineProps<Props>(), {
  currentTheme: 'dark',
  loading: false,
})

const emit = defineEmits<Emits>()

function handleThemeChange() {
  emit('themeChange')
}

function handleOpenMainLayout() {
  emit('openMainLayout')
}
</script>

<template>
  <div class="px-4 py-3 select-none">
    <div class="flex items-center justify-between">
      <!-- 左侧：标题 -->
      <div class="flex items-center gap-3">
        <div class="w-3 h-3 rounded-full bg-primary-500" />
        <h1 class="text-base font-medium text-theme-text">
          寸止 - 告别AI提前终止烦恼，助力AI更加持久
        </h1>
      </div>

      <!-- 右侧：操作按钮 -->
      <n-space size="small">
        <n-button
          size="small"
          quaternary
          circle
          :title="`切换到${currentTheme === 'light' ? '深色' : '浅色'}主题`"
          @click="handleThemeChange"
        >
          <template #icon>
            <ThemeIcon :theme="currentTheme" class="w-4 h-4" />
          </template>
        </n-button>
        <n-button
          size="small"
          quaternary
          circle
          title="打开主界面"
          @click="handleOpenMainLayout"
        >
          <template #icon>
            <div class="i-carbon-settings w-4 h-4" />
          </template>
        </n-button>
      </n-space>
    </div>
  </div>
</template>
