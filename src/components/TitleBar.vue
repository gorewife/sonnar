<script setup lang="ts">
</script>

<template>
  <header class="titlebar">
  </header>
</template>

<style scoped>
.titlebar {
  height: 32px;

  display: flex;
  align-items: center;
  padding-left: 80px;
  /* space for traffic lights */

  background: rgba(11, 17, 23, 0.6);
  backdrop-filter: blur(20px);

  -webkit-app-region: drag;
}

.title {
  font-size: 13px;
  opacity: 0.8;
}
</style>
