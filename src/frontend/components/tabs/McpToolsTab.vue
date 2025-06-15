<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { useMessage } from 'naive-ui'
import { onMounted, ref } from 'vue'

// MCP工具配置接口
interface MCPToolConfig {
  id: string
  name: string
  description: string
  enabled: boolean
  can_disable: boolean
  icon: string
  icon_bg: string
  dark_icon_bg: string
}

// MCP工具配置状态
const mcpTools = ref<MCPToolConfig[]>([])
const loading = ref(true)
const needsReconnect = ref(false)

// Naive UI 消息实例
const message = useMessage()

// 加载MCP工具配置
async function loadMcpTools() {
  try {
    loading.value = true
    const tools = await invoke('get_mcp_tools_config') as MCPToolConfig[]
    mcpTools.value = tools
  }
  catch (error) {
    console.error('加载MCP工具配置失败:', error)
    if (message) {
      message.error(`加载MCP工具配置失败: ${error}`)
    }
  }
  finally {
    loading.value = false
  }
}

// 切换工具启用状态
async function toggleTool(toolId: string) {
  const tool = mcpTools.value.find(t => t.id === toolId)
  if (!tool || !tool.can_disable) {
    return
  }

  try {
    const newEnabled = !tool.enabled
    await invoke('set_mcp_tool_enabled', {
      toolId,
      enabled: newEnabled,
    })

    // 更新本地状态
    tool.enabled = newEnabled

    // 显示重连提示
    needsReconnect.value = true

    if (message) {
      message.warning('MCP工具配置已更新，请在MCP客户端中重连服务')
    }
  }
  catch (error) {
    console.error('更新MCP工具状态失败:', error)
    if (message) {
      message.error(`更新MCP工具状态失败: ${error}`)
    }
  }
}

onMounted(() => {
  loadMcpTools()
})
</script>

<template>
  <div class="max-w-3xl mx-auto tab-content">
    <n-space vertical size="large">
      <!-- MCP服务重连提示 -->
      <n-alert
        v-if="needsReconnect"
        title="需要重连MCP服务"
        type="warning"
        closable
        @close="needsReconnect = false"
      >
        <template #icon>
          <div class="i-carbon-connection-signal text-lg" />
        </template>
        MCP工具配置已更改，请在您的MCP客户端中重新连接寸止服务以使更改生效。
      </n-alert>

      <!-- 加载状态 -->
      <div v-if="loading" class="text-center py-8">
        <n-spin size="medium" />
        <div class="mt-2 text-sm opacity-60">
          加载MCP工具配置中...
        </div>
      </div>

      <!-- MCP工具配置卡片 -->
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
            <!-- 左侧内容区域 - 允许收缩但不会挤压右侧 -->
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

              <!-- 标题和副标题 -->
              <div class="flex-1 min-w-0">
                <n-space align="center">
                  <div class="text-lg font-medium tracking-tight">
                    {{ tool.name }}
                  </div>
                  <!-- 状态标签 -->
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
                    已启用
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

        <!-- 工具状态说明 -->
        <div class="flex items-center text-sm leading-relaxed">
          <div
            class="w-1.5 h-1.5 rounded-full mr-3 flex-shrink-0"
            :class="tool.enabled ? 'bg-green-500' : 'bg-gray-400'"
          />
          <span class="opacity-90">
            {{ tool.enabled ? 'MCP服务已启用此工具' : 'MCP服务已禁用此工具' }}
          </span>
        </div>
      </n-card>

      <!-- 底部统计 - 增强可见性 -->
      <div class="text-center py-2">
        <span class="text-sm text-gray-500 dark:text-gray-400 font-medium">
          {{ mcpTools.filter(t => t.enabled).length }} / {{ mcpTools.length }} 个工具已启用
        </span>
      </div>
    </n-space>
  </div>
</template>
