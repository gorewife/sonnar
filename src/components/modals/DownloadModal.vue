<script setup lang="ts">
import { ref, watchEffect } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { useUIStore } from '@/stores/UIStore'
import { useDownloadsStore } from '@/stores/DownloadsStore'

const ui = useUIStore()
const downloads = useDownloadsStore()

const downloadLink = ref('')

// reset link when modal opens
watchEffect(() => {
  if (ui.isOpen('download')) downloadLink.value = ''
})

async function chooseDir() {
  const path = await open({ directory: true })
  if (typeof path === 'string') {
    downloads.setDownloadDir(path)
  }
}

function cancel() {
  ui.closeModal()
}

function enter() {
  if (!downloadLink.value) return

  // TODO: call downloads store / trigger actual download here

  ui.closeModal()
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
