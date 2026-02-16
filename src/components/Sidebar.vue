<script setup lang="ts">
import { useUIStore } from '@/stores/UIStore'

const ui = useUIStore()
</script>

<template>
  <button class="add-btn" @click="ui.openModal('download')">
    <font-awesome-icon icon="plus" />
  </button>

  <button class="set-btn" @click="ui.openModal('settings')">
    <font-awesome-icon icon="gear" />
  </button>
</template>

<style scoped>
.add-btn:hover {
  filter: drop-shadow(1px 1px 1em #747bff);
}

.set-btn {
  margin-top: auto;
  margin-bottom: 2vh;
}
</style>
