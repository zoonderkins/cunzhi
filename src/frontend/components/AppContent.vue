<script setup lang="ts">
import { useMessage } from 'naive-ui'
import { onMounted } from 'vue'
import MainLayout from './layout/MainLayout.vue'
import McpPopup from './popup/McpPopup.vue'

interface Props {
  mcpRequest: any
  showMcpPopup: boolean
  currentTheme: string
  alwaysOnTop: boolean
  audioNotificationEnabled: boolean
  audioUrl: string
  isInitializing: boolean
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
  updateWindowSize: [size: { width: number, height: number, fixed: boolean }]
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
  <div class="min-h-screen bg-black">
    <!-- MCP弹窗 -->
    <McpPopup
      v-if="showMcpPopup && mcpRequest"
      :request="mcpRequest"
      :current-theme="currentTheme"
      @response="$emit('mcpResponse', $event)"
      @cancel="$emit('mcpCancel')"
      @theme-change="$emit('themeChange', $event)"
    />

    <!-- 弹窗加载骨架屏 或 初始化骨架屏 -->
    <div v-else-if="showMcpPopup || isInitializing" class="flex flex-col w-full h-screen bg-black text-white">
      <!-- 头部骨架 -->
      <div class="flex-shrink-0 bg-gray-100 border-b-2 border-gray-200 px-4 py-3">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <n-skeleton circle :width="12" :height="12" />
            <n-skeleton text :width="256" />
          </div>
          <div class="flex gap-2">
            <n-skeleton circle :width="32" :height="32" />
            <n-skeleton circle :width="32" :height="32" />
          </div>
        </div>
      </div>

      <!-- 内容骨架 -->
      <div class="flex-1 p-4">
        <div class="bg-gray-100 rounded-lg p-4 mb-4">
          <n-skeleton text :repeat="3" />
        </div>

        <div class="space-y-3">
          <n-skeleton text :width="128" />
          <n-skeleton text :repeat="3" />
        </div>
      </div>

      <!-- 底部骨架 -->
      <div class="flex-shrink-0 bg-gray-100 border-t-2 border-gray-200 p-4">
        <div class="flex justify-between items-center">
          <n-skeleton text :width="96" />
          <div class="flex gap-2">
            <n-skeleton text :width="64" :height="32" />
            <n-skeleton text :width="64" :height="32" />
          </div>
        </div>
      </div>
    </div>

    <!-- 主界面 - 只在非弹窗模式且非初始化时显示 -->
    <MainLayout
      v-else :current-theme="currentTheme" :always-on-top="alwaysOnTop"
      :audio-notification-enabled="audioNotificationEnabled" :audio-url="audioUrl"
      @theme-change="$emit('themeChange', $event)" @toggle-always-on-top="$emit('toggleAlwaysOnTop')"
      @toggle-audio-notification="$emit('toggleAudioNotification')" @update-audio-url="$emit('updateAudioUrl', $event)"
      @test-audio="$emit('testAudio')" @stop-audio="$emit('stopAudio')"
      @test-audio-error="$emit('testAudioError', $event)"
    />
  </div>
</template>
