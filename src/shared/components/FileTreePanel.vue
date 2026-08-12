<script setup lang="ts">
import { ref, watch } from "vue";
import { explorerReadDir, type FileEntry } from "@/tauri/commands/explorer";
import { friendlyError } from "@/tauri/commands/_base";
import FileTreeNode from "./FileTreeNode.vue";

const props = defineProps<{ root: string; hideHeader?: boolean }>();

const entries = ref<FileEntry[]>([]);
const loading = ref(false);
const error = ref("");

async function load(path: string) {
  if (!path) {
    entries.value = [];
    return;
  }
  loading.value = true;
  error.value = "";
  try {
    const result = await explorerReadDir(path);
    entries.value = result.entries;
  } catch (e) {
    error.value = friendlyError(e);
  } finally {
    loading.value = false;
  }
}

watch(() => props.root, load, { immediate: true });
</script>

<template>
  <div class="flex h-full flex-col overflow-hidden bg-sidebar text-sidebar-text">
    <div v-if="!hideHeader" class="flex shrink-0 items-center border-b border-sidebar-border px-3 py-2">
      <span class="flex-1 truncate text-[11px] font-semibold uppercase tracking-wide text-sidebar-text" :title="root">Explorer</span>
      <slot name="actions" />
    </div>
    <div class="min-h-0 flex-1 overflow-y-auto py-1">
      <div v-if="loading" class="flex items-center gap-2 px-3 py-2 text-xs text-sidebar-text">
        <i class="pi pi-spinner pi-spin" /> Loading…
      </div>
      <div v-else-if="error" class="px-3 py-2 text-xs text-red-500">{{ error }}</div>
      <div v-else-if="!entries.length" class="px-3 py-2 text-xs text-sidebar-text">Empty folder.</div>
      <FileTreeNode v-for="entry in entries" :key="entry.path" :entry="entry" :depth="0" />
    </div>
  </div>
</template>
