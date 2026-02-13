import { defineStore } from 'pinia'
import { ref } from 'vue'

export type ModalName = 'newDownload' | 'settings' | 'confirm' | null

export const useUIStore = defineStore('ui', () => {
  const activeModal = ref<ModalName>(null)
  const modalProps = ref<Record<string, any>>({})

  function openModal(name: ModalName, props = {}) {
    activeModal.value = name
    modalProps.value = props
  }

  function closeModal() {
    activeModal.value = null
    modalProps.value = {}
  }

  return { activeModal, modalProps, openModal, closeModal }
})