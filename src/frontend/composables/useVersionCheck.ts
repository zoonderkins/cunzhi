import { invoke } from '@tauri-apps/api/core'
import { ref } from 'vue'

interface VersionInfo {
  current: string
  latest: string
  hasUpdate: boolean
  releaseUrl: string
  releaseNotes: string
}

// 全局版本检查状态
const versionInfo = ref<VersionInfo | null>(null)
const isChecking = ref(false)
const lastCheckTime = ref<Date | null>(null)

// 比较版本号
function compareVersions(version1: string, version2: string): number {
  const v1Parts = version1.split('.').map(Number)
  const v2Parts = version2.split('.').map(Number)

  for (let i = 0; i < Math.max(v1Parts.length, v2Parts.length); i++) {
    const v1Part = v1Parts[i] || 0
    const v2Part = v2Parts[i] || 0

    if (v1Part > v2Part)
      return 1
    if (v1Part < v2Part)
      return -1
  }

  return 0
}

// 获取当前版本
async function getCurrentVersion(): Promise<string> {
  try {
    const appInfo = await invoke('get_app_info') as string
    const match = appInfo.match(/v(\d+\.\d+\.\d+)/)
    return match ? match[1] : '0.1.3'
  }
  catch (error) {
    console.error('获取当前版本失败:', error)
    return '0.1.3'
  }
}

// 检查GitHub最新版本
async function checkLatestVersion(): Promise<VersionInfo | null> {
  if (isChecking.value) {
    return versionInfo.value
  }

  try {
    isChecking.value = true

    const response = await fetch('https://api.github.com/repos/imhuso/cunzhi/releases/latest', {
      headers: {
        Accept: 'application/vnd.github.v3+json',
      },
    })

    if (!response.ok) {
      throw new Error(`GitHub API请求失败: ${response.status}`)
    }

    const release = await response.json()
    const latestVersion = release.tag_name.replace(/^v/, '')
    const currentVersion = await getCurrentVersion()

    const hasUpdate = compareVersions(latestVersion, currentVersion) > 0

    const info: VersionInfo = {
      current: currentVersion,
      latest: latestVersion,
      hasUpdate,
      releaseUrl: release.html_url,
      releaseNotes: release.body || '暂无更新说明',
    }

    versionInfo.value = info
    lastCheckTime.value = new Date()

    return info
  }
  catch (error) {
    console.error('检查版本更新失败:', error)
    return null
  }
  finally {
    isChecking.value = false
  }
}

// 静默检查更新（应用启动时调用）
async function silentCheckUpdate(): Promise<boolean> {
  // 如果最近1小时内已经检查过，跳过
  if (lastCheckTime.value && Date.now() - lastCheckTime.value.getTime() < 60 * 60 * 1000) {
    return versionInfo.value?.hasUpdate || false
  }

  const info = await checkLatestVersion()
  return info?.hasUpdate || false
}

// 获取版本信息（如果没有则初始化）
async function getVersionInfo(): Promise<VersionInfo | null> {
  if (!versionInfo.value) {
    const currentVersion = await getCurrentVersion()
    versionInfo.value = {
      current: currentVersion,
      latest: currentVersion,
      hasUpdate: false,
      releaseUrl: '',
      releaseNotes: '',
    }
  }
  return versionInfo.value
}

// 简化的安全打开链接函数
async function safeOpenUrl(url: string): Promise<void> {
  try {
    // 使用已导入的invoke函数
    await invoke('open_external_url', { url })
  }
  catch {
    // 如果Tauri方式失败，复制到剪贴板
    try {
      await navigator.clipboard.writeText(url)
      throw new Error(`无法自动打开链接，已复制到剪贴板，请手动打开: ${url}`)
    }
    catch {
      throw new Error(`无法打开链接，请手动访问: ${url}`)
    }
  }
}

// 打开下载页面
async function openDownloadPage(): Promise<void> {
  await safeOpenUrl('https://github.com/imhuso/cunzhi/releases/latest')
}

// 打开发布页面
async function openReleasePage(): Promise<void> {
  if (versionInfo.value?.releaseUrl) {
    await safeOpenUrl(versionInfo.value.releaseUrl)
  }
}

export function useVersionCheck() {
  return {
    versionInfo,
    isChecking,
    lastCheckTime,
    checkLatestVersion,
    silentCheckUpdate,
    getVersionInfo,
    openDownloadPage,
    openReleasePage,
    compareVersions,
    safeOpenUrl,
  }
}
