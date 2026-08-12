<script setup lang="ts">
import type { FileEntry } from "@/tauri/commands/explorer";
import FileTreeNode from "@/shared/components/FileTreeNode.vue";

defineProps<{ entry: FileEntry; depth: number }>();
</script>

<template>
  <FileTreeNode :entry="entry" :depth="depth" />
</template>
