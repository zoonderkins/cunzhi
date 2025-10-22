import {
  create,
  NAlert,
  NButton,
  NCard,
  NCheckbox,
  NCollapse,
  NCollapseItem,
  NCollapseTransition,
  NConfigProvider,
  NDialogProvider,
  NForm,
  NFormItem,
  NGrid,
  NGridItem,
  NIcon,
  NImage,
  NImageGroup,
  NInput,
  NInputGroup,
  NInputNumber,
  NMessageProvider,
  NModal,
  NNotificationProvider,
  NProgress,
  NRadio,
  NRadioGroup,
  NSkeleton,
  NSpace,
  NSpin,
  NStep,
  NSteps,
  NSwitch,
  NTab,
  NTabPane,
  NTabs,
  NTag,
  NTooltip,
} from 'naive-ui'
import { createApp } from 'vue'
import App from './App.vue'
import 'virtual:uno.css'
import './assets/styles/style.css'

const naive = create({
  components: [
    NAlert,
    NButton,
    NCard,
    NCheckbox,
    NCollapse,
    NCollapseItem,
    NCollapseTransition,
    NConfigProvider,
    NDialogProvider,
    NForm,
    NFormItem,
    NMessageProvider,
    NModal,
    NNotificationProvider,
    NSpace,
    NSpin,
    NStep,
    NSteps,
    NSwitch,
    NTab,
    NTabPane,
    NTabs,
    NInput,
    NInputGroup,
    NInputNumber,
    NTooltip,
    NIcon,
    NImage,
    NImageGroup,
    NGrid,
    NGridItem,
    NTag,
    NSkeleton,
    NProgress,
    NRadio,
    NRadioGroup,
  ],
})

console.log('[main.ts] 開始創建 Vue 應用...')

const app = createApp(App)
console.log('[main.ts] Vue 應用已創建')

app.use(naive)
console.log('[main.ts] Naive UI 已註冊')

// 新增全局錯誤處理
app.config.errorHandler = (err, instance, info) => {
  console.error('[Vue Error]', err)
  console.error('[Vue Error Info]', info)
  console.error('[Vue Instance]', instance)
}

app.mount('#app')
console.log('[main.ts] Vue 應用已掛載到 #app')
