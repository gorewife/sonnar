<script setup lang="ts">
import { onMounted } from 'vue'
import MainWindow from '@/layouts/MainWindow.vue'
import { useDownloadsStore } from '@/stores/DownloadsStore'
import { useTray } from '@/composables/useTray'

const downloads = useDownloadsStore()
useTray()

onMounted(() => {
  downloads.init()
})
</script>

<template>
  <MainWindow />
</template>
