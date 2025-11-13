<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { onMounted, ref } from 'vue'

interface ReplyConfig {
  enable_continue_reply: boolean
  auto_continue_threshold: number
  continue_prompt: string
}

const localConfig = ref<ReplyConfig>({
  enable_continue_reply: true,
  auto_continue_threshold: 1000,
  continue_prompt: '請按照最佳實務繼續',
})

// 載入設定
async function loadConfig() {
  try {
    const config = await invoke('get_reply_config')
    localConfig.value = config as ReplyConfig
  }
  catch (error) {
    console.error('載入繼續回覆設定失敗:', error)
  }
}

// 更新設定
async function updateConfig() {
  try {
    await invoke('set_reply_config', { replyConfig: localConfig.value })
  }
  catch (error) {
    console.error('儲存繼續回覆設定失敗:', error)
  }
}

onMounted(() => {
  loadConfig()
})
</script>

<template>
  <!-- 設定內容 -->
  <n-space vertical size="large">
    <!-- 啟用繼續回复 -->
    <div class="flex items-center justify-between">
      <div class="flex items-center">
        <div class="w-1.5 h-1.5 bg-info rounded-full mr-3 flex-shrink-0" />
        <div>
          <div class="text-sm font-medium leading-relaxed">
            啟用繼續回复
          </div>
          <div class="text-xs opacity-60">
            啟用后将顯示繼續按钮
          </div>
        </div>
      </div>
      <n-switch
        v-model:value="localConfig.enable_continue_reply"
        size="small"
        @update:value="updateConfig"
      />
    </div>

    <!-- 繼續提示詞 -->
    <div v-if="localConfig.enable_continue_reply">
      <div class="flex items-center mb-3">
        <div class="w-1.5 h-1.5 bg-info rounded-full mr-3 flex-shrink-0" />
        <div>
          <div class="text-sm font-medium leading-relaxed">
            繼續提示詞
          </div>
          <div class="text-xs opacity-60">
            点击繼續按钮時傳送的提示詞
          </div>
        </div>
      </div>
      <n-input
        v-model:value="localConfig.continue_prompt"
        size="small"
        placeholder="請按照最佳實務繼續"
        @input="updateConfig"
      />
    </div>
  </n-space>
</template>
