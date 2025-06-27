<script setup lang="ts">
import { ref } from 'vue'
import AudioSettings from '../settings/AudioSettings.vue'
import FontSettings from '../settings/FontSettings.vue'
import ReplySettings from '../settings/ReplySettings.vue'
import TelegramSettings from '../settings/TelegramSettings.vue'
import ThemeSettings from '../settings/ThemeSettings.vue'
import VersionChecker from '../settings/VersionChecker.vue'
import WindowSettings from '../settings/WindowSettings.vue'

interface Props {
  currentTheme: string
  alwaysOnTop: boolean
  audioNotificationEnabled: boolean
  audioUrl: string
  windowWidth: number
  windowHeight: number
  fixedWindowSize: boolean
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
}

defineProps<Props>()
const emit = defineEmits<Emits>()

// 折叠状态管理
const expandedSections = ref<Record<string, boolean>>({
  theme: false, // 主题设置默认收起
  font: false, // 字体设置默认收起
  reply: false, // 继续回复设置默认收起
  window: false, // 窗口设置默认收起
  audio: false, // 音频设置默认收起
  telegram: false, // Telegram设置默认收起
  version: false, // 版本检查默认收起
})

// 切换展开/收起状态
function toggleSection(section: string) {
  expandedSections.value[section] = !expandedSections.value[section]
}

// 处理窗口尺寸更新
function handleWindowSizeUpdate(size: { width: number, height: number, fixed: boolean }) {
  emit('updateWindowSize', size)
}
</script>

<template>
  <div class="max-w-3xl mx-auto tab-content">
    <n-space vertical size="medium">
      <!-- 主题设置 -->
      <div class="setting-section">
        <div
          class="setting-header"
          :class="{ expanded: expandedSections.theme }"
          @click="toggleSection('theme')"
        >
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-primary-100 dark:bg-primary-900 flex items-center justify-center mr-3">
              <div class="i-carbon-color-palette text-lg text-primary-600 dark:text-primary-400" />
            </div>
            <div>
              <div class="text-lg font-medium tracking-tight">
                主题设置
              </div>
              <div class="text-sm opacity-60 font-normal">
                选择您喜欢的界面主题
              </div>
            </div>
          </div>
          <div class="expand-icon" :class="{ rotated: expandedSections.theme }">
            <div class="i-carbon-chevron-down text-lg opacity-60" />
          </div>
        </div>
        <n-collapse-transition :show="expandedSections.theme">
          <div class="setting-content">
            <ThemeSettings :current-theme="currentTheme" @theme-change="$emit('themeChange', $event)" />
          </div>
        </n-collapse-transition>
      </div>

      <!-- 字体设置 -->
      <div class="setting-section">
        <div
          class="setting-header"
          :class="{ expanded: expandedSections.font }"
          @click="toggleSection('font')"
        >
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-orange-100 dark:bg-orange-900 flex items-center justify-center mr-3">
              <div class="i-carbon-text-font text-lg text-orange-600 dark:text-orange-400" />
            </div>
            <div>
              <div class="text-lg font-medium tracking-tight">
                字体设置
              </div>
              <div class="text-sm opacity-60 font-normal">
                自定义应用字体系列和大小
              </div>
            </div>
          </div>
          <div class="expand-icon" :class="{ rotated: expandedSections.font }">
            <div class="i-carbon-chevron-down text-lg opacity-60" />
          </div>
        </div>
        <n-collapse-transition :show="expandedSections.font">
          <div class="setting-content">
            <FontSettings />
          </div>
        </n-collapse-transition>
      </div>

      <!-- 继续回复设置 -->
      <div class="setting-section">
        <div
          class="setting-header"
          :class="{ expanded: expandedSections.reply }"
          @click="toggleSection('reply')"
        >
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-blue-100 dark:bg-blue-900 flex items-center justify-center mr-3">
              <div class="i-carbon-continue text-lg text-blue-600 dark:text-blue-400" />
            </div>
            <div>
              <div class="text-lg font-medium tracking-tight">
                继续回复设置
              </div>
              <div class="text-sm opacity-60 font-normal">
                配置AI继续回复的行为
              </div>
            </div>
          </div>
          <div class="expand-icon" :class="{ rotated: expandedSections.reply }">
            <div class="i-carbon-chevron-down text-lg opacity-60" />
          </div>
        </div>
        <n-collapse-transition :show="expandedSections.reply">
          <div class="setting-content">
            <ReplySettings />
          </div>
        </n-collapse-transition>
      </div>

      <!-- 窗口设置 -->
      <div class="setting-section">
        <div
          class="setting-header"
          :class="{ expanded: expandedSections.window }"
          @click="toggleSection('window')"
        >
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-green-100 dark:bg-green-900 flex items-center justify-center mr-3">
              <div class="i-carbon-application text-lg text-green-600 dark:text-green-400" />
            </div>
            <div>
              <div class="text-lg font-medium tracking-tight">
                窗口设置
              </div>
              <div class="text-sm opacity-60 font-normal">
                调整窗口显示和行为
              </div>
            </div>
          </div>
          <div class="expand-icon" :class="{ rotated: expandedSections.window }">
            <div class="i-carbon-chevron-down text-lg opacity-60" />
          </div>
        </div>
        <n-collapse-transition :show="expandedSections.window">
          <div class="setting-content">
            <WindowSettings
              :always-on-top="alwaysOnTop"
              :window-width="windowWidth"
              :window-height="windowHeight"
              :fixed-window-size="fixedWindowSize"
              @toggle-always-on-top="$emit('toggleAlwaysOnTop')"
              @update-window-size="handleWindowSizeUpdate"
            />
          </div>
        </n-collapse-transition>
      </div>

      <!-- 音频设置 -->
      <div class="setting-section">
        <div
          class="setting-header"
          :class="{ expanded: expandedSections.audio }"
          @click="toggleSection('audio')"
        >
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-yellow-100 dark:bg-yellow-900 flex items-center justify-center mr-3">
              <div class="i-carbon-volume-up text-lg text-yellow-600 dark:text-yellow-400" />
            </div>
            <div>
              <div class="text-lg font-medium tracking-tight">
                音频设置
              </div>
              <div class="text-sm opacity-60 font-normal">
                配置音频通知和提示音
              </div>
            </div>
          </div>
          <div class="expand-icon" :class="{ rotated: expandedSections.audio }">
            <div class="i-carbon-chevron-down text-lg opacity-60" />
          </div>
        </div>
        <n-collapse-transition :show="expandedSections.audio">
          <div class="setting-content">
            <AudioSettings
              :audio-notification-enabled="audioNotificationEnabled"
              :audio-url="audioUrl"
              @toggle-audio-notification="$emit('toggleAudioNotification')"
              @update-audio-url="$emit('updateAudioUrl', $event)"
              @test-audio="$emit('testAudio')"
              @stop-audio="$emit('stopAudio')"
              @test-audio-error="$emit('testAudioError', $event)"
            />
          </div>
        </n-collapse-transition>
      </div>

      <!-- Telegram设置 -->
      <div class="setting-section">
        <div
          class="setting-header"
          :class="{ expanded: expandedSections.telegram }"
          @click="toggleSection('telegram')"
        >
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-cyan-100 dark:bg-cyan-900 flex items-center justify-center mr-3">
              <div class="i-carbon-send text-lg text-cyan-600 dark:text-cyan-400" />
            </div>
            <div>
              <div class="text-lg font-medium tracking-tight">
                Telegram设置
              </div>
              <div class="text-sm opacity-60 font-normal">
                配置Telegram机器人集成
              </div>
            </div>
          </div>
          <div class="expand-icon" :class="{ rotated: expandedSections.telegram }">
            <div class="i-carbon-chevron-down text-lg opacity-60" />
          </div>
        </div>
        <n-collapse-transition :show="expandedSections.telegram">
          <div class="setting-content">
            <TelegramSettings />
          </div>
        </n-collapse-transition>
      </div>

      <!-- 版本检查 -->
      <div class="setting-section">
        <div
          class="setting-header"
          :class="{ expanded: expandedSections.version }"
          @click="toggleSection('version')"
        >
          <div class="flex items-center">
            <div class="w-10 h-10 rounded-lg bg-purple-100 dark:bg-purple-900 flex items-center justify-center mr-3">
              <div class="i-carbon-update-now text-lg text-purple-600 dark:text-purple-400" />
            </div>
            <div>
              <div class="text-lg font-medium tracking-tight">
                版本检查
              </div>
              <div class="text-sm opacity-60 font-normal">
                检查应用更新和版本信息
              </div>
            </div>
          </div>
          <div class="expand-icon" :class="{ rotated: expandedSections.version }">
            <div class="i-carbon-chevron-down text-lg opacity-60" />
          </div>
        </div>
        <n-collapse-transition :show="expandedSections.version">
          <div class="setting-content">
            <VersionChecker />
          </div>
        </n-collapse-transition>
      </div>
    </n-space>
  </div>
</template>

<style scoped>
.setting-section {
  @apply bg-black-100 rounded-lg border border-black-200 overflow-hidden transition-all duration-200;
}

.setting-header {
  @apply flex items-center justify-between p-4 cursor-pointer transition-all duration-200;
  @apply hover:bg-black-200 active:bg-black-300;
}

.setting-header.expanded {
  @apply bg-black-200 border-b border-black-300;
}

.expand-icon {
  @apply transition-transform duration-300 ease-in-out;
  transform: rotate(0deg);
}

.expand-icon.rotated {
  transform: rotate(180deg);
}

.setting-content {
  @apply p-0;
}

/* 移除子组件的卡片样式，因为现在由父容器提供 */
.setting-content :deep(.n-card) {
  @apply bg-transparent border-0 shadow-none;
}

.setting-content :deep(.n-card .n-card-header) {
  @apply hidden;
}

.setting-content :deep(.n-card .n-card__content) {
  @apply p-4 pt-0;
}
</style>
