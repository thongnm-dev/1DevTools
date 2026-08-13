<script setup lang="ts">
import { ref, watch, computed } from "vue";
import { useI18n } from "vue-i18n";
import { marked } from "marked";
import Dialog from "primevue/dialog";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import { explorerReadFile } from "@/tauri/commands/explorer";

const { t } = useI18n();

const props = defineProps<{
  filePath: string;
}>();

const visible = defineModel<boolean>("visible", { default: false });

const content = ref("");
const loading = ref(false);
const error = ref("");
const isMaximized = ref(false);

const fileName = computed(() => {
  if (!props.filePath) return "";
  return props.filePath.split(/[\\/]/).filter(Boolean).pop() ?? "";
});

const rendered = computed(() => {
  if (!content.value) return "";
  return marked.parse(content.value, { async: false }) as string;
});

watch(visible, async (v) => {
  if (!v || !props.filePath) return;
  isMaximized.value = false;
  loading.value = true;
  error.value = "";
  try {
    content.value = await explorerReadFile(props.filePath);
  } catch (e) {
    error.value = String(e);
    content.value = "";
  } finally {
    loading.value = false;
  }
});
</script>

<template>
  <Dialog
    v-model:visible="visible"
    modal
    maximizable
    :header="fileName || t('markdownPreview.title')"
    :style="{ width: '780px' }"
    :pt="{ root: { class: 'md-preview-dialog' } }"
    @maximize="isMaximized = true"
    @unmaximize="isMaximized = false"
  >
    <!-- Loading -->
    <div v-if="loading" class="flex items-center justify-center py-12">
      <i class="pi pi-spinner pi-spin mr-2 text-brand" />
      <span class="text-sm text-muted">{{ t("common.loading") }}</span>
    </div>

    <!-- Error -->
    <div v-else-if="error" class="flex flex-col items-center gap-2 py-12 text-center">
      <i class="pi pi-exclamation-triangle text-2xl text-red-500" />
      <p class="text-sm text-red-500">{{ error }}</p>
    </div>

    <!-- Markdown content -->
    <div
      v-else
      class="md-body"
      :class="isMaximized ? '' : 'h-[520px] overflow-y-auto overflow-x-hidden'"
      v-html="rendered"
    />

    <template #footer>
      <DialogFooter
        :cancel-label="t('common.close')"
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        hide-confirm
        @cancel="visible = false"
      />
    </template>
  </Dialog>
</template>

<style>
.md-preview-dialog.p-dialog-maximized .p-dialog-content {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
}

.md-body {
  font-size: 14px;
  line-height: 1.7;
  color: rgb(var(--color-ink));
  word-wrap: break-word;
}

.md-body h1,
.md-body h2,
.md-body h3,
.md-body h4,
.md-body h5,
.md-body h6 {
  margin-top: 1.25em;
  margin-bottom: 0.5em;
  font-weight: 600;
  line-height: 1.3;
  color: rgb(var(--color-ink));
}

.md-body h1 { font-size: 1.6em; border-bottom: 1px solid rgb(var(--color-border)); padding-bottom: 0.3em; }
.md-body h2 { font-size: 1.35em; border-bottom: 1px solid rgb(var(--color-border)); padding-bottom: 0.25em; }
.md-body h3 { font-size: 1.15em; }
.md-body h4 { font-size: 1em; }

.md-body p { margin: 0.6em 0; }

.md-body a {
  color: rgb(var(--color-brand));
  text-decoration: none;
}
.md-body a:hover { text-decoration: underline; }

.md-body ul,
.md-body ol {
  margin: 0.5em 0;
  padding-left: 1.8em;
}
.md-body li { margin: 0.25em 0; }
.md-body li > ul,
.md-body li > ol { margin: 0.15em 0; }

.md-body blockquote {
  margin: 0.6em 0;
  padding: 0.4em 1em;
  border-left: 3px solid rgb(var(--color-brand) / 0.5);
  color: rgb(var(--color-muted));
  background: rgb(var(--color-panel));
  border-radius: 0 4px 4px 0;
}

.md-body code {
  font-family: "JetBrains Mono", "Fira Code", "Cascadia Code", Consolas, monospace;
  font-size: 0.88em;
  padding: 0.15em 0.35em;
  border-radius: 4px;
  background: rgb(var(--color-panel));
  color: rgb(var(--color-brand));
}

.md-body pre {
  margin: 0.6em 0;
  padding: 0.8em 1em;
  border-radius: 6px;
  background: rgb(var(--color-panel));
  overflow-x: auto;
  border: 1px solid rgb(var(--color-border));
}

.md-body pre code {
  padding: 0;
  background: transparent;
  color: rgb(var(--color-ink));
  font-size: 0.85em;
  line-height: 1.5;
}

.md-body table {
  margin: 0.6em 0;
  border-collapse: collapse;
  width: 100%;
  overflow-x: auto;
  display: block;
}

.md-body th,
.md-body td {
  border: 1px solid rgb(var(--color-border));
  padding: 0.4em 0.8em;
  text-align: left;
  font-size: 0.92em;
}

.md-body th {
  font-weight: 600;
  background: rgb(var(--color-panel));
}

.md-body tr:nth-child(even) {
  background: rgb(var(--color-panel) / 0.4);
}

.md-body hr {
  margin: 1.2em 0;
  border: none;
  border-top: 1px solid rgb(var(--color-border));
}

.md-body img {
  max-width: 100%;
  border-radius: 6px;
}

.md-body input[type="checkbox"] {
  margin-right: 0.4em;
}
</style>
