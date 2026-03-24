<script setup lang="ts">
import { computed } from "vue";
import { useUIStore } from "@/stores/UIStore";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import OphLogo from "@/components/OphLogo.vue";

const ui = useUIStore();

const navItems = [
    { label: "Downloads", icon: "inbox", filter: "all" },
    { label: "Active", icon: "downloading", filter: "downloading" },
    { label: "Finished", icon: "check_circle", filter: "finished" },
    { label: "Paused", icon: "pause_circle", filter: "paused" },
];

const activeFilter = computed(() => "all");

// Mock — wired to backend in Phase 1
const diskUsedPercent = 62;
const diskAvailable = "284 GB";
</script>

<template>
    <aside
        class="w-56 shrink-0 h-screen flex flex-col border-r border-border bg-sidebar overflow-y-auto custom-scrollbar"
    >
        <!-- macOS traffic light clearance -->
        <div class="h-10 shrink-0" />

        <!-- Logo -->
        <div class="px-3 mb-6">
            <div class="flex items-center gap-2">
                <OphLogo :size="28" />
                <span class="text-lg font-semibold text-foreground tracking-tight">ophelia</span>
            </div>
        </div>

        <!-- New Download -->
        <div class="px-3 mb-4">
            <Button
                class="w-full justify-start gap-2 h-8 text-sm font-medium"
                @click="ui.openModal('download')"
            >
                <span class="icon text-sm">add</span>
                New Download
            </Button>
        </div>

        <Separator class="mb-3" />

        <!-- Nav -->
        <nav class="px-2 flex flex-col gap-0.5">
            <a
                v-for="item in navItems"
                :key="item.filter"
                href="#"
                class="flex items-center gap-3 px-3 py-2 rounded-md text-base font-medium transition-colors"
                :class="
                    activeFilter === item.filter
                        ? 'bg-accent text-foreground'
                        : 'text-muted-foreground hover:text-foreground hover:bg-accent/60'
                "
                @click.prevent
            >
                <span class="icon text-base">{{ item.icon }}</span>
                {{ item.label }}
            </a>
        </nav>

        <!-- Storage card -->
        <div class="mt-auto p-3">
            <div class="rounded-lg border border-border bg-card p-3">
                <div class="flex items-center justify-between mb-3">
                    <div class="flex items-center gap-2">
                        <span class="icon text-base text-secondary">database</span>
                        <span class="text-xs font-medium text-muted-foreground">Storage</span>
                    </div>
                    <span class="text-2xs text-muted-foreground/60">{{ diskUsedPercent }}%</span>
                </div>
                <p class="text-md font-semibold text-foreground mb-0.5">{{ diskAvailable }}</p>
                <p class="text-xs text-muted-foreground mb-2.5">available</p>
                <div class="w-full h-1 bg-muted rounded-full overflow-hidden">
                    <div
                        class="h-full bg-secondary rounded-full transition-all"
                        :style="{ width: `${diskUsedPercent}%` }"
                    />
                </div>
            </div>
        </div>
    </aside>
</template>
