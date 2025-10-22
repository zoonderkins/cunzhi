// 匯入 Naive UI 元件 (复用主應用的設定)
import {
  create,
  NButton,
  NCard,
  NCheckbox,
  NConfigProvider,
  NDialogProvider,
  NGrid,
  NGridItem,
  NImage,
  NImageGroup,
  NInput,
  NInputGroup,
  NInputNumber,
  NMessageProvider,
  NNotificationProvider,
  NSpace,
  NSwitch,
  NTab,
  NTabPane,
  NTabs,
  NTag,
} from 'naive-ui'
import { createApp } from 'vue'

// 匯入主題
import { useTheme } from '../composables/useTheme'

import TestApp from './TestApp.vue'
// 匯入样式
import 'virtual:uno.css'

import '../assets/styles/style.css'

// 建立 Naive UI 实例
const naive = create({
  components: [
    NButton,
    NCard,
    NCheckbox,
    NConfigProvider,
    NGrid,
    NGridItem,
    NImage,
    NImageGroup,
    NInput,
    NInputGroup,
    NInputNumber,
    NSpace,
    NSwitch,
    NTab,
    NTabPane,
    NTabs,
    NTag,
    NMessageProvider,
    NNotificationProvider,
    NDialogProvider,
  ],
})

// 建立 Vue 應用
const app = createApp(TestApp)

// 使用 Naive UI
app.use(naive)

// 挂载應用
app.mount('#app')

// 模拟 Tauri API 用于測試环境
if (!window.__TAURI__) {
  window.__TAURI__ = {
    core: {
      invoke: async (command: string) => {
        console.log(`模拟 Tauri 呼叫: ${command}`)
        // 模拟主題設定傳回
        if (command === 'get_theme_config') {
          return { theme: 'dark' }
        }
        if (command === 'get_reply_config') {
          return { continue_prompt: '请按照最佳实践繼續' }
        }
        return {}
      },
    },
  }
}

// 初始化主題
const { loadTheme } = useTheme()
loadTheme().catch(() => {
  console.log('使用預設主題')
})
