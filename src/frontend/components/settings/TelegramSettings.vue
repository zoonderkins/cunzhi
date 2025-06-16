<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { useMessage } from 'naive-ui'
import { onMounted, ref } from 'vue'

interface TelegramConfig {
  enabled: boolean
  bot_token: string
  chat_id: string
  hide_frontend_popup: boolean
}

const emit = defineEmits(['telegramConfigChange'])

// Naive UI 消息实例
const message = useMessage()

// 配置状态
const telegramConfig = ref<TelegramConfig>({
  enabled: false,
  bot_token: '',
  chat_id: '',
  hide_frontend_popup: false,
})

// 测试状态
const isTesting = ref(false)

// 加载Telegram配置
async function loadTelegramConfig() {
  try {
    const config = await invoke('get_telegram_config') as TelegramConfig
    telegramConfig.value = config
  }
  catch (error) {
    console.error('加载Telegram配置失败:', error)
    message.error('加载Telegram配置失败')
  }
}

// 保存配置
async function saveTelegramConfig() {
  try {
    await invoke('set_telegram_config', { telegramConfig: telegramConfig.value })
    message.success('Telegram配置已保存')
    emit('telegramConfigChange', telegramConfig.value)
  }
  catch (error) {
    console.error('保存Telegram配置失败:', error)
    message.error('保存Telegram配置失败')
  }
}

// 切换启用状态
async function toggleTelegramEnabled() {
  telegramConfig.value.enabled = !telegramConfig.value.enabled
  await saveTelegramConfig()
}

async function saveAndTest() {
  if (!telegramConfig.value.bot_token.trim()) {
    message.warning('请输入Bot Token')
    return
  }

  if (!telegramConfig.value.chat_id.trim()) {
    message.warning('请输入Chat ID')
    return
  }

  try {
    isTesting.value = true

    // 先保存配置
    await saveTelegramConfig()

    // 然后测试连接
    const result = await invoke('test_telegram_connection_cmd', {
      botToken: telegramConfig.value.bot_token,
      chatId: telegramConfig.value.chat_id,
    }) as string

    message.success(result)
  }
  catch (error) {
    console.error('测试Telegram连接失败:', error)
    message.error(typeof error === 'string' ? error : '测试连接失败')
  }
  finally {
    isTesting.value = false
  }
}

// 组件挂载时加载配置
onMounted(() => {
  loadTelegramConfig()
})
</script>

<template>
  <n-card size="small">
    <!-- 卡片头部 -->
    <template #header>
      <n-space align="center">
        <!-- 图标 -->
        <div class="w-10 h-10 rounded-lg bg-info/10 dark:bg-info/20 flex items-center justify-center">
          <div class="i-carbon-chat-bot text-lg text-info-600 dark:text-info-400" />
        </div>

        <!-- 标题和副标题 -->
        <div>
          <div class="text-lg font-medium mb-1 tracking-tight">
            Telegram 设置
          </div>
          <div class="text-sm opacity-60 font-normal">
            配置Telegram Bot用于接收通知消息
          </div>
        </div>
      </n-space>
    </template>

    <!-- 设置内容 -->
    <n-space vertical size="large">
      <!-- 启用Telegram Bot -->
      <div class="flex items-center justify-between">
        <div class="flex items-center">
          <div class="w-1.5 h-1.5 bg-info rounded-full mr-3 flex-shrink-0" />
          <div>
            <div class="text-sm font-medium leading-relaxed">
              启用Telegram机器人
            </div>
            <div class="text-xs opacity-60">
              启用后可以通过Telegram Bot接收通知消息
            </div>
          </div>
        </div>
        <n-switch :value="telegramConfig.enabled" size="small" @update:value="toggleTelegramEnabled" />
      </div>

      <!-- 配置项区域 - 条件显示 -->
      <n-collapse-transition :show="telegramConfig.enabled">
        <n-space vertical size="large">
          <!-- Bot Token设置 -->
          <div class="pt-4 border-t border-gray-200 dark:border-gray-700">
            <div class="flex items-start">
              <div class="w-1.5 h-1.5 bg-info rounded-full mr-3 mt-2 flex-shrink-0" />
              <div class="flex-1">
                <div class="text-sm font-medium mb-3 leading-relaxed">
                  Bot Token
                </div>
                <div class="text-xs opacity-60 mb-3">
                  从 @BotFather 获取的Bot Token，用于验证Bot身份
                </div>
                <n-input
                  v-model:value="telegramConfig.bot_token" type="text"
                  placeholder="请输入Bot Token (例如: 123456789:ABCdefGHIjklMNOpqrsTUVwxyz)" size="small"
                  :disabled="isTesting" @blur="saveTelegramConfig"
                />
              </div>
            </div>
          </div>

          <!-- Chat ID设置 -->
          <div class="pt-4 border-t border-gray-200 dark:border-gray-700">
            <div class="flex items-start">
              <div class="w-1.5 h-1.5 bg-info rounded-full mr-3 mt-2 flex-shrink-0" />
              <div class="flex-1">
                <div class="text-sm font-medium mb-3 leading-relaxed">
                  Chat ID
                </div>
                <div class="text-xs opacity-60 mb-3">
                  目标聊天的ID，可以是个人聊天或群组聊天的ID
                </div>
                <n-input
                  v-model:value="telegramConfig.chat_id" type="text"
                  placeholder="请输入Chat ID (例如: 123456789 或 -123456789)" size="small" :disabled="isTesting"
                  @blur="saveTelegramConfig"
                />
              </div>
            </div>
          </div>

          <!-- 隐藏前端弹窗设置 -->
          <div class="pt-4 border-t border-gray-200 dark:border-gray-700">
            <div class="flex items-center justify-between">
              <div class="flex items-center">
                <div class="w-1.5 h-1.5 bg-info rounded-full mr-3 flex-shrink-0" />
                <div>
                  <div class="text-sm font-medium leading-relaxed">
                    隐藏前端弹窗
                  </div>
                  <div class="text-xs opacity-60">
                    启用后仅通过Telegram交互，不显示前端弹窗界面
                  </div>
                </div>
              </div>
              <n-switch
                v-model:value="telegramConfig.hide_frontend_popup" size="small"
                @update:value="saveTelegramConfig"
              />
            </div>
          </div>

          <!-- 保存并测试按钮 -->
          <div class="pt-4 border-t border-gray-200 dark:border-gray-700">
            <div class="flex items-start">
              <div class="w-1.5 h-1.5 bg-info rounded-full mr-3 mt-2 flex-shrink-0" />
              <div class="flex-1">
                <div class="text-sm font-medium mb-3 leading-relaxed">
                  连接测试
                </div>
                <div class="text-xs opacity-60 mb-3">
                  保存配置并发送测试消息验证连接
                </div>
                <n-button
                  type="primary" size="small" :loading="isTesting"
                  :disabled="!telegramConfig.bot_token.trim() || !telegramConfig.chat_id.trim()" @click="saveAndTest"
                >
                  {{ isTesting ? '测试中...' : '测试连接' }}
                </n-button>
              </div>
            </div>
          </div>
        </n-space>
      </n-collapse-transition>
    </n-space>
  </n-card>
</template>
