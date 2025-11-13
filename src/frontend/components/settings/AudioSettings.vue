<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { computed, onMounted, ref, watch } from 'vue'

// 類型定義
interface AudioAsset {
  id: string
  name: string
  filename: string
}

const props = defineProps({
  audioNotificationEnabled: {
    type: Boolean,
    required: true,
  },
  audioUrl: {
    type: String,
    default: '',
  },
})

const emit = defineEmits(['toggleAudioNotification', 'updateAudioUrl', 'testAudio', 'stopAudio', 'testAudioError'])

// 預設音效設定（从后端獲取）
const presetSounds = ref<AudioAsset[]>([])
const loading = ref(true)

// 當前选中的音效類型
const selectedSoundType = ref('preset') // 'preset' | 'custom'
const selectedPreset = ref('default') // 使用资源ID
const customUrl = ref('')
const customName = ref('')

// 自訂音訊狀態
const customAudioState = ref('hidden') // 'hidden' | 'show_input' | 'saved' | 'verified'
const isTestingCustom = ref(false)

// 載入可用音訊资源
async function loadAudioAssets() {
  try {
    loading.value = true
    const assets = await invoke('get_available_audio_assets') as AudioAsset[]
    presetSounds.value = assets.map(asset => ({
      id: asset.id,
      name: asset.name,
      filename: asset.filename,
    }))

    // 如果没有选中的預設，預設選擇第一个
    if (!selectedPreset.value && presetSounds.value.length > 0) {
      selectedPreset.value = presetSounds.value[0].id
    }
  }
  catch (error) {
    console.error('載入音訊资源失敗:', error)
  }
  finally {
    loading.value = false
  }
}

// 计算當前音效URL
const currentAudioUrl = computed(() => {
  if (selectedSoundType.value === 'preset') {
    return selectedPreset.value // 傳回资源ID
  }
  else {
    return customUrl.value
  }
})

// 更新音效設定
function updateAudioConfig() {
  emit('updateAudioUrl', currentAudioUrl.value)
}

// 切换到預設音效
function selectPreset(presetId) {
  selectedSoundType.value = 'preset'
  selectedPreset.value = presetId
  updateAudioConfig()
  // 停止之前的播放并播放新音效
  stopPreviousAudio()
  emit('testAudio')
}

// 停止之前的音訊播放
function stopPreviousAudio() {
  // 傳送停止音訊事件到父元件
  emit('stopAudio')
}

// 切换到自訂音效
function selectCustom() {
  // 如果已经驗證，直接切换到自訂模式
  if (customAudioState.value === 'verified') {
    selectedSoundType.value = 'custom'
    updateAudioConfig()
  }
  else {
    // 否则顯示自訂輸入界面，設定为顯示狀態
    customAudioState.value = 'show_input'
    if (!customUrl.value) {
      customUrl.value = ''
      customName.value = ''
    }
  }
}

// 判断是否为預設音效ID（从服务端獲取的列表）
function isPresetId(id: string) {
  return presetSounds.value.some(s => s.id === id)
}

// 儲存自訂URL
async function saveCustomUrl() {
  if (!customUrl.value.trim())
    return

  customAudioState.value = 'saved'
  // 这里不立即切换到自訂模式，需要試听成功后才切换
}

// 試听自訂音訊
async function testCustomAudio() {
  if (!customUrl.value.trim())
    return

  isTestingCustom.value = true

  // 儲存原来的URL，以便失敗時恢復
  const originalUrl = props.audioUrl

  try {
    // 临時設定自訂URL進行試听
    emit('updateAudioUrl', customUrl.value)

    // 直接呼叫后端測試音訊，而不是透過事件
    await invoke('test_audio_sound')

    // 試听成功，标记为已驗證并切换到自訂模式
    customAudioState.value = 'verified'
    selectedSoundType.value = 'custom'

    console.log('✅ 自訂音訊試听成功，已切换到自訂模式')
  }
  catch (error) {
    // 試听失敗，恢復原来的URL
    emit('updateAudioUrl', originalUrl)
    customAudioState.value = 'show_input'
    console.error('❌ 自訂音訊試听失敗:', error)

    // 顯示錯誤提示给用户
    // 这里可以透過事件通知父元件顯示錯誤消息
    emit('testAudioError', error)
  }
  finally {
    isTestingCustom.value = false
  }
}

// 更新自訂URL輸入
function updateCustomUrl() {
  // 輸入時重置狀態
  if (customAudioState.value !== 'verified') {
    customAudioState.value = 'show_input'
  }
}

// 初始化當前狀態
function initializeState() {
  if (isPresetId(props.audioUrl)) {
    selectedSoundType.value = 'preset'
    customAudioState.value = 'hidden'
    selectedPreset.value = props.audioUrl
    // 清空自訂URL，避免顯示預設ID
    customUrl.value = ''
    customName.value = ''
  }
  else if (props.audioUrl) {
    selectedSoundType.value = 'custom'
    customUrl.value = props.audioUrl
    customAudioState.value = 'verified' // 已有自訂URL说明已驗證
  }
  else {
    // 預設狀態
    selectedSoundType.value = 'preset'
    customAudioState.value = 'hidden'
    customUrl.value = ''
    customName.value = ''
    // 如果没有設定，使用第一个可用的預設
    if (presetSounds.value.length > 0) {
      selectedPreset.value = presetSounds.value[0].id
    }
  }
}

// 監聽props變化
watch(() => props.audioUrl, () => {
  initializeState()
}, { immediate: true })

onMounted(async () => {
  await loadAudioAssets()
  initializeState()
})
</script>

<template>
  <!-- 設定內容 -->
  <n-space vertical size="large">
    <!-- 音訊通知开关 -->
    <div class="flex items-center justify-between">
      <div class="flex items-center">
        <div class="w-1.5 h-1.5 bg-warning rounded-full mr-3 flex-shrink-0" />
        <div>
          <div class="text-sm font-medium leading-relaxed">
            音訊通知
          </div>
          <div class="text-xs opacity-60">
            啟用后在MCP工具被觸發時播放音訊提示
          </div>
        </div>
      </div>
      <n-switch
        :value="audioNotificationEnabled"
        size="small"
        @update:value="$emit('toggleAudioNotification')"
      />
    </div>

    <!-- 音效選擇 -->
    <div v-if="audioNotificationEnabled" class="pt-4 border-t border-gray-200 dark:border-gray-700">
      <div class="flex items-start">
        <div class="w-1.5 h-1.5 bg-warning rounded-full mr-3 mt-2 flex-shrink-0" />
        <div class="flex-1">
          <div class="text-sm font-medium mb-3 leading-relaxed">
            音效選擇
          </div>

          <!-- 預設音效 -->
          <div class="mb-4">
            <div class="text-xs opacity-60 mb-2">
              預設音效
            </div>
            <div v-if="loading" class="text-xs opacity-60">
              載入中...
            </div>
            <n-space v-else>
              <n-button
                v-for="preset in presetSounds"
                :key="preset.id"
                :type="selectedSoundType === 'preset' && selectedPreset === preset.id ? 'primary' : 'default'"
                size="small"
                @click="selectPreset(preset.id)"
              >
                {{ preset.name }}
              </n-button>
            </n-space>
          </div>

          <!-- 自訂音效 -->
          <div class="mb-3">
            <div class="flex items-center justify-between mb-2">
              <div class="text-xs opacity-60">
                自訂音效
              </div>
              <n-button
                :type="selectedSoundType === 'custom' ? 'primary' : 'default'"
                size="tiny"
                @click="selectCustom"
              >
                使用自訂
              </n-button>
            </div>

            <div v-if="selectedSoundType === 'custom' || customAudioState !== 'hidden'" class="space-y-2">
              <!-- 自訂名称 -->
              <n-input
                v-model:value="customName"
                size="small"
                placeholder="音效名称（可选）"
              />

              <!-- 自訂URL -->
              <n-input-group>
                <n-input
                  v-model:value="customUrl"
                  size="small"
                  placeholder="音效檔案路径或URL"
                  @input="updateCustomUrl"
                />
                <n-button
                  v-if="customAudioState === 'show_input'"
                  type="primary"
                  size="small"
                  :disabled="!customUrl.trim()"
                  @click="saveCustomUrl"
                >
                  <template #icon>
                    <div class="i-carbon-save text-sm" />
                  </template>
                  儲存
                </n-button>
                <n-button
                  v-else-if="customAudioState === 'saved'"
                  type="success"
                  size="small"
                  :disabled="!customUrl.trim() || isTestingCustom"
                  :loading="isTestingCustom"
                  @click="testCustomAudio"
                >
                  <template #icon>
                    <div class="i-carbon-volume-up text-sm" />
                  </template>
                  試听
                </n-button>
                <n-button
                  v-else
                  type="info"
                  size="small"
                  :disabled="!customUrl.trim()"
                  @click="$emit('testAudio')"
                >
                  <template #icon>
                    <div class="i-carbon-volume-up text-sm" />
                  </template>
                  試听
                </n-button>
              </n-input-group>

              <div class="text-xs opacity-60">
                <span v-if="customAudioState === 'show_input'">
                  示例：/path/to/sound.mp3 或 https://example.com/notification.wav
                </span>
                <span v-else-if="customAudioState === 'saved'" class="text-orange-500">
                  請点击試听按钮測試音訊，試听成功后将自動切换到自訂音效
                </span>
                <span v-else class="text-green-500">
                  ✅ 自訂音效已驗證并啟用
                </span>
              </div>
            </div>
          </div>

          <!-- 當前音效顯示 -->
          <div class="mt-3 p-2 bg-gray-100 rounded text-xs">
            <span class="opacity-60">當前音效：</span>
            <span v-if="selectedSoundType === 'preset'">
              {{ presetSounds.find(p => p.id === selectedPreset)?.name }}
            </span>
            <span v-else-if="customUrl">
              {{ customName || '自訂音效' }} ({{ customUrl }})
            </span>
            <span v-else class="opacity-60">未設定</span>
          </div>
        </div>
      </div>
    </div>
  </n-space>
</template>
