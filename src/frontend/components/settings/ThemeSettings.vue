<script setup>
defineProps({
  currentTheme: {
    type: String,
    required: true,
  },
})

defineEmits(['themeChange'])
</script>

<template>
  <n-card size="small">
    <!-- 卡片头部 -->
    <template #header>
      <n-space align="center">
        <!-- 图标 -->
        <div class="w-10 h-10 rounded-lg bg-purple-100 dark:bg-purple-900 flex items-center justify-center">
          <div class="i-carbon-color-palette text-lg text-gray-700 dark:text-gray-200" />
        </div>

        <!-- 标题和副标题 -->
        <div>
          <div class="text-lg font-medium mb-1 tracking-tight">
            主题设置
          </div>
          <div class="text-sm opacity-60 font-normal">
            选择您喜欢的界面主题
          </div>
        </div>
      </n-space>
    </template>

    <!-- 设置内容 -->
    <div class="flex items-center justify-between">
      <div class="flex items-center">
        <div class="w-1.5 h-1.5 bg-green-500 rounded-full mr-3 flex-shrink-0"></div>
        <div>
          <div class="text-sm font-medium leading-relaxed">
            界面主题
          </div>
          <div class="text-xs opacity-60">
            选择浅色、深色或跟随系统
          </div>
        </div>
      </div>
      <n-space>
        <!-- 浅色主题 -->
        <n-button
          :type="currentTheme === 'light' ? 'primary' : 'default'"
          size="small"
          @click="$emit('themeChange', 'light')"
        >
          <template #icon>
            <div
              class="w-3 h-3 bg-white rounded-full border"
              :class="currentTheme === 'light' ? 'border-white' : 'border-gray-400'"
            />
          </template>
          浅色
        </n-button>

        <!-- 深色主题 -->
        <n-button
          :type="currentTheme === 'dark' ? 'primary' : 'default'"
          size="small"
          @click="$emit('themeChange', 'dark')"
        >
          <template #icon>
            <div
              class="w-3 h-3 bg-gray-800 rounded-full border"
              :class="currentTheme === 'dark' ? 'border-gray-800' : 'border-gray-400'"
            />
          </template>
          深色
        </n-button>

        <!-- 系统跟随 -->
        <n-button
          :type="currentTheme === 'system' ? 'primary' : 'default'"
          size="small"
          @click="$emit('themeChange', 'system')"
        >
          <template #icon>
            <div
              class="w-3 h-3 bg-gradient-to-r from-white to-gray-800 rounded-full border"
              :class="currentTheme === 'system' ? 'border-gray-600' : 'border-gray-400'"
            />
          </template>
          跟随
        </n-button>
      </n-space>
    </div>
  </n-card>
</template>
