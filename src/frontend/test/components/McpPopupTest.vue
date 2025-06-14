<script setup lang="ts">
import { ref } from 'vue'
import McpPopup from '../../components/McpPopup.vue'

// Props
const props = defineProps<{
  showControls?: boolean
}>()

// 默认显示控制面板
const showControls = ref(props.showControls !== false)

const currentTheme = ref('dark')
const showPopup = ref(true)

// 模拟不同类型的 MCP 请求
const requestTemplates = [
  {
    name: '基础文本请求',
    request: {
      id: 'test-1',
      message: `# 基础文本请求

这是一个简单的文本请求示例，用于测试基本的 Markdown 渲染功能。

请输入您的回复：`,
      predefined_options: undefined,
      is_markdown: true
    }
  },
  {
    name: '预定义选项请求',
    request: {
      id: 'test-2',
      message: `# 预定义选项请求

请选择您需要的操作：`,
      predefined_options: [
        '查看项目结构',
        '生成代码文档',
        '运行测试用例',
        '部署到生产环境'
      ],
      is_markdown: true
    }
  },
  {
    name: '复杂 Markdown 请求',
    request: {
      id: 'test-3',
      message: `# 复杂 Markdown 请求

这是一个包含多种 Markdown 元素的请求示例。

## 代码示例

\`\`\`javascript
function greet(name) {
  console.log(\`Hello, \${name}!\`);
}

greet('World');
\`\`\`

## 列表示例

- 项目一
- 项目二
  - 子项目 A
  - 子项目 B
- 项目三

## 表格示例

| 功能 | 状态 | 描述 |
|------|------|------|
| 主题切换 | ✅ | 支持浅色/深色主题 |
| 组件测试 | ✅ | 完整的组件测试环境 |
| 实时预览 | ✅ | 修改即时生效 |

请选择您的操作：`,
      predefined_options: [
        '继续开发',
        '提交代码',
        '创建文档',
        '运行测试'
      ],
      is_markdown: true
    }
  }
]

const currentTemplate = ref(0)
const currentRequest = ref(requestTemplates[0].request)

function switchTemplate(index: number) {
  currentTemplate.value = index
  currentRequest.value = requestTemplates[index].request
}

function handleResponse(response: any) {
  console.log('MCP 响应:', response)
  alert(`收到响应: ${JSON.stringify(response, null, 2)}`)
}

function handleCancel() {
  console.log('MCP 取消')
  alert('用户取消了操作')
}

function handleThemeChange(theme: string) {
  currentTheme.value = theme
  console.log('主题切换:', theme)
}

function togglePopup() {
  showPopup.value = !showPopup.value
}
</script>

<template>
  <div class="mcp-popup-test">
    <!-- 控制面板模式 -->
    <div v-if="showControls">
      <n-card title="MCP 弹窗测试 - 真实组件">
        <template #header-extra>
          <n-space>
            <n-tag size="small" type="info">
              引用: McpPopup.vue
            </n-tag>
            <n-button size="small" @click="togglePopup">
              {{ showPopup ? '隐藏弹窗' : '显示弹窗' }}
            </n-button>
          </n-space>
        </template>

        <!-- 控制面板 -->
        <div class="control-panel">
          <n-card title="测试控制" size="small">
            <n-space vertical>
              <div class="control-section">
                <h4>请求模板:</h4>
                <n-space>
                  <n-button
                    v-for="(template, index) in requestTemplates"
                    :key="index"
                    :type="currentTemplate === index ? 'primary' : 'default'"
                    size="small"
                    @click="switchTemplate(index)"
                  >
                    {{ template.name }}
                  </n-button>
                </n-space>
              </div>

              <div class="control-section">
                <h4>当前状态:</h4>
                <n-space vertical size="small">
                  <n-space align="center" justify="space-between">
                    <span>主题:</span>
                    <n-tag size="small" :type="currentTheme === 'dark' ? 'warning' : 'info'">
                      {{ currentTheme }}
                    </n-tag>
                  </n-space>

                  <n-space align="center" justify="space-between">
                    <span>弹窗:</span>
                    <n-tag size="small" :type="showPopup ? 'success' : 'default'">
                      {{ showPopup ? '显示' : '隐藏' }}
                    </n-tag>
                  </n-space>

                  <n-space align="center" justify="space-between">
                    <span>选项数量:</span>
                    <n-tag size="small" type="info">
                      {{ currentRequest.predefined_options?.length || 0 }}
                    </n-tag>
                  </n-space>
                </n-space>
              </div>
            </n-space>
          </n-card>
        </div>

        <!-- 真实的 MCP 弹窗组件 -->
        <div v-if="showPopup" class="popup-container">
          <!-- 模拟窗口背景遮罩 -->
          <div class="popup-overlay">
            <McpPopup
              :request="currentRequest"
              :current-theme="currentTheme"
              @response="handleResponse"
              @cancel="handleCancel"
              @theme-change="handleThemeChange"
            />
          </div>
        </div>

        <!-- 说明信息 -->
        <div class="info-panel">
          <n-card title="测试说明" size="small">
            <n-space vertical size="small">
              <div class="flex items-center text-sm">
                <div class="w-1.5 h-1.5 bg-green-500 rounded-full mr-3 flex-shrink-0"></div>
                这是真实的 McpPopup 组件，所有修改都会实时反映
              </div>
              <div class="flex items-center text-sm">
                <div class="w-1.5 h-1.5 bg-green-500 rounded-full mr-3 flex-shrink-0"></div>
                可以切换不同的请求模板测试各种场景
              </div>
              <div class="flex items-center text-sm">
                <div class="w-1.5 h-1.5 bg-green-500 rounded-full mr-3 flex-shrink-0"></div>
                支持 Markdown 渲染、代码高亮、预定义选项等功能
              </div>
              <div class="flex items-center text-sm">
                <div class="w-1.5 h-1.5 bg-green-500 rounded-full mr-3 flex-shrink-0"></div>
                响应和取消事件会在控制台输出和弹窗显示
              </div>
              <div class="flex items-center text-sm">
                <div class="w-1.5 h-1.5 bg-blue-500 rounded-full mr-3 flex-shrink-0"></div>
                <span class="opacity-70">src/frontend/components/McpPopup.vue</span>
              </div>
            </n-space>
          </n-card>
        </div>
      </n-card>
    </div>

    <!-- 纯净模式 - 只显示弹窗 -->
    <div v-else class="pure-mode">
      <McpPopup
        :request="currentRequest"
        :current-theme="currentTheme"
        @response="handleResponse"
        @cancel="handleCancel"
        @theme-change="handleThemeChange"
      />
    </div>
  </div>
</template>

<style scoped>
.mcp-popup-test {
  max-width: 1200px;
  margin: 0 auto;
}

.control-panel {
  margin-bottom: 20px;
}

.control-section {
  margin-bottom: 15px;
}

.control-section h4 {
  margin: 0 0 8px 0;
  color: var(--text-color);
  font-size: 0.9rem;
  font-weight: 500;
}

.popup-container {
  margin: 20px 0;
  border: 2px dashed var(--border-color);
  border-radius: 8px;
  padding: 0;
  background: var(--card-color);
  position: relative;
  min-height: 400px;
  overflow: hidden;
}

.popup-container::before {
  content: '真实 MCP 弹窗预览 - 模拟全屏弹窗效果';
  position: absolute;
  top: -10px;
  left: 20px;
  background: var(--card-color);
  padding: 0 10px;
  font-size: 0.8rem;
  color: var(--text-color);
  opacity: 0.6;
  z-index: 10;
}

.popup-overlay {
  position: relative;
  width: 100%;
  height: 100%;
  min-height: 400px;
  background: rgba(0, 0, 0, 0.1);
  display: flex;
  align-items: center;
  justify-content: center;
}

.popup-overlay :deep(.popup-container) {
  position: relative !important;
  width: 100% !important;
  height: 100% !important;
  max-width: none !important;
  max-height: none !important;
  border-radius: 0 !important;
}

.info-panel {
  margin-top: 20px;
}



/* 纯净模式 */
.pure-mode {
  width: 100%;
  height: 100%;
}

.pure-mode :deep(.popup-container) {
  position: relative !important;
  inset: 0 !important;
  width: 100% !important;
  height: 100% !important;
}
</style>
