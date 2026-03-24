<template>
    <svg
        :width="size"
        :height="size"
        viewBox="0 0 24 24"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
    >
        <path
            fill-rule="evenodd"
            d="M12 3 A9 9 0 1 0 12 21 A9 9 0 1 0 12 3 Z
               M15 4.5 A7.5 7.5 0 1 0 15 19.5 A7.5 7.5 0 1 0 15 4.5 Z"
            fill="var(--primary)"
        />
        <circle cx="15" cy="12" r="1.5" fill="var(--primary)" opacity="0.45" />
    </svg>
</template>

<script setup lang="ts">
withDefaults(defineProps<{ size?: number }>(), { size: 24 });
</script>
