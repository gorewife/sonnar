// Mirrors src-tauri/src/downloads/mod.rs

export interface DownloadInfo {
  id: number
  url: string
  default_name: string
  total_bytes: number | null
}

// DownloadEvent uses adjacent tagging: { status: "...", content: { ... } }
export type DownloadEvent =
  | { status: 'started';  content: { id: number; url: string; totalBytes: number | null } }
  | { status: 'progress'; content: { id: number; currentByte: number } }
  | { status: 'finished'; content: { id: number } }

// DownloadError is serialized as a plain string by the backend
export type DownloadError = string

export type DownloadStatus = 'pending' | 'downloading' | 'finished' | 'error'

export interface ActiveDownload {
  info: DownloadInfo
  status: DownloadStatus
  currentByte: number
  error: DownloadError | null
}
