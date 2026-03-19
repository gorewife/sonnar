<script setup lang="ts">
import { computed } from "vue";
import type { ActiveDownload } from "@/types/downloads";

const { download } = defineProps<{ download: ActiveDownload }>();

const progress = computed(() => {
    const total = download.info.total_bytes;
    if (!total) return null;
    return Math.round((download.currentByte / total) * 100);
});
</script>

<template>
    <div class="row" :data-status="download.status">
        <div class="row-body">
            <span class="row-name">{{ download.info.default_name }}</span>
            <span class="row-meta">
                <template v-if="download.status === 'downloading'">
                    {{ progress != null ? `${progress}%` : "···" }}
                </template>
                <template v-else-if="download.status === 'error'">
                    {{ download.error }}
                </template>
                <template v-else-if="download.status === 'finished'">
                    done
                </template>
            </span>
        </div>
  <div :class="['row', `is-${download.status}`]">
    <div class="row-body">
      <span class="row-name">{{ download.info.default_name }}</span>
      <span class="row-meta">
        <template v-if="download.status === 'downloading'">
          {{ progress != null ? `${progress}%` : '···' }}
        </template>
        <template v-else-if="download.status === 'error'">
          {{ download.error }}
        </template>
        <template v-else-if="download.status === 'finished'">
          done
        </template>
      </span>
    </div>

        <div class="row-line">
            <div
                v-if="download.status === 'downloading'"
                class="row-line-fill"
                :style="{ width: `${progress ?? 0}%` }"
            />
        </div>
    </div>
</template>

<style scoped>
.row {
    display: flex;
    flex-direction: column;
}

.row[data-status="finished"] .row-name {
    opacity: 0.35;
.row.is-finished .row-name {
  opacity: 0.35;
}

.row-body {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    gap: 2rem;
    padding: 0.875rem 0;
}

.row-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    min-width: 0;
    flex: 1;
}

.row-meta {
    font-size: 0.8em;
    flex-shrink: 0;
    color: var(--color-highlight);
    opacity: 0.7;
}

.row[data-status="error"] .row-meta {
    color: var(--color-text);
    opacity: 0.6;
}

.row[data-status="finished"] .row-meta {
    color: var(--color-text);
    opacity: 0.25;
.row.is-error .row-meta {
  color: var(--color-text);
  opacity: 0.6;
}

.row.is-finished .row-meta {
  color: var(--color-text);
  opacity: 0.25;
}

/* separator that doubles as progress bar */
.row-line {
    height: 1px;
    background: var(--color-border);
    position: relative;
}

.row-line-fill {
    position: absolute;
    inset: 0;
    background: var(--color-highlight);
    transition: width 0.1s linear;
}
</style>
