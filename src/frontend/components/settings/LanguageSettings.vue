<script setup lang="ts">
import { useMessage } from 'naive-ui'
import { useI18n } from '../../i18n'

const { t, locale, setLocale, locales, localeNames } = useI18n()
const message = useMessage()

async function handleLanguageChange(lang: string) {
  try {
    await setLocale(lang as any)
    message.success(t('success.configSaved'))
  }
  catch (error) {
    console.error('Failed to change language:', error)
    message.error(t('errors.saveConfig'))
  }
}
</script>

<template>
  <!-- 設定內容 -->
  <div class="flex items-center justify-between">
    <div class="flex items-center">
      <div class="w-1.5 h-1.5 bg-purple-500 rounded-full mr-3 flex-shrink-0" />
      <div>
        <div class="text-sm font-medium leading-relaxed">
          {{ t('settings.language.label') }}
        </div>
        <div class="text-xs opacity-60">
          {{ t('settings.language.hint') }}
        </div>
      </div>
    </div>
    <n-space>
      <!-- 簡體中文 -->
      <n-button
        :type="locale === 'zh-CN' ? 'primary' : 'default'"
        size="small"
        @click="handleLanguageChange('zh-CN')"
      >
        <template #icon>
          <div class="i-carbon-language text-base" />
        </template>
        {{ localeNames['zh-CN'] }}
      </n-button>

      <!-- 繁體中文 -->
      <n-button
        :type="locale === 'zh-TW' ? 'primary' : 'default'"
        size="small"
        @click="handleLanguageChange('zh-TW')"
      >
        <template #icon>
          <div class="i-carbon-language text-base" />
        </template>
        {{ localeNames['zh-TW'] }}
      </n-button>
    </n-space>
  </div>
</template>

