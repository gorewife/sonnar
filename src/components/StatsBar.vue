<script setup lang="ts">
import { computed } from "vue";
import { useDownloadsStore } from "@/stores/DownloadsStore";
import { useSpeedTracker } from "@/composables/useSpeedTracker";
import { Card, CardContent } from "@/components/ui/card";

const downloads = useDownloadsStore();
const { downloadSpeed } = useSpeedTracker();

const activeCount = computed(
    () => downloads.downloads.filter((d) => d.status === "downloading").length,
);

const finishedCount = computed(
    () => downloads.downloads.filter((d) => d.status === "finished").length,
);

const formattedSpeed = computed(() => {
    const s = downloadSpeed.value;
    if (s === 0) return "0 KB/s";
    if (s < 1024) return `${s.toFixed(0)} B/s`;
    if (s < 1024 * 1024) return `${(s / 1024).toFixed(1)} KB/s`;
    return `${(s / 1024 / 1024).toFixed(2)} MB/s`;
});

const stats = computed(() => [
    { label: "Speed", value: formattedSpeed.value, icon: "sensors" },
    { label: "Active", value: String(activeCount.value), icon: "downloading" },
    { label: "Finished", value: String(finishedCount.value), icon: "check_circle" },
    { label: "Total", value: String(downloads.downloads.length), icon: "inbox" },
]);
</script>

<template>
    <div class="grid grid-cols-4 gap-3 mb-8">
        <Card
            v-for="stat in stats"
            :key="stat.label"
            class="gap-0 py-4 px-4 rounded-lg"
        >
            <CardContent class="p-0">
                <p class="text-xs text-muted-foreground font-medium mb-2 flex items-center gap-1.5">
                    <span class="icon text-xs">{{ stat.icon }}</span>
                    {{ stat.label }}
                </p>
                <p class="text-2xl font-semibold text-foreground tracking-tight">
                    {{ stat.value }}
                </p>
            </CardContent>
        </Card>
    </div>
</template>
