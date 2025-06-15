<script setup lang="ts">
import { useMessage } from 'naive-ui'
import { onMounted, ref } from 'vue'
import MainLayout from './layout/MainLayout.vue'
import McpPopup from './popup/McpPopup.vue'
import PopupHeader from './popup/PopupHeader.vue'

interface AppConfig {
  theme: string
  window: {
    alwaysOnTop: boolean
    width: number
    height: number
    fixed: boolean
  }
  audio: {
    enabled: boolean
    url: string
  }
  reply: {
    enabled: boolean
    prompt: string
  }
}

interface Props {
  mcpRequest: any
  showMcpPopup: boolean
  appConfig: AppConfig
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
  updateReplyConfig: [config: { enable_continue_reply?: boolean, continue_prompt?: string }]
  messageReady: [message: any]
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

// 弹窗中的设置显示控制
const showPopupSettings = ref(false)

// 初始化 Naive UI 消息实例
const message = useMessage()

// 切换弹窗设置显示
function togglePopupSettings() {
  showPopupSettings.value = !showPopupSettings.value
}

onMounted(() => {
  // 将消息实例传递给父组件
  emit('messageReady', message)
})
</script>

<template>
  <div class="min-h-screen bg-black">
    <!-- MCP弹窗模式 -->
    <div v-if="props.showMcpPopup && props.mcpRequest" class="flex flex-col w-full h-screen bg-black text-white select-none">
      <!-- 头部 - 固定在顶部 -->
      <div class="sticky top-0 z-50 flex-shrink-0 bg-gray-100 border-b-2 border-gray-200">
        <PopupHeader
          :current-theme="props.appConfig.theme"
          :loading="false"
          :show-main-layout="showPopupSettings"
          @theme-change="$emit('themeChange', $event)"
          @open-main-layout="togglePopupSettings"
        />
      </div>

      <!-- 设置界面 -->
      <div v-if="showPopupSettings" class="flex-1 overflow-y-auto">
        <MainLayout
          :current-theme="props.appConfig.theme"
          :always-on-top="props.appConfig.window.alwaysOnTop"
          :audio-notification-enabled="props.appConfig.audio.enabled"
          :audio-url="props.appConfig.audio.url"
          :window-width="props.appConfig.window.width"
          :window-height="props.appConfig.window.height"
          :fixed-window-size="props.appConfig.window.fixed"
          @theme-change="$emit('themeChange', $event)"
          @toggle-always-on-top="$emit('toggleAlwaysOnTop')"
          @toggle-audio-notification="$emit('toggleAudioNotification')"
          @update-audio-url="$emit('updateAudioUrl', $event)"
          @test-audio="$emit('testAudio')"
          @stop-audio="$emit('stopAudio')"
          @test-audio-error="$emit('testAudioError', $event)"
          @update-window-size="$emit('updateWindowSize', $event)"
        />
      </div>

      <!-- 弹窗内容 -->
      <McpPopup
        v-else
        :request="props.mcpRequest"
        :app-config="props.appConfig"
        @response="$emit('mcpResponse', $event)"
        @cancel="$emit('mcpCancel')"
        @theme-change="$emit('themeChange', $event)"
      />
    </div>

    <!-- 弹窗加载骨架屏 或 初始化骨架屏 -->
    <div v-else-if="props.showMcpPopup || props.isInitializing" class="flex flex-col w-full h-screen bg-black text-white">
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
      v-else
      :current-theme="props.appConfig.theme"
      :always-on-top="props.appConfig.window.alwaysOnTop"
      :audio-notification-enabled="props.appConfig.audio.enabled"
      :audio-url="props.appConfig.audio.url"
      :window-width="props.appConfig.window.width"
      :window-height="props.appConfig.window.height"
      :fixed-window-size="props.appConfig.window.fixed"
      @theme-change="$emit('themeChange', $event)"
      @toggle-always-on-top="$emit('toggleAlwaysOnTop')"
      @toggle-audio-notification="$emit('toggleAudioNotification')"
      @update-audio-url="$emit('updateAudioUrl', $event)"
      @test-audio="$emit('testAudio')"
      @stop-audio="$emit('stopAudio')"
      @test-audio-error="$emit('testAudioError', $event)"
      @update-window-size="$emit('updateWindowSize', $event)"
    />
  </div>
</template>
