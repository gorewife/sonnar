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
    <div v-if="show" class="modal-mask">
      <div class="modal-container">

        <div class="modal-header">
          <h3>Enter download link</h3>
        </div>

        <div class="modal-body">
          <input
            v-model="downloadLink"
            placeholder="Download Link..."
            @keyup.enter="enter"
          />
            <button @click="chooseDir" title="Select a directory">📁</button>
        </div>

        <div class="modal-footer">
          <button @click="cancel">Cancel</button>
          <button :disabled="!downloadLink" @click="enter">Enter</button>
        </div>

      </div>
    </div>
  </Transition>
</template>