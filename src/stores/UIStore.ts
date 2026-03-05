import { defineStore } from "pinia";
import { ref, watch } from "vue";

export type ModalName = string | null;
export type ThemeMode = "system" | "light" | "dark";

export const useUIStore = defineStore("ui", () => {
  /* ---------------- MODALS ---------------- */

  const activeModal = ref<ModalName>(null);
  const modalProps = ref<Record<string, unknown>>({});

  function openModal(name: string, props: Record<string, unknown> = {}) {
    activeModal.value = name;
    modalProps.value = props;
  }

  function closeModal() {
    activeModal.value = null;
    modalProps.value = {};
  }

  const isOpen = (name: string) => activeModal.value === name;

  /* ---------------- THEME ---------------- */

  const themeMode = ref<ThemeMode>("system");

  function applyTheme() {
    const html = document.documentElement;

    const prefersDark = window.matchMedia(
      "(prefers-color-scheme: dark)",
    ).matches;

    const isDark =
      themeMode.value === "dark" ||
      (themeMode.value === "system" && prefersDark);

    html.classList.toggle("dark", isDark);
  }

  function setThemeMode(mode: ThemeMode) {
    themeMode.value = mode;
    localStorage.setItem("theme", mode);
    applyTheme();
  }

  function initTheme() {
    const saved = localStorage.getItem("theme") as ThemeMode | null;
    if (saved) themeMode.value = saved;

    applyTheme();

    // react to OS theme changes
    window
      .matchMedia("(prefers-color-scheme: dark)")
      .addEventListener("change", () => {
        if (themeMode.value === "system") {
          applyTheme();
        }
      });
  }

  // auto-apply when changed anywhere
  watch(themeMode, applyTheme);

  return {
    /* modals */
    activeModal,
    modalProps,
    openModal,
    closeModal,
    isOpen,

    /* theme */
    themeMode,
    setThemeMode,
    initTheme,
  };
});
