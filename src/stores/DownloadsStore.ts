import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useDownloadsStore = defineStore('downloads', () => {
  const downloadDir = ref(localStorage.getItem('downloadDir') ?? '')

  function setDownloadDir(path: string) {
    downloadDir.value = path
    localStorage.setItem('downloadDir', path)
  }

  return { downloadDir, setDownloadDir }
})
