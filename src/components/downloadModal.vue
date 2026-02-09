<script setup>
import { ref, watch, onMounted } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'

const props = defineProps({ show: Boolean })
const emit = defineEmits(['close', 'submit'])

const downloadLink = ref('')
const downloadDir = ref('')

onMounted(() => {
  const saved = localStorage.getItem('downloadDir')
  if (saved) downloadDir.value = saved
})

watch(() => props.show, (newVal) => {
  if (newVal && !downloadDir.value) downloadDir.value = ''
})

async function chooseDir() {
  const path = await open({ directory: true })
  if (path) {
    downloadDir.value = path
    localStorage.setItem('downloadDir', path)
  }
}

function cancel() {
  emit('close')
}

function enter() {
  if (!downloadLink.value) return
  emit('submit', downloadLink.value)
  emit('close')
}
</script>

<template>
  <Transition name="modal">
    <div v-if="show" class="modal-backdrop" @click="cancel">
      <div class="modal-dialog" @click.stop>

        <header class="modal-header">
          <h3>Enter download link</h3>
        </header>

        <section class="modal-body">
          <input
            v-model="downloadLink"
            placeholder="Download Link..."
            @keyup.enter="enter"
          />
            <button
            @click="chooseDir" 
            title="Select a directory"
          >📁</button>
        </section>

        <footer class="modal-footer">
          <button @click="cancel">Cancel</button>
          <button :disabled="!downloadLink" @click="enter">Enter</button>
        </footer>

      </div>
    </div>
  </Transition>
</template>

<style>
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

</style>