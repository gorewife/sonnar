import { defineStore } from 'pinia'
import { ref } from 'vue'

export type ModalName = string | null

export const useUIStore = defineStore('ui', () => {
  const activeModal = ref<ModalName>(null)
  const modalProps = ref<Record<string, unknown>>({})

  function openModal(name: string, props: Record<string, unknown> = {}) {
    activeModal.value = name
    modalProps.value = props
  }

  function closeModal() {
    activeModal.value = null
    modalProps.value = {}
  }

  const isOpen = (name: string) =>
    activeModal.value === name

  return {
    activeModal,
    modalProps,
    openModal,
    closeModal,
    isOpen
  }
})
