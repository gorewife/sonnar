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
  padding: 1rem 0;
}

.sidebar-nav button {
  border: none;
  background: transparent;
  padding: 0.6rem;
  font-size: 1em;
  opacity: 0.35;
  transition: opacity 0.15s;
  letter-spacing: 0;
  text-transform: none;
}

.sidebar-nav button:hover {
  border: none;
  opacity: 1;
}

.set-btn {
  margin-top: auto;
  margin-bottom: 1rem;
}
</style>
