import { ref, onUnmounted } from 'vue'
import { useDownloadsStore } from '@/stores/DownloadsStore'

export function useSpeedTracker() {
  const downloads = useDownloadsStore()
  const downloadSpeed = ref(0) // bytes per second

  const prevBytes = new Map<number, number>()

  const intervalId = window.setInterval(() => {
    let delta = 0

    for (const dl of downloads.downloads) {
      if (dl.status === 'downloading') {
        const prev = prevBytes.get(dl.info.id) ?? dl.currentByte
        delta += Math.max(0, dl.currentByte - prev)
      }
      prevBytes.set(dl.info.id, dl.currentByte)
    }

    // drop entries for downloads that are gone
    const activeIds = new Set(downloads.downloads.map(d => d.info.id))
    for (const id of prevBytes.keys()) {
      if (!activeIds.has(id)) prevBytes.delete(id)
    }

    downloadSpeed.value = delta
  }, 1000)

  onUnmounted(() => clearInterval(intervalId))

  return { downloadSpeed }
}
