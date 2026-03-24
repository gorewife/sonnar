<script setup lang="ts">
import { ref, watch } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { useUIStore } from "@/stores/UIStore";
import { useDownloadsStore } from "@/stores/DownloadsStore";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";

const ui = useUIStore();
const downloads = useDownloadsStore();

const downloadLink = ref("");
const error = ref("");
const errorDetails = ref("");

watch(
    () => ui.isOpen("download"),
    (isOpen) => {
        if (isOpen) {
            downloadLink.value = "";
            error.value = "";
            errorDetails.value = "";
        }
    },
);

async function chooseDir() {
    const path = await open({ directory: true });
    if (typeof path === "string") {
        downloads.setDownloadDir(path);
    }
}

function cancel() {
    ui.closeModal();
}

function copyDetails() {
    navigator.clipboard.writeText(errorDetails.value);
}

async function enter() {
    if (!downloadLink.value) return;
    error.value = "";
    errorDetails.value = "";

    try {
        await downloads.addDownload(downloadLink.value);
        ui.closeModal();
    } catch (err) {
        console.error("[ophelia] addDownload failed:", err);
        error.value = "Couldn't start the download. Please try again.";
        errorDetails.value = err instanceof Error ? err.message : String(err);
    }
}
</script>

<template>
    <Transition name="modal">
        <div
            v-if="ui.isOpen('download')"
            class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm"
            @click="cancel"
        >
            <div
                class="w-[420px] bg-card border border-border rounded-xl shadow-2xl p-5 flex flex-col gap-4"
                @click.stop
            >
                <!-- Header -->
                <div>
                    <h3
                        class="text-xs font-semibold text-muted-foreground uppercase tracking-widest"
                    >
                        New Download
                    </h3>
                </div>

                <!-- URL input + folder picker -->
                <div class="flex gap-2">
                    <Input
                        v-model="downloadLink"
                        placeholder="https://..."
                        class="flex-1 h-8 text-base"
                        autofocus
                        @keyup.enter="enter"
                    />
                    <Button
                        variant="outline"
                        size="icon"
                        class="size-8 shrink-0"
                        title="Select download folder"
                        @click="chooseDir"
                    >
                        <span class="icon text-sm">folder_open</span>
                    </Button>
                </div>

                <!-- Download dir preview -->
                <p class="text-xs text-muted-foreground truncate -mt-2">
                    <span class="icon text-xs align-middle mr-1">save</span>
                    {{ downloads.downloadDir || "No folder selected" }}
                </p>

                <!-- Error -->
                <div
                    v-if="error"
                    class="flex items-center justify-between rounded-md bg-destructive/10 border border-destructive/20 px-3 py-2"
                >
                    <p class="text-sm text-destructive">{{ error }}</p>
                    <Button
                        variant="ghost"
                        size="sm"
                        class="h-auto p-0 text-xs text-muted-foreground hover:text-foreground shrink-0 ml-3"
                        @click="copyDetails"
                    >
                        copy details
                    </Button>
                </div>

                <!-- Footer -->
                <div class="flex justify-end gap-2">
                    <Button
                        variant="ghost"
                        size="sm"
                        class="h-7 text-xs"
                        @click="cancel"
                    >
                        Cancel
                    </Button>
                    <Button
                        size="sm"
                        class="h-7 text-xs"
                        :disabled="!downloadLink"
                        @click="enter"
                    >
                        Download
                    </Button>
                </div>
            </div>
        </div>
    </Transition>
</template>
