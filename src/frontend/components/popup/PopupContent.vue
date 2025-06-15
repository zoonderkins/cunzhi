<script setup lang="ts">
import type { McpRequest } from '../../types/popup'
import hljs from 'highlight.js'
import MarkdownIt from 'markdown-it'
import { useMessage } from 'naive-ui'
import { nextTick, onMounted, watch } from 'vue'
import 'highlight.js/styles/github-dark.css'

interface Props {
  request: McpRequest | null
  loading?: boolean
  currentTheme?: string
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
  currentTheme: 'dark',
})

const message = useMessage()

// 创建 Markdown 实例 - 保持代码高亮功能
const md = new MarkdownIt({
  html: true,
  xhtmlOut: false,
  breaks: true,
  langPrefix: 'language-',
  linkify: true,
  typographer: true,
  quotes: '""\'\'',
  highlight(str: string, lang: string) {
    if (lang && hljs.getLanguage(lang)) {
      try {
        return hljs.highlight(str, { language: lang }).value
      }
      catch {
        // 忽略错误
      }
    }
    return ''
  },
})

// Markdown 渲染函数
function renderMarkdown(content: string) {
  try {
    return md.render(content)
  }
  catch (error) {
    console.error('Markdown 渲染失败:', error)
    return content
  }
}

// 创建复制按钮
function createCopyButton(preEl: Element) {
  // 检查是否已经有复制按钮
  if (preEl.querySelector('.copy-button'))
    return

  const copyButton = document.createElement('div')
  copyButton.className = 'copy-button'
  copyButton.style.cssText = 'position: absolute; top: 12px; right: 12px; z-index: 999; opacity: 0; transition: opacity 0.2s; pointer-events: auto; height: 24px; width: 24px; display: flex; align-items: center; justify-content: center;'
  copyButton.innerHTML = `
    <button style="display: flex; align-items: center; justify-content: center; width: 100%; height: 100%; color: #d1d5db; transition: color 0.2s; border: none; background: none; cursor: pointer; padding: 0; margin: 0;" onmouseover="this.style.color='#9ca3af'" onmouseout="this.style.color='#d1d5db'">
      <div class="i-carbon-copy" style="width: 16px; height: 16px; display: block;"></div>
    </button>
  `

  const button = copyButton.querySelector('button')!
  button.addEventListener('click', async (e) => {
    e.stopPropagation()
    e.preventDefault()
    try {
      const codeEl = preEl.querySelector('code')
      const textContent = codeEl?.textContent || preEl.textContent || ''
      await navigator.clipboard.writeText(textContent)

      // 更新为成功状态
      const icon = button.querySelector('div')!
      icon.className = 'i-carbon-checkmark'
      icon.style.cssText = 'width: 16px; height: 16px; color: #22c55e;'

      setTimeout(() => {
        icon.className = 'i-carbon-copy'
        icon.style.cssText = 'width: 16px; height: 16px;'
      }, 2000)
      message.success('代码已复制到剪贴板')
    }
    catch {
      message.error('复制失败')
    }
  })

  // 确保父元素有相对定位和足够的层级
  const preElement = preEl as HTMLElement
  preElement.style.position = 'relative'
  preElement.style.zIndex = '1'

  // 添加鼠标悬停事件来显示/隐藏按钮
  preElement.addEventListener('mouseenter', () => {
    copyButton.style.opacity = '1'
  })
  preElement.addEventListener('mouseleave', () => {
    copyButton.style.opacity = '0'
  })

  preElement.appendChild(copyButton)
}

// 设置内联代码复制
function setupInlineCodeCopy() {
  const inlineCodeElements = document.querySelectorAll('.markdown-content p code, .markdown-content li code')
  inlineCodeElements.forEach((codeEl) => {
    codeEl.addEventListener('click', async () => {
      try {
        await navigator.clipboard.writeText(codeEl.textContent || '')
        message.success('代码已复制到剪贴板')
      }
      catch {
        message.error('复制失败')
      }
    })
  })
}

// 设置代码复制功能
let setupCodeCopyTimer: number | null = null
function setupCodeCopy() {
  if (setupCodeCopyTimer) {
    clearTimeout(setupCodeCopyTimer)
  }

  setupCodeCopyTimer = window.setTimeout(() => {
    nextTick(() => {
      // 确保选择正确的 pre 元素
      const preElements = document.querySelectorAll('.markdown-content pre')
      preElements.forEach((preEl) => {
        createCopyButton(preEl)
      })
      setupInlineCodeCopy()
    })
  }, 100)
}

// 监听request变化，重新设置代码复制
watch(() => props.request, () => {
  if (props.request) {
    setupCodeCopy()
  }
}, { deep: true })

onMounted(() => {
  if (props.request) {
    setupCodeCopy()
  }
})
</script>

<template>
  <div class="text-theme-text">
    <!-- 加载状态 -->
    <div v-if="loading" class="flex flex-col items-center justify-center py-8">
      <n-spin size="medium" />
      <p class="text-sm mt-3 text-theme-text opacity-60">
        加载中...
      </p>
    </div>

    <!-- 消息显示区域 -->
    <div v-else-if="request?.message">
      <!-- 主要内容 -->
      <div
        v-if="request.is_markdown" class="markdown-content prose prose-sm dark:prose-invert max-w-none
               prose-headings:font-semibold prose-headings:text-theme-text prose-headings:leading-tight
               prose-h1:!mt-4 prose-h1:!mb-2 prose-h1:!text-lg prose-h1:!font-bold prose-h1:!leading-tight
               prose-h2:!mt-3 prose-h2:!mb-1.5 prose-h2:!text-base prose-h2:!font-semibold prose-h2:!leading-tight
               prose-h3:!mt-2.5 prose-h3:!mb-1 prose-h3:!text-sm prose-h3:!font-medium prose-h3:!leading-tight
               prose-h4:!mt-2 prose-h4:!mb-1 prose-h4:!text-sm prose-h4:!font-medium prose-h4:!leading-tight
               prose-p:my-1 prose-p:text-theme-text prose-p:opacity-85 prose-p:leading-relaxed prose-p:text-sm
               prose-ul:my-1 prose-ul:text-theme-text prose-ul:opacity-85 prose-ul:text-sm prose-ul:pl-4
               prose-ol:my-1 prose-ol:text-theme-text prose-ol:opacity-85 prose-ol:text-sm prose-ol:pl-4
               prose-li:my-1 prose-li:text-theme-text prose-li:opacity-85 prose-li:text-sm prose-li:leading-relaxed
               prose-blockquote:my-2 prose-blockquote:text-theme-text-muted prose-blockquote:opacity-90 prose-blockquote:text-sm
               prose-blockquote:border-l-4 prose-blockquote:border-primary-500
               prose-blockquote:pl-4 prose-blockquote:ml-0 prose-blockquote:italic
               prose-pre:relative prose-pre:bg-gray-50/50 dark:prose-pre:bg-gray-800/30
               prose-pre:border prose-pre:border-gray-200/50 dark:prose-pre:border-gray-700/30 prose-pre:rounded-lg prose-pre:p-4 prose-pre:my-3 prose-pre:overflow-x-auto
               prose-code:px-1 prose-code:py-0.5 prose-code:text-xs prose-code:cursor-pointer prose-code:font-mono
               prose-a:text-primary-500 prose-a:no-underline hover:prose-a:underline hover:prose-a:text-primary-400
               prose-strong:text-theme-text prose-strong:font-semibold
               prose-em:text-theme-text-secondary prose-em:italic" v-html="renderMarkdown(request.message)"
      />
      <div v-else class="whitespace-pre-wrap leading-relaxed text-theme-text opacity-80">
        {{ request.message }}
      </div>
    </div>

    <!-- 错误状态 -->
    <n-alert v-else type="error" title="数据加载错误">
      <div class="text-theme-text opacity-80">
        Request对象: {{ JSON.stringify(request) }}
      </div>
    </n-alert>
  </div>
</template>
