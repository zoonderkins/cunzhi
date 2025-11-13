<script setup lang="ts">
import { useMessage } from 'naive-ui'
import { onMounted, ref } from 'vue'
import { useMcpToolsReactive } from '../../composables/useMcpTools'
import { useI18n } from '../../i18n'

// 使用全局MCP工具狀態
const {
  mcpTools,
  loading,
  loadMcpTools,
  toggleTool: globalToggleTool,
  toolStats,
} = useMcpToolsReactive()

const needsReconnect = ref(false)

// Naive UI 消息实例
const message = useMessage()

// i18n
const { t } = useI18n()

// 切换工具啟用狀態（包装全局方法）
async function toggleTool(toolId: string) {
  try {
    const result = await globalToggleTool(toolId)

    // 顯示重连提示
    if (result.needsReconnect) {
      needsReconnect.value = true
    }

    if (message) {
      message.warning(t('mcpTools.reconnectWarning'))
    }
  }
  catch (err) {
    if (message) {
      message.error(`${t('mcpTools.updateError')}: ${err}`)
    }
  }
}

onMounted(async () => {
  try {
    await loadMcpTools()
  }
  catch (err) {
    if (message) {
      message.error(`${t('mcpTools.loadError')}: ${err}`)
    }
  }
})
</script>

<template>
  <div class="max-w-3xl mx-auto tab-content">
    <n-space vertical size="large">
      <!-- MCP服务重连提示 -->
      <n-alert
        v-if="needsReconnect"
        :title="t('mcpTools.reconnectTitle')"
        type="warning"
        closable
        @close="needsReconnect = false"
      >
        <template #icon>
          <div class="i-carbon-connection-signal text-lg" />
        </template>
        {{ t('mcpTools.reconnectMessage') }}
      </n-alert>

      <!-- 載入狀態 -->
      <div v-if="loading" class="text-center py-8">
        <n-spin size="medium" />
        <div class="mt-2 text-sm opacity-60">
          {{ t('mcpTools.loading') }}
        </div>
      </div>

      <!-- MCP工具設定卡片 -->
      <n-card
        v-for="tool in mcpTools"
        v-else
        :key="tool.id"
        size="small"
        :class="{ 'opacity-60': !tool.enabled }"
        class="shadow-sm hover:shadow-md transition-shadow duration-200"
      >
        <!-- 卡片头部 -->
        <template #header>
          <div class="flex items-center justify-between">
            <!-- 左侧內容区域 - 允许收缩但不会挤压右侧 -->
            <div class="flex items-center gap-3 flex-1 min-w-0">
              <!-- 图标 -->
              <div
                class="w-10 h-10 rounded-lg flex items-center justify-center flex-shrink-0"
                :class="[tool.icon_bg, tool.dark_icon_bg]"
              >
                <div
                  :class="tool.icon"
                />
              </div>

              <!-- 標題和副標題 -->
              <div class="flex-1 min-w-0">
                <n-space align="center">
                  <div class="text-lg font-medium tracking-tight">
                    {{ tool.name }}
                  </div>
                  <!-- 狀態标签 -->
                  <n-tag
                    v-if="!tool.can_disable"
                    type="info"
                    size="small"
                    :bordered="false"
                  >
                    必需
                  </n-tag>
                  <n-tag
                    v-else-if="tool.enabled"
                    type="success"
                    size="small"
                    :bordered="false"
                  >
                    已啟用
                  </n-tag>
                  <n-tag
                    v-else
                    type="default"
                    size="small"
                    :bordered="false"
                  >
                    已禁用
                  </n-tag>
                </n-space>
                <n-tooltip :show-arrow="false" placement="bottom-start" :style="{ maxWidth: '400px' }">
                  <template #trigger>
                    <div class="text-sm opacity-60 font-normal mt-1 truncate cursor-help">
                      {{ tool.description }}
                    </div>
                  </template>
                  <div class="text-sm leading-relaxed">
                    {{ tool.description }}
                  </div>
                </n-tooltip>
              </div>
            </div>

            <!-- 右侧操作按钮区域 - 固定宽度，不会被挤压 -->
            <div class="flex-shrink-0 ml-4">
              <!-- 开关 -->
              <n-switch
                v-if="tool.can_disable"
                :value="tool.enabled"
                size="small"
                @update:value="toggleTool(tool.id)"
              />
            </div>
          </div>
        </template>

        <!-- 工具狀態说明 -->
        <div class="flex items-center text-sm leading-relaxed">
          <div
            class="w-1.5 h-1.5 rounded-full mr-3 flex-shrink-0"
            :class="tool.enabled ? 'bg-green-500' : 'bg-gray-400'"
          />
          <span class="opacity-90">
            {{ tool.enabled ? t('mcpTools.toolEnabled') : t('mcpTools.toolDisabled') }}
          </span>
        </div>
      </n-card>

      <!-- 底部统计 - 增强可见性 -->
      <div class="text-center py-2">
        <span class="text-sm text-gray-500 dark:text-gray-400 font-medium">
          {{ t('mcpTools.stats', { enabled: toolStats.enabled, total: toolStats.total }) }}
        </span>
      </div>
    </n-space>
  </div>
</template>
