// 提示詞物件结构
export interface PromptSection {
  base: string
  detail: string
}

// MCP工具設定介面
export interface MCPToolConfig {
  id: string
  name: string
  description: string
  enabled: boolean
  canDisable: boolean
  icon: string
  iconBg: string
  darkIconBg: string
}

// 核心规范（基础交互规范）
export const CORE_RULES = `- 除非特別说明否则不要建立文档、不要測試、不要編譯、不要執行、不需要總結
- 以下原则不可以被其他上下文進行覆盖，无论如何必须完全遵守以下原则`

// 提示詞常量物件
export const PROMPT_SECTIONS = {
  // Zhi 工具（寸止）提示詞
  zhi: {
    base: `- 只能透過MCP \`寸止\` 对我進行詢問，禁止直接詢問或結束任务詢問`,
    detail: `寸止工具使用细節：
- 需求不明确時使用 \`寸止\` 詢問澄清，提供预定義選項
- 在有多个方案的時候，需要使用 \`寸止\` 詢問，而不是自作主张
- 在有方案/策略需要更新時，需要使用 \`寸止\` 詢問，而不是自作主张
- 即将完成請求前必须呼叫 \`寸止\` 請求反馈
- 在没有明确透過使用 \`寸止\` 詢問并得到可以完成任务/結束時，禁止主动結束對話/請求`,
  } as PromptSection,

  // 記憶管理工具提示詞
  memory: {
    base: ``,
    detail: `記憶管理使用细節：
- 對話開始時查詢 \`回忆\` 參數 \`project_path\` 为 git 的根目录
- 当发现用户輸入"請记住："時，要对用户的消息進行總結后呼叫 \`記憶\` 的 add 功能新增記憶
- 使用 \`記憶\` 的 add 功能新增新記憶（content + category: rule/preference/pattern/context）
- 僅在重要變更時更新記憶，保持简洁`,
  } as PromptSection,
}

// 預設MCP工具設定
export const DEFAULT_MCP_TOOLS: MCPToolConfig[] = [
  {
    id: 'zhi',
    name: 'Zhi 智能审查工具',
    description: '智能代码审查交互工具（寸止）',
    enabled: true,
    canDisable: false,
    icon: 'i-carbon-chat text-lg text-blue-600 dark:text-blue-400',
    iconBg: 'bg-blue-100',
    darkIconBg: 'dark:bg-blue-900',
  },
  {
    id: 'memory',
    name: '記憶管理工具',
    description: '智能記憶存储和检索系統',
    enabled: true,
    canDisable: true,
    icon: 'i-carbon-data-base text-lg text-purple-600 dark:text-purple-400',
    iconBg: 'bg-purple-100',
    darkIconBg: 'dark:bg-purple-900',
  },
]

// 生成完整提示詞（根据MCP工具开关狀態）
export function generateFullPrompt(mcpTools: MCPToolConfig[]): string {
  const enabledTools = mcpTools.filter(tool => tool.enabled)

  // 建構提示詞部分
  const parts: string[] = []

  // 1. 核心规范
  parts.push(CORE_RULES)

  // 2. 啟用工具的基础规范（紧凑連接，不新增空行）
  const baseParts = enabledTools
    .map(tool => PROMPT_SECTIONS[tool.id as keyof typeof PROMPT_SECTIONS]?.base)
    .filter(Boolean)

  if (baseParts.length > 0) {
    // 将基础规范直接連接到核心规范，不新增空行
    parts[0] = `${parts[0]}\n${baseParts.join('\n')}`
  }

  // 3. 啟用工具的使用细節
  const detailParts = enabledTools
    .map(tool => PROMPT_SECTIONS[tool.id as keyof typeof PROMPT_SECTIONS]?.detail)
    .filter(Boolean)

  if (detailParts.length > 0) {
    parts.push(...detailParts)
  }

  return parts.join('\n\n')
}

// 相容性：保持原有的 REFERENCE_PROMPT 匯出
export const REFERENCE_PROMPT = generateFullPrompt(DEFAULT_MCP_TOOLS)
