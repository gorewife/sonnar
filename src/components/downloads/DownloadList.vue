<script setup lang="ts">
import { useDownloadsStore } from "@/stores/DownloadsStore";
import DownloadRow from "./DownloadRow.vue";
import { Button } from "@/components/ui/button";

const downloads = useDownloadsStore();
</script>

<template>
    <div>
        <div class="flex items-center justify-between mb-4">
            <h3 class="text-xs font-semibold text-muted-foreground uppercase tracking-widest">
                Recent
            </h3>
            <Button
                v-if="downloads.downloads.some((d) => d.status === 'finished')"
                variant="ghost"
                size="sm"
                class="h-6 text-xs text-muted-foreground hover:text-foreground px-2"
            >
                Clear finished
            </Button>
        </div>

        <!-- Empty state -->
        <div
            v-if="!downloads.downloads.length"
            class="flex flex-col items-center justify-center py-20 text-center"
        >
            <span class="icon text-4xl text-muted-foreground/25 mb-3">inbox</span>
            <p class="text-sm text-muted-foreground/40">No downloads yet</p>
        </div>

        <!-- List -->
        <div v-else class="flex flex-col gap-1.5">
            <DownloadRow
                v-for="dl in downloads.downloads"
                :key="dl.info.id"
                :download="dl"
            />
        </div>
    </div>
</template>
