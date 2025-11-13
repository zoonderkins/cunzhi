<script setup lang="ts">
import type { CustomPrompt, CustomPromptConfig } from '../../types/popup'
import { invoke } from '@tauri-apps/api/core'
import { emit } from '@tauri-apps/api/event'
import { useMessage } from 'naive-ui'
import { onMounted, ref } from 'vue'

const message = useMessage()

// 設定狀態
const config = ref<CustomPromptConfig>({
  prompts: [],
  enabled: true,
  maxPrompts: 50,
})

// UI狀態
const loading = ref(false)
const showAddDialog = ref(false)
const showEditDialog = ref(false)
const showDeleteDialog = ref(false)
const editingPrompt = ref<CustomPrompt | null>(null)
const deletingPromptId = ref<string>('')

// 新prompt表單
const newPrompt = ref({
  name: '',
  content: '',
  description: '',
  type: 'normal' as 'normal' | 'conditional',
  condition_text: '',
  template_true: '',
  template_false: '',
  current_state: false,
})

// 載入設定
async function loadConfig() {
  try {
    loading.value = true
    const result = await invoke('get_custom_prompt_config')
    config.value = result as CustomPromptConfig
    // 按sort_order排序
    config.value.prompts.sort((a, b) => a.sort_order - b.sort_order)
  }
  catch (error) {
    console.error('載入自訂prompt設定失敗:', error)
    message.error('載入設定失敗')
  }
  finally {
    loading.value = false
  }
}

// 切换啟用狀態
async function toggleEnabled() {
  try {
    await invoke('set_custom_prompt_enabled', { enabled: config.value.enabled })

    // 傳送事件通知其他元件更新
    await emit('custom-prompt-updated')

    message.success(config.value.enabled ? '已啟用快捷模板功能' : '已禁用快捷模板功能')
  }
  catch (error) {
    console.error('更新啟用狀態失敗:', error)
    message.error('更新失敗')
    // 回滚狀態
    config.value.enabled = !config.value.enabled
  }
}

// 新增prompt
async function addPrompt() {
  if (!newPrompt.value.name.trim()) {
    message.warning('請填写名称')
    return
  }

  // 上下文追加的驗證
  if (newPrompt.value.type === 'conditional') {
    if (!newPrompt.value.condition_text.trim()) {
      message.warning('請填写条件描述')
      return
    }
    if (!newPrompt.value.template_true.trim() && !newPrompt.value.template_false.trim()) {
      message.warning('請至少填写一个模板內容')
      return
    }
  }

  try {
    const prompt: CustomPrompt = {
      id: `custom_${Date.now()}`,
      name: newPrompt.value.name.trim(),
      content: newPrompt.value.content, // 允许空內容
      description: newPrompt.value.description.trim() || undefined,
      sort_order: config.value.prompts.length + 1,
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
      type: newPrompt.value.type,
      condition_text: newPrompt.value.type === 'conditional' ? newPrompt.value.condition_text.trim() || undefined : undefined,
      template_true: newPrompt.value.type === 'conditional' ? newPrompt.value.template_true.trim() || undefined : undefined,
      template_false: newPrompt.value.type === 'conditional' ? newPrompt.value.template_false.trim() || undefined : undefined,
      current_state: newPrompt.value.type === 'conditional' ? newPrompt.value.current_state : undefined,
    }

    await invoke('add_custom_prompt', { prompt })
    config.value.prompts.push(prompt)

    // 傳送事件通知其他元件更新
    await emit('custom-prompt-updated')

    // 重置表單
    newPrompt.value = {
      name: '',
      content: '',
      description: '',
      type: 'normal',
      condition_text: '',
      template_true: '',
      template_false: '',
      current_state: false,
    }
    showAddDialog.value = false
    message.success('新增成功')
  }
  catch (error) {
    console.error('新增prompt失敗:', error)
    message.error(`新增失敗: ${error}`)
  }
}

// 編輯prompt
function editPrompt(prompt: CustomPrompt) {
  editingPrompt.value = { ...prompt }
  showEditDialog.value = true
}

// 更新prompt
async function updatePrompt() {
  if (!editingPrompt.value)
    return

  // 上下文追加的驗證
  if (editingPrompt.value.type === 'conditional') {
    if (!editingPrompt.value.condition_text?.trim()) {
      message.warning('請填写条件描述')
      return
    }
    if (!editingPrompt.value.template_true?.trim() && !editingPrompt.value.template_false?.trim()) {
      message.warning('請至少填写一个模板內容')
      return
    }
  }

  try {
    editingPrompt.value.updated_at = new Date().toISOString()
    await invoke('update_custom_prompt', { prompt: editingPrompt.value })

    // 更新本地狀態
    const index = config.value.prompts.findIndex(p => p.id === editingPrompt.value!.id)
    if (index !== -1) {
      config.value.prompts[index] = { ...editingPrompt.value }
    }

    // 傳送事件通知其他元件更新
    await emit('custom-prompt-updated')

    showEditDialog.value = false
    editingPrompt.value = null
    message.success('更新成功')
  }
  catch (error) {
    console.error('更新prompt失敗:', error)
    message.error(`更新失敗: ${error}`)
  }
}

// 顯示刪除確認對話框
function showDeleteConfirm(promptId: string) {
  deletingPromptId.value = promptId
  showDeleteDialog.value = true
}

// 刪除prompt
async function deletePrompt() {
  if (!deletingPromptId.value)
    return

  try {
    await invoke('delete_custom_prompt', { promptId: deletingPromptId.value })
    config.value.prompts = config.value.prompts.filter(p => p.id !== deletingPromptId.value)

    // 傳送事件通知其他元件更新
    await emit('custom-prompt-updated')

    message.success('刪除成功')
  }
  catch (error) {
    console.error('刪除prompt失敗:', error)
    message.error(`刪除失敗: ${error}`)
  }
  finally {
    showDeleteDialog.value = false
    deletingPromptId.value = ''
  }
}

// 取消編輯
function cancelEdit() {
  showEditDialog.value = false
  editingPrompt.value = null
}

// 元件挂载時載入設定
onMounted(() => {
  loadConfig()
})
</script>

<template>
  <div class="p-4">
    <!-- 啟用开关 -->
    <div class="flex items-center justify-between mb-6">
      <div>
        <div class="text-sm opacity-60">
          是否开啟快捷模板功能
        </div>
      </div>
      <n-switch
        v-model:value="config.enabled"
        @update:value="toggleEnabled"
      />
    </div>

    <div v-if="config.enabled" data-guide="custom-prompt-settings">
      <!-- 新增按钮 -->
      <div class="flex justify-between items-center mb-4">
        <div class="text-sm opacity-60">
          已建立 {{ config.prompts.length }} 个模板
        </div>
        <n-button
          type="primary"
          size="small"
          :disabled="config.prompts.length >= config.maxPrompts"
          data-guide="add-prompt-button"
          @click="showAddDialog = true"
        >
          <template #icon>
            <div class="i-carbon-add w-4 h-4" />
          </template>
          新增模板
        </n-button>
      </div>

      <!-- Prompt列表 -->
      <div v-if="loading" class="text-center py-8">
        <n-spin size="medium" />
      </div>

      <div v-else-if="config.prompts.length === 0" class="text-center py-8 opacity-60">
        <div class="i-carbon-document-blank text-4xl mb-2" />
        <div>暂无快捷模板</div>
      </div>

      <div v-else class="space-y-3">
        <div class="space-y-3">
          <div
            v-for="prompt in config.prompts"
            :key="prompt.id"
            class="bg-black-50 rounded-lg p-4 border border-black-200 shadow-sm hover:border-black-300 transition-colors"
          >
            <div class="flex justify-between items-start mb-2">
              <div class="flex-1">
                <div class="flex items-center gap-2 mb-1">
                  <span class="font-medium text-white">{{ prompt.name }}</span>
                  <!-- 類型标识 -->
                  <n-tag v-if="prompt.type === 'conditional'" size="small" type="info">
                    上下文追加
                  </n-tag>
                  <n-tag v-else size="small" type="default">
                    快捷模板
                  </n-tag>
                </div>
                <div v-if="prompt.description" class="text-sm opacity-60 mb-2">
                  {{ prompt.description }}
                </div>

                <!-- 快捷模板內容顯示 -->
                <div v-if="prompt.type !== 'conditional'" class="text-sm bg-black-100 p-2 rounded border border-black-200">
                  <span v-if="prompt.content.trim()">{{ prompt.content }}</span>
                  <span v-else class="italic opacity-60">（空內容 - 清空輸入框）</span>
                </div>

                <!-- 上下文追加內容顯示 -->
                <div v-else class="space-y-2">
                  <div class="text-sm bg-black-100 p-2 rounded border border-black-200">
                    <div class="font-medium mb-1">
                      条件：{{ prompt.condition_text }}
                    </div>
                    <div class="space-y-1 text-xs">
                      <div v-if="prompt.template_true">
                        <span class="text-green-400">✓ 开啟：</span>{{ prompt.template_true }}
                      </div>
                      <div v-if="prompt.template_false">
                        <span class="text-red-400">✗ 關閉：</span>{{ prompt.template_false }}
                      </div>
                      <div class="text-gray-700 dark:text-white">
                        當前狀態：{{ prompt.current_state ? '开啟' : '關閉' }}
                      </div>
                    </div>
                  </div>
                </div>
              </div>
              <div class="flex gap-1 ml-4">
                <n-button size="small" quaternary @click="editPrompt(prompt)">
                  <template #icon>
                    <div class="i-carbon-edit w-4 h-4" />
                  </template>
                </n-button>
                <n-button size="small" quaternary type="error" @click="showDeleteConfirm(prompt.id)">
                  <template #icon>
                    <div class="i-carbon-trash-can w-4 h-4" />
                  </template>
                </n-button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 新增對話框 -->
    <n-modal v-model:show="showAddDialog" preset="card" title="新增快捷模板" style="width: 600px">
      <n-form :model="newPrompt" label-placement="top">
        <n-form-item label="名称" required>
          <n-input v-model:value="newPrompt.name" placeholder="輸入模板名称" />
        </n-form-item>
        <n-form-item label="描述">
          <n-input v-model:value="newPrompt.description" placeholder="简短描述这个模板的用途" />
        </n-form-item>

        <!-- 模板類型選擇 -->
        <n-form-item label="類型">
          <n-radio-group v-model:value="newPrompt.type">
            <n-radio value="normal">
              快捷模板
            </n-radio>
            <n-radio value="conditional">
              上下文追加
            </n-radio>
          </n-radio-group>
        </n-form-item>

        <!-- 快捷模板內容 -->
        <n-form-item v-if="newPrompt.type === 'normal'" label="內容">
          <n-input
            v-model:value="newPrompt.content"
            type="textarea"
            placeholder="輸入模板內容（留空可實作清空輸入框效果）"
            :autosize="{ minRows: 4, maxRows: 8 }"
          />
        </n-form-item>

        <!-- 上下文追加字段 -->
        <template v-if="newPrompt.type === 'conditional'">
          <n-form-item label="条件描述" required>
            <n-input v-model:value="newPrompt.condition_text" placeholder="例如：是否使用TypeScript" />
          </n-form-item>
          <n-form-item label="开啟時的內容">
            <n-input
              v-model:value="newPrompt.template_true"
              type="textarea"
              placeholder="例如：✔️需要使用TypeScript"
              :autosize="{ minRows: 2, maxRows: 4 }"
            />
          </n-form-item>
          <n-form-item label="關閉時的內容">
            <n-input
              v-model:value="newPrompt.template_false"
              type="textarea"
              placeholder="例如：❌切记，不要使用TypeScript"
              :autosize="{ minRows: 2, maxRows: 4 }"
            />
          </n-form-item>
          <n-form-item label="當前狀態">
            <n-switch v-model:value="newPrompt.current_state">
              <template #checked>
                开啟
              </template>
              <template #unchecked>
                關閉
              </template>
            </n-switch>
          </n-form-item>
        </template>
      </n-form>
      <template #footer>
        <div class="flex justify-end gap-2">
          <n-button @click="showAddDialog = false">
            取消
          </n-button>
          <n-button type="primary" @click="addPrompt">
            新增
          </n-button>
        </div>
      </template>
    </n-modal>

    <!-- 編輯對話框 -->
    <n-modal v-model:show="showEditDialog" preset="card" title="編輯快捷模板" style="width: 600px">
      <n-form v-if="editingPrompt" :model="editingPrompt" label-placement="top">
        <n-form-item label="名称" required>
          <n-input v-model:value="editingPrompt.name" placeholder="輸入模板名称" />
        </n-form-item>
        <n-form-item label="描述">
          <n-input v-model:value="editingPrompt.description" placeholder="简短描述这个模板的用途" />
        </n-form-item>

        <!-- 模板類型選擇 -->
        <n-form-item label="類型">
          <n-radio-group v-model:value="editingPrompt.type">
            <n-radio value="normal">
              快捷模板
            </n-radio>
            <n-radio value="conditional">
              上下文追加
            </n-radio>
          </n-radio-group>
        </n-form-item>

        <!-- 快捷模板內容 -->
        <n-form-item v-if="editingPrompt.type === 'normal' || !editingPrompt.type" label="內容">
          <n-input
            v-model:value="editingPrompt.content"
            type="textarea"
            placeholder="輸入模板內容（留空可實作清空輸入框效果）"
            :autosize="{ minRows: 4, maxRows: 8 }"
          />
        </n-form-item>

        <!-- 上下文追加字段 -->
        <template v-if="editingPrompt.type === 'conditional'">
          <n-form-item label="条件描述" required>
            <n-input v-model:value="editingPrompt.condition_text" placeholder="例如：是否使用TypeScript" />
          </n-form-item>
          <n-form-item label="开啟時的內容">
            <n-input
              v-model:value="editingPrompt.template_true"
              type="textarea"
              placeholder="例如：✔️需要使用TypeScript"
              :autosize="{ minRows: 2, maxRows: 4 }"
            />
          </n-form-item>
          <n-form-item label="關閉時的內容">
            <n-input
              v-model:value="editingPrompt.template_false"
              type="textarea"
              placeholder="例如：❌切记，不要使用TypeScript"
              :autosize="{ minRows: 2, maxRows: 4 }"
            />
          </n-form-item>
          <n-form-item label="當前狀態">
            <n-switch v-model:value="editingPrompt.current_state">
              <template #checked>
                开啟
              </template>
              <template #unchecked>
                關閉
              </template>
            </n-switch>
          </n-form-item>
        </template>
      </n-form>
      <template #footer>
        <div class="flex justify-end gap-2">
          <n-button @click="cancelEdit">
            取消
          </n-button>
          <n-button type="primary" @click="updatePrompt">
            儲存
          </n-button>
        </div>
      </template>
    </n-modal>

    <!-- 刪除確認對話框 -->
    <n-modal v-model:show="showDeleteDialog" preset="dialog" title="確認刪除">
      <div>确定要刪除这个模板吗？此操作無法復原。</div>
      <template #action>
        <div class="flex justify-end gap-2">
          <n-button @click="showDeleteDialog = false">
            取消
          </n-button>
          <n-button type="error" @click="deletePrompt">
            确定刪除
          </n-button>
        </div>
      </template>
    </n-modal>
  </div>
</template>

<style scoped>
/* 拖拽排序样式 */
.sortable-item {
  cursor: default;
  transition: all 0.2s ease;
}

.sortable-ghost {
  opacity: 0.5;
  transform: scale(0.95);
  background: rgba(59, 130, 246, 0.1) !important;
  border: 2px dashed rgba(59, 130, 246, 0.3) !important;
}

.sortable-chosen {
  cursor: grabbing !important;
  transform: scale(1.02);
}

.sortable-drag {
  opacity: 0.8;
  transform: rotate(2deg);
  box-shadow: 0 8px 25px rgba(0, 0, 0, 0.15);
  z-index: 1000;
}
</style>
