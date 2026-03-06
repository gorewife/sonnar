<script setup lang="ts">
import Sidebar from '@/components/Sidebar.vue'
import DownloadModal from '@/components/modals/DownloadModal.vue'
</script>

<template>
  <main class="container">
    <div class="app-layout">
      <!-- Sidebar -->
      <aside class="sidebar">
        <Sidebar />
      </aside>

      <!-- Content -->
      <section class="content">
        <h1>Sonnar</h1>
      </section>
    </div>
  </main>

  <!-- Modal -->
  <Teleport to="body">
    <DownloadModal />
  </Teleport>
</template>
