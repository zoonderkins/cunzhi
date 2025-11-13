<script setup lang="ts">
import { useMessage } from 'naive-ui'
import { computed, onMounted, ref } from 'vue'
import { useFontManager } from '../../composables/useFontManager'

const message = useMessage()
const {
  fontConfig,
  fontFamilyOptions,
  fontSizeOptions,
  currentFontFamily,
  currentFontScale,
  loadFontConfig,
  loadFontOptions,
  setFontFamily,
  setFontSize,
  setCustomFontFamily,
  resetFontConfig,
} = useFontManager()

// 本地狀態
const customFontInput = ref('')
const fontNameInput = ref('')
const isLoading = ref(false)

// 计算當前字型系列的顯示名称
const currentFontFamilyName = computed(() => {
  const option = fontFamilyOptions.value.find(opt => opt.id === fontConfig.value.font_family)
  return option?.name || '未知'
})

// 计算當前字型大小的顯示名称
const currentFontSizeName = computed(() => {
  const option = fontSizeOptions.value.find(opt => opt.id === fontConfig.value.font_size)
  return option?.name || '未知'
})

// 處理字型系列變更
async function handleFontFamilyChange(value: string) {
  if (isLoading.value)
    return

  try {
    isLoading.value = true
    await setFontFamily(value)

    // 如果切换到非自訂字型，清空自訂字型輸入
    if (value !== 'custom') {
      customFontInput.value = ''
    }
    else {
      // 如果切换到自訂字型，使用當前的自訂字型值
      customFontInput.value = fontConfig.value.custom_font_family
    }

    message.success('字型系列已更新')
  }
  catch (error) {
    message.error('更新字型系列失敗')
    console.error(error)
  }
  finally {
    isLoading.value = false
  }
}

// 處理字型大小變更
async function handleFontSizeChange(value: string) {
  if (isLoading.value)
    return

  try {
    isLoading.value = true
    await setFontSize(value)
    message.success('字型大小已更新')
  }
  catch (error) {
    message.error('更新字型大小失敗')
    console.error(error)
  }
  finally {
    isLoading.value = false
  }
}

// 處理自訂字型系列變更
async function handleCustomFontFamilyChange() {
  if (isLoading.value || !customFontInput.value.trim())
    return

  try {
    isLoading.value = true
    await setCustomFontFamily(customFontInput.value.trim())
    message.success('自訂字型系列已更新')
  }
  catch (error) {
    message.error('更新自訂字型系列失敗')
    console.error(error)
  }
  finally {
    isLoading.value = false
  }
}

// 處理字型名称輸入應用
async function handleFontNameApply() {
  if (isLoading.value || !fontNameInput.value.trim())
    return

  const fontName = fontNameInput.value.trim()
  // 建構字型CSS值，包含降级字型
  const fontCssValue = `"${fontName}", -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif`

  try {
    isLoading.value = true
    // 先設定为自訂字型模式
    await setFontFamily('custom')
    // 然后設定自訂字型值
    await setCustomFontFamily(fontCssValue)
    customFontInput.value = fontCssValue
    message.success(`字型 "${fontName}" 已應用`)
  }
  catch (error) {
    message.error('應用字型失敗')
    console.error(error)
  }
  finally {
    isLoading.value = false
  }
}

// 處理重置設定
async function handleResetConfig() {
  if (isLoading.value)
    return

  try {
    isLoading.value = true
    await resetFontConfig()
    customFontInput.value = fontConfig.value.custom_font_family
    message.success('字型設定已重置')
  }
  catch (error) {
    message.error('重置字型設定失敗')
    console.error(error)
  }
  finally {
    isLoading.value = false
  }
}

// 元件挂载時載入資料
onMounted(async () => {
  try {
    await Promise.all([
      loadFontConfig(),
      loadFontOptions(),
    ])
    customFontInput.value = fontConfig.value.custom_font_family
  }
  catch (error) {
    console.error('載入字型設定失敗:', error)
  }
})
</script>

<template>
  <!-- 設定內容 -->
  <div class="space-y-6">
    <n-space vertical size="large">
      <!-- 字型系列設定 -->
      <div>
        <div class="flex items-center justify-between mb-3">
          <div class="flex items-center">
            <div class="w-1.5 h-1.5 bg-orange-500 rounded-full mr-3 flex-shrink-0" />
            <div class="flex-1">
              <div class="text-sm font-medium leading-relaxed">
                字型系列
              </div>
              <div class="text-xs opacity-60">
                選擇或自訂應用使用的字型系列
              </div>
            </div>
          </div>
          <!-- 重置按钮 -->
          <n-button
            size="small"
            type="default"
            :loading="isLoading"
            @click="handleResetConfig"
          >
            重置
          </n-button>
        </div>

        <!-- 字型選擇器 -->
        <n-select
          :value="fontConfig.font_family"
          :options="fontFamilyOptions.map(opt => ({ label: opt.name, value: opt.id }))"
          :loading="isLoading"
          size="small"
          @update:value="handleFontFamilyChange"
        />

        <!-- 自訂字型輸入（当選擇自訂時顯示） -->
        <n-collapse-transition :show="fontConfig.font_family === 'custom'">
          <div class="mt-3">
            <div class="text-xs opacity-60 mb-2">
              自訂字型CSS值
            </div>
            <n-input-group>
              <n-input
                v-model:value="customFontInput"
                placeholder="例如: 'MyFont', Arial, sans-serif"
                size="small"
                :disabled="isLoading"
                @keyup.enter="handleCustomFontFamilyChange"
              />
              <n-button
                type="primary"
                size="small"
                :loading="isLoading"
                @click="handleCustomFontFamilyChange"
              >
                應用
              </n-button>
            </n-input-group>
          </div>
        </n-collapse-transition>

        <div class="text-xs opacity-50 mt-2">
          當前: {{ currentFontFamilyName }}
        </div>
      </div>

      <!-- 快速應用字型 -->
      <div>
        <div class="flex items-center mb-3">
          <div class="w-1.5 h-1.5 bg-orange-500 rounded-full mr-3 flex-shrink-0" />
          <div class="flex-1">
            <div class="text-sm font-medium leading-relaxed">
              快速應用字型
            </div>
            <div class="text-xs opacity-60">
              輸入系統中已安装的字型名称
            </div>
          </div>
        </div>

        <n-input-group>
          <n-input
            v-model:value="fontNameInput"
            placeholder="例如: Microsoft YaHei, 微软雅黑, PingFang SC"
            size="small"
            :disabled="isLoading"
            @keyup.enter="handleFontNameApply"
          />
          <n-button
            type="primary"
            size="small"
            :loading="isLoading"
            @click="handleFontNameApply"
          >
            應用
          </n-button>
        </n-input-group>

        <div class="text-xs opacity-50 mt-2">
          常用字型：微软雅黑、苹方、思源黑体、JetBrains Mono 等
        </div>
      </div>

      <!-- 字型大小設定 -->
      <div>
        <div class="flex items-center mb-3">
          <div class="w-1.5 h-1.5 bg-orange-500 rounded-full mr-3 flex-shrink-0" />
          <div class="flex-1">
            <div class="text-sm font-medium leading-relaxed">
              字型大小
            </div>
            <div class="text-xs opacity-60">
              調整應用界面的字型大小
            </div>
          </div>
        </div>
        <n-space>
          <n-button
            v-for="option in fontSizeOptions"
            :key="option.id"
            :type="fontConfig.font_size === option.id ? 'primary' : 'default'"
            size="small"
            :loading="isLoading"
            @click="handleFontSizeChange(option.id)"
          >
            {{ option.name }}
          </n-button>
        </n-space>
        <div class="text-xs opacity-50 mt-1">
          當前: {{ currentFontSizeName }} ({{ (currentFontScale * 100).toFixed(0) }}%)
        </div>
      </div>

      <!-- 字型预览 -->
      <div>
        <div class="flex items-center justify-between mb-3">
          <div class="flex items-center">
            <div class="w-1.5 h-1.5 bg-orange-500 rounded-full mr-3 flex-shrink-0" />
            <div>
              <div class="text-sm font-medium leading-relaxed">
                字型预览
              </div>
              <div class="text-xs opacity-60">
                实時预览當前字型效果
              </div>
            </div>
          </div>
        </div>
        <div
          class="bg-black-200 rounded p-4 border border-black-300 transition-all duration-200"
          :style="{
            fontFamily: currentFontFamily,
            fontSize: `${currentFontScale}rem`,
          }"
        >
          <div class="mb-3 font-medium text-lg">
            寸止 - AI 對話持续工具
          </div>
          <div class="mb-3 opacity-80">
            The quick brown fox jumps over the lazy dog.
          </div>
          <div class="mb-3 opacity-80">
            微软雅黑 苹方 思源黑体 Source Han Sans
          </div>
          <div class="text-sm opacity-60 mb-2">
            ABCDEFGHIJKLMNOPQRSTUVWXYZ
          </div>
          <div class="text-sm opacity-60 mb-2">
            abcdefghijklmnopqrstuvwxyz
          </div>
          <div class="text-sm opacity-60">
            0123456789 !@#$%^&amp;*()_+-=[]{}|;:&apos;&quot;,./?
          </div>
        </div>
        <div class="text-xs opacity-50 mt-2 space-y-1">
          <div>當前字型: {{ currentFontFamily }}</div>
        </div>
      </div>
    </n-space>
  </div>
</template>
