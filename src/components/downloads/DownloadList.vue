<script setup lang="ts">
import { useDownloadsStore } from "@/stores/DownloadsStore";
import DownloadRow from "./DownloadRow.vue";

const downloads = useDownloadsStore();
</script>

<template>
    <div class="list">
        <div v-if="!downloads.downloads.length" class="empty">nothing yet</div>
        <DownloadRow
            v-for="dl in downloads.downloads"
            :key="dl.info.id"
            :download="dl"
        />
    </div>
</template>

<style scoped>
.list {
    display: flex;
    flex-direction: column;
}

.empty {
    padding: 2rem 0;
    font-size: 0.8em;
    opacity: 0.2;
}
</style>
