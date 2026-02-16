<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { useUIStore } from '@/stores/UIStore'

const ui = useUIStore()

const downloadLink = ref('')
const downloadDir = ref('')

// load saved download directory
onMounted(() => {
  const saved = localStorage.getItem('downloadDir')
  if (saved) downloadDir.value = saved
})

// reset link when modal opens
watch(() => ui.activeModal, (newVal) => {
  if (newVal === 'download') downloadLink.value = ''
})

async function chooseDir() {
  const path = await open({ directory: true })
  if (path) {
    downloadDir.value = path
    localStorage.setItem('downloadDir', path)
  }
}

function cancel() {
  ui.closeModal()
}

function enter() {
  if (!downloadLink.value) return

  console.log('Download submitted:', downloadLink.value)
  // TODO: call downloads store / trigger actual download here

  ui.closeModal()
  downloadLink.value = '' // reset
}
</script>

<template>
  <Transition name="modal">
    <div v-if="ui.isOpen('download')" class="modal-backdrop" @click="cancel">
      <div class="modal-dialog" @click.stop>
        <header class="modal-header">
          <h3>Enter download link</h3>
        </header>

        <section class="modal-body">
          <input v-model="downloadLink" placeholder="Download Link..." @keyup.enter="enter" />
          <button @click="chooseDir" title="Select a directory">
            <font-awesome-icon icon="folder" />
          </button>
        </section>

        <footer class="modal-footer">
          <button @click="cancel">Cancel</button>
          <button :disabled="!downloadLink" @click="enter">Enter</button>
        </footer>
      </div>
    </div>
  </Transition>
</template>
