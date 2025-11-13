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

// 持久化存储的键名
const CANCELLED_VERSIONS_KEY = 'cunzhi_cancelled_versions'

// 載入已取消的版本
function loadCancelledVersions(): Set<string> {
  try {
    const stored = localStorage.getItem(CANCELLED_VERSIONS_KEY)
    if (stored) {
      const versions = JSON.parse(stored) as string[]
      return new Set(versions)
    }
  }
  catch (error) {
    console.warn('載入已取消版本失敗:', error)
  }
  return new Set()
}

// 儲存已取消的版本
function saveCancelledVersions(versions: Set<string>) {
  try {
    const versionsArray = Array.from(versions)
    localStorage.setItem(CANCELLED_VERSIONS_KEY, JSON.stringify(versionsArray))
  }
  catch (error) {
    console.warn('儲存已取消版本失敗:', error)
  }
}

// 全局版本檢查狀態
const versionInfo = ref<VersionInfo | null>(null)
const isChecking = ref(false)
const lastCheckTime = ref<Date | null>(null)

// 更新相关狀態
const isUpdating = ref(false)
const updateProgress = ref<UpdateProgress | null>(null)
const updateStatus = ref<'idle' | 'checking' | 'downloading' | 'installing' | 'completed' | 'error'>('idle')

// 自動更新弹窗狀態
const showUpdateModal = ref(false)
const autoCheckEnabled = ref(true)
// 記錄用户取消的版本，避免重复弹窗（持久化存储）
const cancelledVersions = ref<Set<string>>(loadCancelledVersions())

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

// 獲取當前版本
async function getCurrentVersion(): Promise<string> {
  try {
    const appInfo = await invoke('get_app_info') as string
    const match = appInfo.match(/v(\d+\.\d+\.\d+)/)
    const version = match ? match[1] : '0.2.0'
    return version
  }
  catch (error) {
    console.error('獲取當前版本失敗:', error)
    return '0.2.0'
  }
}

// 檢查GitHub最新版本
async function checkLatestVersion(): Promise<VersionInfo | null> {
  if (isChecking.value) {
    return versionInfo.value
  }

  try {
    isChecking.value = true

    const response = await fetch('https://api.github.com/repos/zoonderkins/cunzhi/releases/latest', {
      headers: {
        Accept: 'application/vnd.github.v3+json',
      },
    })

    if (!response.ok) {
      throw new Error(`GitHub API請求失敗: ${response.status}`)
    }

    const release = await response.json()
    // 提取版本号，處理中文tag的情况
    let latestVersion = release.tag_name
    // 移除前缀 v 和中文字符，只保留数字和点
    latestVersion = latestVersion.replace(/^v/, '').replace(/[^\d.]/g, '')
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
    console.error('檢查版本更新失敗:', error)
    return null
  }
  finally {
    isChecking.value = false
  }
}

// 自動檢查更新并弹窗（應用啟動時呼叫）
async function autoCheckUpdate(): Promise<boolean> {
  // 如果禁用自動檢查，跳過
  if (!autoCheckEnabled.value) {
    return false
  }

  // 如果最近1小時内已经檢查过，跳過
  if (lastCheckTime.value && Date.now() - lastCheckTime.value.getTime() < 60 * 60 * 1000) {
    const hasUpdate = versionInfo.value?.hasUpdate || false
    // 如果有更新且未顯示弹窗，且用户未取消该版本，则顯示弹窗
    if (hasUpdate && !showUpdateModal.value && versionInfo.value?.latest && !cancelledVersions.value.has(versionInfo.value.latest)) {
      showUpdateModal.value = true
    }
    return hasUpdate
  }

  try {
    const info = await checkLatestVersion()

    // 如果检測到新版本且用户未取消该版本，自動顯示更新弹窗
    if (info?.hasUpdate && !cancelledVersions.value.has(info.latest)) {
      showUpdateModal.value = true
      return true
    }

    return false
  }
  catch (error) {
    console.warn('自動檢查更新失敗:', error)
    return false
  }
}

// 静默檢查更新（不弹窗，保持相容性）
async function silentCheckUpdate(): Promise<boolean> {
  const originalAutoCheck = autoCheckEnabled.value
  autoCheckEnabled.value = false

  try {
    const info = await checkLatestVersion()
    return info?.hasUpdate || false
  }
  finally {
    autoCheckEnabled.value = originalAutoCheck
  }
}

// 獲取版本訊息（如果没有则初始化）
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

// 简化的安全開啟連結函數
async function safeOpenUrl(url: string): Promise<void> {
  try {
    // 使用已匯入的invoke函數
    await invoke('open_external_url', { url })
  }
  catch {
    // 如果Tauri方式失敗，複製到剪贴板
    try {
      await navigator.clipboard.writeText(url)
      throw new Error(`無法自動開啟連結，已複製到剪贴板，請手動開啟: ${url}`)
    }
    catch {
      throw new Error(`無法開啟連結，請手動访问: ${url}`)
    }
  }
}

// 開啟下载页面
async function openDownloadPage(): Promise<void> {
  await safeOpenUrl('https://github.com/zoonderkins/cunzhi/releases/latest')
}

// 開啟發布页面
async function openReleasePage(): Promise<void> {
  if (versionInfo.value?.releaseUrl) {
    await safeOpenUrl(versionInfo.value.releaseUrl)
  }
}

// 使用改进的更新檢查（避免Tauri原生updater的中文tag问题）
async function checkForUpdatesWithTauri(): Promise<UpdateInfo | null> {
  try {
    const updateInfo = await invoke('check_for_updates') as UpdateInfo
    console.log('✅ Tauri 更新檢查成功:', updateInfo)
    return updateInfo
  }
  catch (error) {
    console.error('❌ Tauri更新檢查失敗，使用 GitHub API fallback:', error)

    // 如果Tauri檢查失敗，fallback到前端GitHub API檢查
    const githubInfo = await checkLatestVersion()

    if (githubInfo?.hasUpdate) {
      return {
        available: true,
        current_version: githubInfo.current,
        latest_version: githubInfo.latest,
        release_notes: githubInfo.releaseNotes,
        download_url: githubInfo.releaseUrl,
      }
    }

    return null
  }
}

// 一键更新功能
async function performOneClickUpdate(): Promise<void> {
  if (isUpdating.value) {
    console.log('⚠️ 更新已在進行中，跳過')
    return
  }

  try {
    isUpdating.value = true
    updateStatus.value = 'checking'
    updateProgress.value = null

    // 首先檢查是否有更新
    const updateInfo = await checkForUpdatesWithTauri()

    if (!updateInfo?.available) {
      throw new Error('没有可用的更新')
    }

    // 設定事件監聽器
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

    const unlistenManualDownload = await listen('update_manual_download_required', (event) => {
      console.log('🔗 需要手動下载，URL:', event.payload)
    })

    try {
      // 開始下载和安装
      updateStatus.value = 'downloading'
      await invoke('download_and_install_update')
      updateStatus.value = 'completed'
    }
    catch (backendError) {
      console.error('🔴 后端更新呼叫失敗:', backendError)
      throw backendError
    }
    finally {
      // 清理事件監聽器
      unlistenProgress()
      unlistenInstallStart()
      unlistenInstallFinish()
      unlistenManualDownload()
    }
  }
  catch (error) {
    console.error('🔥 更新失敗:', error)
    updateStatus.value = 'error'
    throw error
  }
  finally {
    isUpdating.value = false
  }
}

// 重新啟動應用
async function restartApp(): Promise<void> {
  try {
    await invoke('restart_app')
  }
  catch (error) {
    console.error('重新啟動應用失敗:', error)
    throw error
  }
}

// 關閉更新弹窗
function closeUpdateModal() {
  showUpdateModal.value = false
}

// 關閉更新弹窗（不再自動弹出该版本的更新提醒）
function dismissUpdate() {
  if (versionInfo.value?.latest) {
    cancelledVersions.value.add(versionInfo.value.latest)
    saveCancelledVersions(cancelledVersions.value)
    console.log(`🚫 用户關閉了版本 ${versionInfo.value.latest} 的更新弹窗`)
  }
  showUpdateModal.value = false
}

// 手動檢查更新（重置取消狀態）
async function manualCheckUpdate(): Promise<VersionInfo | null> {
  // 清空取消的版本記錄，因为这是用户主动檢查
  cancelledVersions.value.clear()
  saveCancelledVersions(cancelledVersions.value)
  console.log('🔄 手動檢查更新，清空取消記錄')

  const info = await checkLatestVersion()

  // 如果有更新，顯示弹窗
  if (info?.hasUpdate) {
    showUpdateModal.value = true
  }

  return info
}

export function useVersionCheck() {
  return {
    versionInfo,
    isChecking,
    lastCheckTime,
    isUpdating,
    updateProgress,
    updateStatus,
    showUpdateModal,
    autoCheckEnabled,
    checkLatestVersion,
    autoCheckUpdate,
    silentCheckUpdate,
    getVersionInfo,
    openDownloadPage,
    openReleasePage,
    checkForUpdatesWithTauri,
    performOneClickUpdate,
    restartApp,
    closeUpdateModal,
    dismissUpdate,
    manualCheckUpdate,
    compareVersions,
    safeOpenUrl,
  }
}
