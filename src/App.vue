<script setup lang="ts">
import { onMounted } from "vue";
import MainWindow from "@/layouts/MainWindow.vue";
import { useDownloadsStore } from "@/stores/DownloadsStore";

const downloads = useDownloadsStore();
onMounted(() => downloads.init());
</script>

<template>
    <MainWindow />
</template>
