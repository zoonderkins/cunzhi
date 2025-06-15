import { computed } from 'vue'

/**
 * 跨平台键盘快捷键处理
 */
export function useKeyboard() {
  // 检测操作系统
  const isMac = computed(() => {
    if (typeof navigator !== 'undefined') {
      return navigator.platform.toUpperCase().indexOf('MAC') >= 0
    }
    return false
  })

  // 获取修饰键名称
  const modifierKey = computed(() => {
    return isMac.value ? 'Cmd' : 'Ctrl'
  })

  // 获取粘贴快捷键描述
  const pasteShortcut = computed(() => {
    return isMac.value ? '⌘+V' : 'Ctrl+V'
  })

  // 检查是否按下了粘贴快捷键
  function isPasteShortcut(event: KeyboardEvent): boolean {
    const isModifierPressed = isMac.value ? event.metaKey : event.ctrlKey
    return isModifierPressed && event.key.toLowerCase() === 'v'
  }

  // 获取常用快捷键描述
  function getShortcutText(action: 'paste' | 'copy' | 'cut' | 'save' | 'undo' | 'redo'): string {
    const prefix = isMac.value ? '⌘' : 'Ctrl'
    
    switch (action) {
      case 'paste':
        return `${prefix}+V`
      case 'copy':
        return `${prefix}+C`
      case 'cut':
        return `${prefix}+X`
      case 'save':
        return `${prefix}+S`
      case 'undo':
        return `${prefix}+Z`
      case 'redo':
        return isMac.value ? '⌘+Shift+Z' : 'Ctrl+Y'
      default:
        return ''
    }
  }

  return {
    isMac,
    modifierKey,
    pasteShortcut,
    isPasteShortcut,
    getShortcutText,
  }
}
