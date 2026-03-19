import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke, Channel } from "@tauri-apps/api/core";
import { downloadDir as getOsDownloadDir } from "@tauri-apps/api/path";
import type {
  DownloadInfo,
  DownloadEvent,
  ActiveDownload,
} from "@/types/downloads";

export const useDownloadsStore = defineStore("downloads", () => {
  const downloadDir = ref(localStorage.getItem("downloadDir") ?? "");

  async function init() {
    if (!downloadDir.value) {
      downloadDir.value = await getOsDownloadDir();
    }
  }
  const downloads = ref<ActiveDownload[]>([]);

  function setDownloadDir(path: string) {
    downloadDir.value = path;
    localStorage.setItem("downloadDir", path);
  }

  async function getDownloadInfo(link: string): Promise<DownloadInfo> {
    return invoke<DownloadInfo>("get_download_info", { link });
  }

  async function startDownload(id: number, path: string) {
    const channel = new Channel<DownloadEvent>();

    channel.onmessage = (event) => {
      const download = downloads.value.find((d) => d.info.id === id);
      if (!download) return;

      if (event.status === "started") {
        download.status = "downloading";
      } else if (event.status === "progress") {
        download.currentByte = event.content.currentByte;
      } else if (event.status === "finished") {
        download.status = "finished";
      }
    };

    await invoke("start_download", { id, pathString: path, chan: channel });
  }

  async function addDownload(link: string) {
    if (!downloadDir.value) throw new Error("No download directory set");

    const info = await getDownloadInfo(link);

    downloads.value.push({
      info,
      status: "pending",
      currentByte: 0,
      error: null,
    });

    const path = `${downloadDir.value}/${info.default_name}`;

    startDownload(info.id, path).catch((err: string) => {
      const download = downloads.value.find((d) => d.info.id === info.id);
      if (download) {
        download.status = "error";
        download.error = err;
      }
    });

    return info;
  }

  async function cancelDownload(id: number) {
    await invoke<boolean>("early_download_cancel", { id });
    const index = downloads.value.findIndex((d) => d.info.id === id);
    if (index !== -1) downloads.value.splice(index, 1);
  }

  return {
    downloadDir,
    downloads,
    init,
    setDownloadDir,
    getDownloadInfo,
    addDownload,
    cancelDownload,
  };
});
