<script setup lang="ts">
import { useMessage } from 'naive-ui'
import { onMounted } from 'vue'
import MainLayout from './layout/MainLayout.vue'
import McpPopup from './McpPopup.vue'

interface Props {
  mcpRequest: any
  showMcpPopup: boolean
  currentTheme: string
  alwaysOnTop: boolean
  audioNotificationEnabled: boolean
  audioUrl: string
}

interface Emits {
  mcpResponse: [response: any]
  mcpCancel: []
  themeChange: [theme: string]
  toggleAlwaysOnTop: []
  toggleAudioNotification: []
  updateAudioUrl: [url: string]
  testAudio: []
  stopAudio: []
  testAudioError: [error: any]
  updateWindowSize: [size: { width: number; height: number; fixed: boolean }]
  messageReady: [message: any]
}

defineProps<Props>()
const emit = defineEmits<Emits>()

// 初始化 Naive UI 消息实例
const message = useMessage()

onMounted(() => {
  // 将消息实例传递给父组件
  emit('messageReady', message)
})
</script>

<template>
  <div id="app" class="min-h-screen" style="background-color: var(--body-color)">
    <!-- MCP弹窗 -->
    <McpPopup
      v-if="showMcpPopup && mcpRequest" :request="mcpRequest" :current-theme="currentTheme"
      @response="$emit('mcpResponse', $event)" @cancel="$emit('mcpCancel')"
      @theme-change="$emit('themeChange', $event)"
    />

    <!-- 主界面 -->
    <MainLayout
      v-else :current-theme="currentTheme" :always-on-top="alwaysOnTop"
      :audio-notification-enabled="audioNotificationEnabled" :audio-url="audioUrl"
      @theme-change="$emit('themeChange', $event)" @toggle-always-on-top="$emit('toggleAlwaysOnTop')"
      @toggle-audio-notification="$emit('toggleAudioNotification')" @update-audio-url="$emit('updateAudioUrl', $event)"
      @test-audio="$emit('testAudio')" @stop-audio="$emit('stopAudio')"
      @test-audio-error="$emit('testAudioError', $event)"
      @update-window-size="$emit('updateWindowSize', $event)"
    />
  </div>
</template>
