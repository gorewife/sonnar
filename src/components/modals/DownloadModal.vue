<script setup lang="ts">
import { ref, watch } from "vue";
import { ref, watch } from 'vue'
// error is shown inline in the modal; cleared on next open
import { open } from "@tauri-apps/plugin-dialog";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import { useUIStore } from "@/stores/UIStore";
import { useDownloadsStore } from "@/stores/DownloadsStore";

const ui = useUIStore();
const downloads = useDownloadsStore();

const downloadLink = ref("");
const error = ref("");

// reset on open
watch(
    () => ui.isOpen("download"),
    (isOpen) => {
        if (isOpen) {
            downloadLink.value = "";
            error.value = "";
        }
    },
);
watch(() => ui.isOpen('download'), (isOpen) => {
  if (isOpen) {
    downloadLink.value = ''
    error.value = ''
  }
})

async function chooseDir() {
    const path = await open({ directory: true });
    if (typeof path === "string") {
        downloads.setDownloadDir(path);
    }
}

function cancel() {
    ui.closeModal();
}

async function enter() {
    if (!downloadLink.value) return;
    error.value = "";

    try {
        await downloads.addDownload(downloadLink.value);
        ui.closeModal();
    } catch (err) {
        console.error("[sonnar] addDownload failed:", err);
        error.value = "Couldn't start the download. Please try again.";
    }
}
</script>

<template>
    <Transition name="modal">
        <div
            v-if="ui.isOpen('download')"
            class="modal-backdrop"
            @click="cancel"
        >
            <div class="modal-dialog" @click.stop>
                <header class="modal-header">
                    <h3>Enter download link</h3>
                </header>

                <section class="modal-body">
                    <input
                        v-model="downloadLink"
                        placeholder="Download Link..."
                        @keyup.enter="enter"
                    />
                    <button @click="chooseDir" title="Select a directory">
                        <FontAwesomeIcon icon="folder" />
                    </button>
                </section>

                <p v-if="error" class="modal-error">{{ error }}</p>

                <footer class="modal-footer">
                    <button @click="cancel">Cancel</button>
                    <button :disabled="!downloadLink" @click="enter">
                        Enter
                    </button>
                </footer>
            </div>
        </div>
    </Transition>
</template>
