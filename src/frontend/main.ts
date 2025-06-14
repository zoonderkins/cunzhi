import {
  create,
  darkTheme,
  lightTheme,
  NButton,
  NCard,
  NCheckbox,
  NConfigProvider,
  NDialogProvider,
  NGrid,
  NGridItem,
  NIcon,
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
  NTooltip,
} from 'naive-ui'
import { createApp } from 'vue'
import App from './App.vue'
import 'virtual:uno.css'
import '../assets/css/style.css'

const naive = create({
  components: [
    NButton,
    NCard,
    NCheckbox,
    NConfigProvider,
    NDialogProvider,
    NMessageProvider,
    NNotificationProvider,
    NSpace,
    NSwitch,
    NTab,
    NTabPane,
    NTabs,
    NInput,
    NInputGroup,
    NInputNumber,
    NTooltip,
    NIcon,
    NGrid,
    NGridItem,
    NTag,
  ],
})

const app = createApp(App)
app.use(naive)
app.mount('#app')
