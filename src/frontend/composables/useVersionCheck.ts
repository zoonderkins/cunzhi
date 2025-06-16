import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { ref } from 'vue'

interface VersionInfo {
  current: string
  latest: string
  hasUpdate: boolean
  releaseUrl: string
  releaseNotes: string
}

interface UpdateInfo {
  available: boolean
  current_version: string
  latest_version: string
  release_notes: string
  download_url: string
}

interface UpdateProgress {
  chunk_length: number
  content_length?: number
  downloaded: number
  percentage: number
}

// 全局版本检查状态
const versionInfo = ref<VersionInfo | null>(null)
const isChecking = ref(false)
const lastCheckTime = ref<Date | null>(null)

// 更新相关状态
const isUpdating = ref(false)
const updateProgress = ref<UpdateProgress | null>(null)
const updateStatus = ref<'idle' | 'checking' | 'downloading' | 'installing' | 'completed' | 'error'>('idle')

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

// 使用Tauri Updater检查更新
async function checkForUpdatesWithTauri(): Promise<UpdateInfo | null> {
  try {
    const updateInfo = await invoke('check_for_updates') as UpdateInfo
    return updateInfo
  }
  catch (error) {
    console.error('Tauri更新检查失败:', error)
    return null
  }
}

// 一键更新功能
async function performOneClickUpdate(): Promise<void> {
  if (isUpdating.value) {
    return
  }

  try {
    isUpdating.value = true
    updateStatus.value = 'checking'
    updateProgress.value = null

    // 首先检查是否有更新
    const updateInfo = await checkForUpdatesWithTauri()
    if (!updateInfo?.available) {
      throw new Error('没有可用的更新')
    }

    // 设置事件监听器
    const unlistenProgress = await listen('update_download_progress', (event) => {
      updateProgress.value = event.payload as UpdateProgress
      updateStatus.value = 'downloading'
    })

    const unlistenInstallStart = await listen('update_install_started', () => {
      updateStatus.value = 'installing'
    })

    const unlistenInstallFinish = await listen('update_install_finished', () => {
      updateStatus.value = 'completed'
    })

    try {
      // 开始下载和安装
      updateStatus.value = 'downloading'
      await invoke('download_and_install_update')

      // 更新完成
      updateStatus.value = 'completed'
    }
    finally {
      // 清理事件监听器
      unlistenProgress()
      unlistenInstallStart()
      unlistenInstallFinish()
    }
  }
  catch (error) {
    updateStatus.value = 'error'
    throw error
  }
  finally {
    isUpdating.value = false
  }
}

// 重启应用
async function restartApp(): Promise<void> {
  try {
    await invoke('restart_app')
  }
  catch (error) {
    console.error('重启应用失败:', error)
    throw error
  }
}

export function useVersionCheck() {
  return {
    versionInfo,
    isChecking,
    lastCheckTime,
    isUpdating,
    updateProgress,
    updateStatus,
    checkLatestVersion,
    silentCheckUpdate,
    getVersionInfo,
    openDownloadPage,
    openReleasePage,
    checkForUpdatesWithTauri,
    performOneClickUpdate,
    restartApp,
    compareVersions,
    safeOpenUrl,
  }
}
