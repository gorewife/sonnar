<script setup lang="ts">
import { computed } from "vue";
import type { ActiveDownload } from "@/types/downloads";
import { useDownloadsStore } from "@/stores/DownloadsStore";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Progress } from "@/components/ui/progress";

const { download } = defineProps<{ download: ActiveDownload }>();
const downloads = useDownloadsStore();

const progress = computed(() => {
    const total = download.info.total_bytes;
    if (!total) return null;
    return Math.round((download.currentByte / total) * 100);
});

const formattedSize = computed(() => {
    const total = download.info.total_bytes;
    if (!total) return "—";
    if (total < 1024 * 1024) return `${(total / 1024).toFixed(1)} KB`;
    if (total < 1024 * 1024 * 1024)
        return `${(total / 1024 / 1024).toFixed(1)} MB`;
    return `${(total / 1024 / 1024 / 1024).toFixed(2)} GB`;
});

const fileIcon = computed(() => {
    const name = download.info.default_name.toLowerCase();
    if (/\.(mp4|mkv|avi|mov|webm)$/.test(name)) return "movie";
    if (/\.(mp3|flac|wav|aac|ogg)$/.test(name)) return "music_note";
    if (/\.(zip|tar|gz|rar|7z)$/.test(name)) return "folder_zip";
    if (/\.(jpg|jpeg|png|gif|webp|svg)$/.test(name)) return "image";
    if (/\.(pdf)$/.test(name)) return "picture_as_pdf";
    if (/\.(exe|dmg|pkg|deb|appimage)$/.test(name)) return "install_desktop";
    return "draft";
});

const badgeVariant = computed(() => {
    if (download.status === "error") return "destructive" as const;
    if (download.status === "finished") return "secondary" as const;
    return "outline" as const;
});
</script>

<template>
    <div
        class="flex items-center gap-4 px-4 py-3 rounded-lg border border-border bg-card hover:bg-accent/30 transition-colors"
        :class="{ 'opacity-40': download.status === 'finished' }"
    >
        <!-- File type icon -->
        <span
            class="icon text-base shrink-0"
            :class="
                download.status === 'error'
                    ? 'text-destructive'
                    : 'text-muted-foreground'
            "
        >
            {{ download.status === "error" ? "error_outline" : fileIcon }}
        </span>

        <!-- Name + progress -->
        <div class="flex-1 min-w-0">
            <p
                class="text-base font-medium text-foreground truncate leading-snug"
            >
                {{ download.info.default_name }}
            </p>
            <Progress
                v-if="download.status === 'downloading'"
                :model-value="progress ?? 0"
                class="h-0.5 mt-1.5 bg-border"
            />
            <p
                v-else-if="download.status === 'error'"
                class="text-xs text-destructive mt-0.5 truncate"
            >
                {{ download.error }}
            </p>
        </div>

        <!-- File size -->
        <span class="text-xs text-muted-foreground shrink-0 tabular-nums">
            {{ formattedSize }}
        </span>

        <!-- Progress % -->
        <span
            v-if="download.status === 'downloading'"
            class="text-xs text-muted-foreground shrink-0 tabular-nums w-8 text-right"
        >
            {{ progress != null ? `${progress}%` : "···" }}
        </span>

        <!-- Status badge -->
        <Badge :variant="badgeVariant" class="text-2xs shrink-0">
            {{ download.status }}
        </Badge>

        <!-- Actions -->
        <div class="flex items-center gap-0.5 shrink-0">
            <Button
                v-if="download.status === 'downloading'"
                variant="ghost"
                size="icon"
                class="size-6"
                title="Pause"
            >
                <span class="icon text-sm">pause</span>
            </Button>
            <Button
                v-if="download.status !== 'finished'"
                variant="ghost"
                size="icon"
                class="size-6 hover:text-destructive hover:bg-destructive/10"
                title="Cancel"
                @click="downloads.cancelDownload(download.info.id)"
            >
                <span class="icon text-sm">close</span>
            </Button>
        </div>
    </div>
</template>
