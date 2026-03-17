<script setup lang="ts">
import Sidebar from '@/components/Sidebar.vue'
import DownloadModal from '@/components/modals/DownloadModal.vue'
import DownloadList from '@/components/downloads/DownloadList.vue'
</script>

<template>
  <main class="container">
    <div class="titlebar" data-tauri-drag-region />

    <div class="app-layout">
      <!-- Sidebar -->
      <aside class="sidebar">
        <Sidebar />
      </aside>

      <!-- Content -->
      <section class="content">
        <DownloadList />
      </section>
    </div>
  </main>

  <!-- Modal -->
  <Teleport to="body">
    <DownloadModal />
  </Teleport>
</template>
