<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { useMessage } from 'naive-ui'
import { onMounted, onUnmounted, ref } from 'vue'
import { useI18n } from '../../i18n'
import AudioSettings from '../settings/AudioSettings.vue'
import CustomPromptSettings from '../settings/CustomPromptSettings.vue'
import FontSettings from '../settings/FontSettings.vue'
import LanguageSettings from '../settings/LanguageSettings.vue'
import ReplySettings from '../settings/ReplySettings.vue'
import ShortcutSettings from '../settings/ShortcutSettings.vue'
import ThemeSettings from '../settings/ThemeSettings.vue'
import VersionChecker from '../settings/VersionChecker.vue'
import WindowSettings from '../settings/WindowSettings.vue'

const { t } = useI18n()

interface Props {
  currentTheme: string
  alwaysOnTop: boolean
  audioNotificationEnabled: boolean
  audioUrl: string
  windowWidth: number
  windowHeight: number
  fixedWindowSize: boolean
}

defineProps<Props>()
const emit = defineEmits<Emits>()
const message = useMessage()
const isReloading = ref(false)
const configFilePath = ref('config.json')
let unlistenConfigReloaded: (() => void) | null = null

// 重新載入設定（透過重新載入設定實作）
async function reloadConfig() {
  if (isReloading.value)
    return

  isReloading.value = true
  try {
    // 觸發重新載入設定的事件
    emit('configReloaded')
    message.success(t('settings.reload.success'))
  }
  catch (error) {
    console.error('重新載入設定失敗:', error)
    message.error(t('settings.reload.error'))
  }
  finally {
    isReloading.value = false
  }
}

// 獲取設定檔案路径
async function loadConfigFilePath() {
  try {
    const path = await invoke('get_config_file_path')
    configFilePath.value = path as string
    console.log('設定檔案路径:', configFilePath.value)
  }
  catch (error) {
    console.error('獲取設定檔案路径失敗:', error)
    configFilePath.value = 'config.json' // 使用預設值
  }
}

// 監聽設定重新載入事件
onMounted(async () => {
  try {
    // 獲取設定檔案路径
    await loadConfigFilePath()

    unlistenConfigReloaded = await listen('config_reloaded', () => {
      // 設定重新載入后，重新載入設定而不是重新整理整个页面
      console.log('收到設定重新載入事件，重新載入設定')
      // 觸發重新載入設定的事件
      emit('configReloaded')
    })
  }
  catch (error) {
    console.error('設定設定重新載入監聽器失敗:', error)
  }
})

onUnmounted(() => {
  if (unlistenConfigReloaded) {
    unlistenConfigReloaded()
  }
})

interface Emits {
  themeChange: [theme: string]
  toggleAlwaysOnTop: []
  toggleAudioNotification: []
  updateAudioUrl: [url: string]
  testAudio: []
  stopAudio: []
  testAudioError: [error: any]
  updateWindowSize: [size: { width: number, height: number, fixed: boolean }]
  configReloaded: []
}

// 處理視窗尺寸更新
function handleWindowSizeUpdate(size: { width: number, height: number, fixed: boolean }) {
  emit('updateWindowSize', size)
}
</script>

<template>
  <div class="max-w-3xl mx-auto tab-content">
    <n-collapse size="large" :default-expanded-names="[]" arrow-placement="right">
      <!-- 主題設定 -->
      <n-collapse-item name="theme">
        <template #header>
          <div class="flex items-center justify-between w-full">
            <div class="flex items-center">
              <div class="w-10 h-10 rounded-lg bg-primary-100 dark:bg-primary-900 flex items-center justify-center mr-4">
                <div class="i-carbon-color-palette text-lg text-primary-600 dark:text-primary-400" />
              </div>
              <div>
                <div class="text-lg font-medium tracking-tight mb-1">
                  {{ t('settings.theme.title') }}
                </div>
                <div class="text-sm opacity-60 font-normal">
                  {{ t('settings.theme.description') }}
                </div>
              </div>
            </div>
          </div>
        </template>
        <div class="setting-content">
          <ThemeSettings :current-theme="currentTheme" @theme-change="$emit('themeChange', $event)" />
        </div>
      </n-collapse-item>

      <!-- 語言設定 -->
      <n-collapse-item name="language">
        <template #header>
          <div class="flex items-center justify-between w-full">
            <div class="flex items-center">
              <div class="w-10 h-10 rounded-lg bg-purple-100 dark:bg-purple-900 flex items-center justify-center mr-4">
                <div class="i-carbon-language text-lg text-purple-600 dark:text-purple-400" />
              </div>
              <div>
                <div class="text-lg font-medium tracking-tight mb-1">
                  {{ t('settings.language.title') }}
                </div>
                <div class="text-sm opacity-60 font-normal">
                  {{ t('settings.language.description') }}
                </div>
              </div>
            </div>
          </div>
        </template>
        <div class="setting-content">
          <LanguageSettings />
        </div>
      </n-collapse-item>

      <!-- 字型設定 -->
      <n-collapse-item name="font">
        <template #header>
          <div class="flex items-center justify-between w-full">
            <div class="flex items-center">
              <div class="w-10 h-10 rounded-lg bg-orange-100 dark:bg-orange-900 flex items-center justify-center mr-4">
                <div class="i-carbon-text-font text-lg text-orange-600 dark:text-orange-400" />
              </div>
              <div>
                <div class="text-lg font-medium tracking-tight mb-1">
                  {{ t('settings.font.title') }}
                </div>
                <div class="text-sm opacity-60 font-normal">
                  {{ t('settings.font.description') }}
                </div>
              </div>
            </div>
          </div>
        </template>
        <div class="setting-content">
          <FontSettings />
        </div>
      </n-collapse-item>

      <!-- 繼續回复設定 -->
      <n-collapse-item name="reply">
        <template #header>
          <div class="flex items-center justify-between w-full">
            <div class="flex items-center">
              <div class="w-10 h-10 rounded-lg bg-blue-100 dark:bg-blue-900 flex items-center justify-center mr-4">
                <div class="i-carbon-continue text-lg text-blue-600 dark:text-blue-400" />
              </div>
              <div>
                <div class="text-lg font-medium tracking-tight mb-1">
                  {{ t('settings.reply.title') }}
                </div>
                <div class="text-sm opacity-60 font-normal">
                  {{ t('settings.reply.description') }}
                </div>
              </div>
            </div>
          </div>
        </template>
        <div class="setting-content">
          <ReplySettings />
        </div>
      </n-collapse-item>

      <!-- 視窗設定 -->
      <n-collapse-item name="window">
        <template #header>
          <div class="flex items-center justify-between w-full">
            <div class="flex items-center">
              <div class="w-10 h-10 rounded-lg bg-green-100 dark:bg-green-900 flex items-center justify-center mr-4">
                <div class="i-carbon-application text-lg text-green-600 dark:text-green-400" />
              </div>
              <div>
                <div class="text-lg font-medium tracking-tight mb-1">
                  {{ t('settings.window.title') }}
                </div>
                <div class="text-sm opacity-60 font-normal">
                  {{ t('settings.window.description') }}
                </div>
              </div>
            </div>
          </div>
        </template>
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
      </n-collapse-item>

      <!-- 音訊設定 -->
      <n-collapse-item name="audio">
        <template #header>
          <div class="flex items-center justify-between w-full">
            <div class="flex items-center">
              <div class="w-10 h-10 rounded-lg bg-yellow-100 dark:bg-yellow-900 flex items-center justify-center mr-4">
                <div class="i-carbon-volume-up text-lg text-yellow-600 dark:text-yellow-400" />
              </div>
              <div>
                <div class="text-lg font-medium tracking-tight mb-1">
                  {{ t('settings.audio.title') }}
                </div>
                <div class="text-sm opacity-60 font-normal">
                  {{ t('settings.audio.description') }}
                </div>
              </div>
            </div>
          </div>
        </template>
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
      </n-collapse-item>

      <!-- Telegram 功能已移除 -->

      <!-- 快捷模板設定 -->
      <n-collapse-item name="custom-prompt">
        <template #header>
          <div class="flex items-center justify-between w-full">
            <div class="flex items-center">
              <div class="w-10 h-10 rounded-lg bg-orange-100 dark:bg-orange-900 flex items-center justify-center mr-4">
                <div class="i-carbon-text-creation text-lg text-orange-600 dark:text-orange-400" />
              </div>
              <div>
                <div class="text-lg font-medium tracking-tight mb-1">
                  {{ t('settings.customPrompt.title') }}
                </div>
                <div class="text-sm opacity-60 font-normal">
                  {{ t('settings.customPrompt.description') }}
                </div>
              </div>
            </div>
          </div>
        </template>
        <div class="setting-content">
          <CustomPromptSettings />
        </div>
      </n-collapse-item>

      <!-- 快捷键設定 -->
      <n-collapse-item name="shortcuts">
        <template #header>
          <div class="flex items-center justify-between w-full">
            <div class="flex items-center">
              <div class="w-10 h-10 rounded-lg bg-indigo-100 dark:bg-indigo-900 flex items-center justify-center mr-4">
                <div class="i-carbon-keyboard text-lg text-indigo-600 dark:text-indigo-400" />
              </div>
              <div>
                <div class="text-lg font-medium tracking-tight mb-1">
                  {{ t('settings.shortcut.title') }}
                </div>
                <div class="text-sm opacity-60 font-normal">
                  {{ t('settings.shortcut.description') }}
                </div>
              </div>
            </div>
          </div>
        </template>
        <div class="setting-content">
          <ShortcutSettings />
        </div>
      </n-collapse-item>

      <!-- 設定管理 -->
      <n-collapse-item name="config">
        <template #header>
          <div class="flex items-center justify-between w-full">
            <div class="flex items-center">
              <div class="w-10 h-10 rounded-lg bg-blue-100 dark:bg-blue-900 flex items-center justify-center mr-4">
                <div class="i-carbon-settings-adjust text-lg text-blue-600 dark:text-blue-400" />
              </div>
              <div>
                <div class="text-lg font-medium tracking-tight mb-1">
                  設定管理
                </div>
                <div class="text-sm opacity-60 font-normal">
                  重新載入設定檔案和管理設定
                </div>
              </div>
            </div>
          </div>
        </template>
        <div class="setting-content">
          <n-space vertical size="large">
            <!-- 重新載入設定 -->
            <div class="flex items-center justify-between">
              <div class="flex items-center">
                <div class="w-1.5 h-1.5 bg-info rounded-full mr-3 flex-shrink-0" />
                <div>
                  <div class="text-sm font-medium leading-relaxed">
                    重新載入設定檔案
                  </div>
                  <div class="text-xs opacity-60">
                    从 config.json 重新載入所有設定設定
                  </div>
                </div>
              </div>
              <n-button
                size="small"
                type="primary"
                :loading="isReloading"
                @click="reloadConfig"
              >
                <template #icon>
                  <div class="i-carbon-restart w-4 h-4" />
                </template>
                重新載入
              </n-button>
            </div>

            <!-- 設定檔案位置说明 -->
            <div class="flex items-start">
              <div class="w-1.5 h-1.5 bg-warning rounded-full mr-3 flex-shrink-0 mt-2" />
              <div>
                <div class="text-sm font-medium leading-relaxed mb-1">
                  設定檔案位置
                </div>
                <div class="text-xs opacity-60 leading-relaxed">
                  設定檔案路径：<br>
                  <code class="bg-black-100 px-1 rounded text-xs break-all">{{ configFilePath }}</code><br>
                  您可以直接編輯该檔案，然后点击"重新載入"按钮使更改生效
                </div>
              </div>
            </div>
          </n-space>
        </div>
      </n-collapse-item>

      <!-- 版本檢查 -->
      <n-collapse-item name="version">
        <template #header>
          <div class="flex items-center justify-between w-full">
            <div class="flex items-center">
              <div class="w-10 h-10 rounded-lg bg-purple-100 dark:bg-purple-900 flex items-center justify-center mr-4">
                <div class="i-carbon-update-now text-lg text-purple-600 dark:text-purple-400" />
              </div>
              <div>
                <div class="text-lg font-medium tracking-tight mb-1">
                  版本檢查
                </div>
                <div class="text-sm opacity-60 font-normal">
                  檢查應用更新和版本訊息
                </div>
              </div>
            </div>
          </div>
        </template>
        <div class="setting-content">
          <VersionChecker />
        </div>
      </n-collapse-item>
    </n-collapse>
  </div>
</template>

<style>
/* 移除子元件的卡片样式，因为现在由折叠面板提供容器 */
.setting-content :deep(.n-card) {
  background: transparent;
  border: none;
  box-shadow: none;
}

.setting-content :deep(.n-card .n-card-header) {
  display: none;
}
</style>
