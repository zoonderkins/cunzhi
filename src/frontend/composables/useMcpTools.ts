import { invoke } from '@tauri-apps/api/core'
import { computed, ref } from 'vue'

// MCP工具設定介面
export interface MCPToolConfig {
  id: string
  name: string
  description: string
  enabled: boolean
  can_disable: boolean
  icon: string
  icon_bg: string
  dark_icon_bg: string
}

// 全局MCP工具狀態
const mcpTools = ref<MCPToolConfig[]>([])
const loading = ref(false)
const error = ref<string | null>(null)

// 计算属性：启用的工具
const enabledTools = computed(() => mcpTools.value.filter(tool => tool.enabled))

// 计算属性：工具统计
const toolStats = computed(() => ({
  total: mcpTools.value.length,
  enabled: enabledTools.value.length,
  disabled: mcpTools.value.length - enabledTools.value.length,
}))

// 載入MCP工具設定
async function loadMcpTools() {
  try {
    loading.value = true
    error.value = null
    const tools = await invoke('get_mcp_tools_config') as MCPToolConfig[]
    mcpTools.value = tools
    console.log('✅ MCP工具設定已載入:', tools)
  }
  catch (err) {
    error.value = `載入MCP工具設定失敗: ${err}`
    console.error('❌ 載入MCP工具設定失敗:', err)
    throw err
  }
  finally {
    loading.value = false
  }
}

// 切换工具启用狀態
async function toggleTool(toolId: string) {
  const tool = mcpTools.value.find(t => t.id === toolId)
  if (!tool || !tool.can_disable) {
    throw new Error('工具不存在或不可禁用')
  }

  try {
    const newEnabled = !tool.enabled
    await invoke('set_mcp_tool_enabled', {
      toolId,
      enabled: newEnabled,
    })

    // 更新本地狀態
    tool.enabled = newEnabled

    console.log(`✅ 工具 ${toolId} 狀態已更新为: ${newEnabled}`)

    return {
      toolId,
      enabled: newEnabled,
      needsReconnect: true,
    }
  }
  catch (err) {
    error.value = `更新MCP工具狀態失敗: ${err}`
    console.error('❌ 更新MCP工具狀態失敗:', err)
    throw err
  }
}

// 重置MCP工具設定
async function resetMcpTools() {
  try {
    loading.value = true
    error.value = null
    await invoke('reset_mcp_tools_config')
    await loadMcpTools() // 重新載入設定
    console.log('✅ MCP工具設定已重置')
  }
  catch (err) {
    error.value = `重置MCP工具設定失敗: ${err}`
    console.error('❌ 重置MCP工具設定失敗:', err)
    throw err
  }
  finally {
    loading.value = false
  }
}

// 獲取工具狀態
function getToolStatus(toolId: string): boolean {
  const tool = mcpTools.value.find(t => t.id === toolId)
  return tool?.enabled ?? false
}

// 檢查工具是否可禁用
function canDisableTool(toolId: string): boolean {
  const tool = mcpTools.value.find(t => t.id === toolId)
  return tool?.can_disable ?? false
}

// 全局MCP工具管理composable
export function useMcpTools() {
  return {
    // 狀態
    mcpTools: mcpTools.value,
    loading: loading.value,
    error: error.value,

    // 计算属性
    enabledTools,
    toolStats,

    // 方法
    loadMcpTools,
    toggleTool,
    resetMcpTools,
    getToolStatus,
    canDisableTool,
  }
}

// 響應式狀態访问（用于模板）
export function useMcpToolsReactive() {
  return {
    // 響應式狀態
    mcpTools,
    loading,
    error,

    // 计算属性
    enabledTools,
    toolStats,

    // 方法
    loadMcpTools,
    toggleTool,
    resetMcpTools,
    getToolStatus,
    canDisableTool,
  }
}

// 初始化函數（在應用啟動时呼叫）
export async function initMcpTools() {
  try {
    await loadMcpTools()
  }
  catch (err) {
    console.error('初始化MCP工具失敗:', err)
  }
}
