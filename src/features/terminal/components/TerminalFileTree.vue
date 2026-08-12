<script setup lang="ts">
import FileTreePanel from "@/shared/components/FileTreePanel.vue";

defineProps<{ root: string }>();
</script>

<template>
  <FileTreePanel :root="root" />
</template>
