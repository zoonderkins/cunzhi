import { invoke } from '@tauri-apps/api/core'

/**
 * 音訊管理組合式函數
 */
export function useAudioManager() {
  /**
   * 播放音訊檔案
   */
  async function playAudio(audioName: string): Promise<void> {
    try {
      await invoke('play_audio', { audioName })
    }
    catch (error) {
      console.error('播放音訊失敗:', error)
      throw error
    }
  }

  /**
   * 停止音訊播放
   */
  async function stopAudio(): Promise<void> {
    try {
      await invoke('stop_audio')
    }
    catch (error) {
      console.error('停止音訊失敗:', error)
      throw error
    }
  }

  /**
   * 獲取可用的音訊檔案列表
   */
  async function getAvailableAudioFiles(): Promise<string[]> {
    try {
      return await invoke('get_available_audio_files')
    }
    catch (error) {
      console.error('獲取音訊檔案列表失敗:', error)
      return []
    }
  }

  /**
   * 測試音訊播放
   */
  async function testAudio(audioName: string): Promise<void> {
    try {
      await playAudio(audioName)
      // 可以新增一些測試逻辑，比如播放一小段時间后停止
      setTimeout(async () => {
        try {
          await stopAudio()
        }
        catch (error) {
          console.warn('停止測試音訊失敗:', error)
        }
      }, 2000) // 2秒后停止
    }
    catch (error) {
      console.error('測試音訊失敗:', error)
      throw error
    }
  }

  /**
   * 處理音訊測試錯誤
   */
  function handleTestAudioError(error: any) {
    console.error('音訊測試錯誤:', error)
    // 这里可以顯示錯誤提示给用户
  }

  return {
    playAudio,
    stopAudio,
    getAvailableAudioFiles,
    testAudio,
    handleTestAudioError,
  }
}
