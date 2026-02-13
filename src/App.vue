<script setup lang="ts">
import { ref } from "vue";
// import { invoke } from "@tauri-apps/api/core";
import downloadModal from "./components/downloadModal.vue";


const downloadLink = ref("");
const showModal = ref(false);

/* async function greet() {
  greetMsg.value = await invoke("greet", { name: name.value });
} */

</script>

<template>
  <main class="container">
    <div class="app-layout">
      <!-- Sidebar -->
      <aside class="sidebar">
        <button class="add-btn" @click="showModal = true">
          <font-awesome-icon icon="plus" />
        </button>

        <button class="set-btn">
          <font-awesome-icon icon="gear" />
        </button>
      </aside>

      <!-- Content -->
      <section class="content">
        <h1>
          Sonnar
        </h1>
      </section>
    </div>
  </main>

  <!-- Modal -->
  <Teleport to="body">
    <downloadModal :show="showModal" @close="showModal = false" @submit="downloadLink = $event" />
  </Teleport>
</template>

<style scoped>
.add-btn:hover {
  filter: drop-shadow(1 1 1em #747bff);
}

.set-btn {
  margin-top: auto;
  margin-bottom: 2vh;
}
</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;
  height: 100%;

  --color-bg: #0b1117;
  --color-text: #ff7c72;
  --color-highlight: #79c0ff;
  --color-border: #1f2a37;
  --color-surface: #0f141b;

  color: var(--color-text);
  background-color: var(--color-bg);

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

html,
body {
  margin: 0;
  padding: 0;
}

.container {
  margin: 0;
  padding: 0;
}

.app-layout {
  display: flex;
  min-height: 100vh;
}

.content {
  flex: 1;
  padding: 1.5rem;
}

.sidebar {
  width: 9vw;
  background-color: var(--color-surface);
  border-right: 4px solid var(--color-border);

  display: flex;
  flex-direction: column;
  align-items: center;
  padding-top: 1rem;
}

/* modals */
.modal-backdrop {
  display: grid;
  place-items: center;
  z-index: 1000;

  position: fixed;
  inset: 0;
  background: rgba(11, 17, 23, 0.75);
  backdrop-filter: blur(6px);
}

.modal-dialog {
  background: var(--color-surface);
  border-radius: 12px;
  padding: 1.5rem;
}

/* links & highlights */
a {
  font-weight: 500;
  color: var(--color-highlight);
  text-decoration: inherit;
}

a:hover {
  text-decoration: underline;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 6px;
  border: 1px solid var(--color-border);
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;

  color: var(--color-text);
  background-color: var(--color-surface);

  transition: border-color 0.25s, background-color 0.25s;
}

button {
  cursor: pointer;
}

button:hover {
  border-color: var(--color-highlight);
}

button:active {
  background-color: rgba(121, 192, 255, 0.1);
}

input::placeholder {
  opacity: 0.6;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}
</style>