<script setup lang="ts">
import AppSidebar from "@/components/AppSidebar.vue";
import StatsBar from "@/components/StatsBar.vue";
import DownloadList from "@/components/downloads/DownloadList.vue";
import DownloadModal from "@/components/modals/DownloadModal.vue";
import { Input } from "@/components/ui/input";
</script>

<template>
    <div class="flex h-screen overflow-hidden bg-background text-foreground">
        <AppSidebar />

        <main class="flex-1 flex flex-col overflow-hidden">
            <!-- Titlebar: full-width drag region with centred search -->
            <header
                class="h-10 shrink-0 flex items-center justify-center px-6 border-b border-border relative"
                data-tauri-drag-region
            >
                <!-- Search sits in the centre; pointer-events re-enabled so it's clickable -->
                <div class="relative w-56" style="-webkit-app-region: no-drag">
                    <span class="material-symbols-outlined absolute left-2.5 top-1/2 -translate-y-1/2 text-muted-foreground text-[14px] pointer-events-none">
                        search
                    </span>
                    <Input
                        placeholder="Search..."
                        class="pl-8 h-6 text-[12px] bg-muted/40 border-transparent focus:border-border focus:bg-background rounded-md"
                    />
                </div>
            </header>

            <!-- Page content -->
            <div class="flex-1 overflow-y-auto px-6 py-5 custom-scrollbar">
                <StatsBar />
                <DownloadList />
            </div>
        </main>

        <Teleport to="body">
            <DownloadModal />
        </Teleport>
    </div>
</template>
