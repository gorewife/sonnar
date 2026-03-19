import { watch } from 'vue'
import { TrayIcon } from '@tauri-apps/api/tray'
import { defaultWindowIcon } from '@tauri-apps/api/app'
import { useSpeedTracker } from './useSpeedTracker'

function formatSpeed(bytesPerSec: number): string {
  if (bytesPerSec < 1024) return `${bytesPerSec} B/s`
  if (bytesPerSec < 1024 * 1024) return `${(bytesPerSec / 1024).toFixed(1)} KB/s`
  return `${(bytesPerSec / (1024 * 1024)).toFixed(1)} MB/s`
}

export function useTray() {
  // Called synchronously so onUnmounted in useSpeedTracker binds to this component
  const { downloadSpeed } = useSpeedTracker()

  async function init() {
    const icon = await defaultWindowIcon()
    const tray = await TrayIcon.new({
      id: 'main',
      icon: icon ?? undefined,
      tooltip: 'Sonnar',
    })

    watch(downloadSpeed, (speed) => {
      // show speed when active, clear title when idle so only the icon shows
      tray.setTitle(speed > 0 ? `↓ ${formatSpeed(speed)}` : '')
    })
  }

  init().catch(console.error)
}
