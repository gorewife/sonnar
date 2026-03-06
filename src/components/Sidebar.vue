<script setup lang="ts">
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { useUIStore } from '@/stores/UIStore'

const ui = useUIStore()
</script>

<template>
  <nav class="sidebar-nav">
    <button class="add-btn" @click="ui.openModal('download')">
      <FontAwesomeIcon icon="plus" />
    </button>

    <button class="set-btn" @click="ui.openModal('settings')">
      <FontAwesomeIcon icon="gear" />
    </button>
  </nav>
</template>

<style scoped>
.sidebar-nav {
  display: flex;
  flex-direction: column;
  align-items: center;
  flex: 1;
  padding-top: 1rem;
}

.add-btn:hover {
  filter: drop-shadow(1px 1px 1em var(--color-highlight));
}

.set-btn {
  margin-top: auto;
  margin-bottom: 2vh;
}
</style>
