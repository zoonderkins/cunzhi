<script setup>
import { useI18n } from '../../i18n'

const { t } = useI18n()

defineProps({
  currentTheme: {
    type: String,
    required: true,
  },
})

defineEmits(['themeChange'])
</script>

<template>
  <!-- 設定内容 -->
  <div class="flex items-center justify-between">
    <div class="flex items-center">
      <div class="w-1.5 h-1.5 bg-primary-500 rounded-full mr-3 flex-shrink-0" />
      <div>
        <div class="text-sm font-medium leading-relaxed">
          {{ t('settings.theme.label') }}
        </div>
        <div class="text-xs opacity-60">
          {{ t('settings.theme.hint') }}
        </div>
      </div>
    </div>
    <n-space>
      <!-- 浅色主題 -->
      <n-button
        :type="currentTheme === 'light' ? 'primary' : 'default'"
        size="small"
        @click="$emit('themeChange', 'light')"
      >
        <template #icon>
          <div
            class="w-3 h-3 rounded-full border transition-all duration-200"
            :style="{
              backgroundColor: '#ffffff',
              borderColor: currentTheme === 'light' ? '#14b8a6' : '#d1d5db',
            }"
          />
        </template>
        {{ t('settings.theme.light') }}
      </n-button>

      <!-- 深色主題 -->
      <n-button
        :type="currentTheme === 'dark' ? 'primary' : 'default'"
        size="small"
        @click="$emit('themeChange', 'dark')"
      >
        <template #icon>
          <div
            class="w-3 h-3 rounded-full border transition-all duration-200"
            :style="{
              backgroundColor: '#1f2937',
              borderColor: currentTheme === 'dark' ? '#14b8a6' : '#d1d5db',
            }"
          />
        </template>
        {{ t('settings.theme.dark') }}
      </n-button>
    </n-space>
  </div>
</template>
