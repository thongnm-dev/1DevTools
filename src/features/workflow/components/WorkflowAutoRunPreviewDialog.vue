<script setup lang="ts">
import { onBeforeUnmount, ref } from "vue";
import { useI18n } from "vue-i18n";
import Dialog from "primevue/dialog";
import WorkflowDiagramPreview from "./WorkflowDiagramPreview.vue";
import type { Workflow } from "@/models/workflow";

defineProps<{ workflow: Workflow | null }>();
const visible = defineModel<boolean>("visible", { default: false });

const { t } = useI18n();

// --- Resize dialog xem-trước bằng cách kéo góc dưới-phải ---
const previewWidth = ref(768);
const previewHeight = ref(480);
const MIN_WIDTH = 480;
const MIN_HEIGHT = 300;
let cleanupResize: (() => void) | null = null;

function startResize(event: MouseEvent) {
  event.preventDefault();
  const startX = event.clientX;
  const startY = event.clientY;
  const startWidth = previewWidth.value;
  const startHeight = previewHeight.value;
  function onMove(ev: MouseEvent) {
    previewWidth.value = Math.max(MIN_WIDTH, startWidth + (ev.clientX - startX));
    previewHeight.value = Math.max(MIN_HEIGHT, startHeight + (ev.clientY - startY));
  }
  function onUp() {
    document.removeEventListener("mousemove", onMove);
    document.removeEventListener("mouseup", onUp);
    document.body.style.userSelect = "";
    cleanupResize = null;
  }
  document.body.style.userSelect = "none";
  document.addEventListener("mousemove", onMove);
  document.addEventListener("mouseup", onUp);
  cleanupResize = onUp;
}

onBeforeUnmount(() => cleanupResize?.());
</script>

<template>
  <Dialog
    :visible="visible"
    class="rounded-lg bg-panel shadow-xl"
    :style="{ width: previewWidth + 'px', height: previewHeight + 'px' }"
    :closable="true"
    modal
    @update:visible="visible = $event"
  >
    <template #header>
      <div class="flex min-w-0 items-center gap-2">
        <i :class="[workflow?.icon, 'text-brand']" />
        <h3 class="section-title truncate">{{ workflow?.name }}</h3>
        <span v-if="workflow" class="shrink-0 text-xs text-muted">
          {{ t("workflow.stepCount", { count: workflow.step_count }) }}
        </span>
      </div>
    </template>

    <WorkflowDiagramPreview v-if="workflow" :workflow="workflow" />

    <div class="dialog-resize-handle" :title="t('common.resize')" @mousedown="startResize" />
  </Dialog>
</template>
