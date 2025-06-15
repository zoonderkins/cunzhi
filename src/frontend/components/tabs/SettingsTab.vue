<script setup lang="ts">
import AudioSettings from '../settings/AudioSettings.vue'
import ReplySettings from '../settings/ReplySettings.vue'
import ThemeSettings from '../settings/ThemeSettings.vue'
import WindowSettings from '../settings/WindowSettings.vue'

interface Props {
  currentTheme: string
  alwaysOnTop: boolean
  audioNotificationEnabled: boolean
  audioUrl: string
}

interface Emits {
  themeChange: [theme: string]
  toggleAlwaysOnTop: []
  toggleAudioNotification: []
  updateAudioUrl: [url: string]
  testAudio: []
  stopAudio: []
  testAudioError: [error: any]
  updateWindowSize: [size: { width: number, height: number, fixed: boolean }]
  updateWindowMode: [fixed: boolean]
}

defineProps<Props>()
const emit = defineEmits<Emits>()

// 处理窗口尺寸更新
function handleWindowSizeUpdate(size: { width: number, height: number, fixed: boolean }) {
  emit('updateWindowSize', size)
}
</script>

<template>
  <div class="max-w-3xl mx-auto tab-content">
    <n-space vertical size="large">
      <!-- 主题设置组件 -->
      <ThemeSettings :current-theme="currentTheme" @theme-change="$emit('themeChange', $event)" />

      <!-- 继续回复设置组件 -->
      <ReplySettings />

      <!-- 窗口设置组件 -->
      <WindowSettings
        :always-on-top="alwaysOnTop"
        :window-width="600"
        :window-height="900"
        :fixed-window-size="false"
        @toggle-always-on-top="$emit('toggleAlwaysOnTop')"
        @update-window-size="handleWindowSizeUpdate"
      />

      <!-- 音效设置组件 -->
      <AudioSettings
        :audio-notification-enabled="audioNotificationEnabled"
        :audio-url="audioUrl"
        @toggle-audio-notification="$emit('toggleAudioNotification')"
        @update-audio-url="$emit('updateAudioUrl', $event)"
        @test-audio="$emit('testAudio')"
        @stop-audio="$emit('stopAudio')"
        @test-audio-error="$emit('testAudioError', $event)"
      />
    </n-space>
  </div>
</template>
