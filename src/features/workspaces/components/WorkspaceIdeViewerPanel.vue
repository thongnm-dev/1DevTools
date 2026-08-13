<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import hljs from "highlight.js/lib/common";
import { explorerReadFile } from "@/tauri/commands/explorer";
import { friendlyError } from "@/tauri/commands/_base";

const props = defineProps<{ filePath: string; hideHeader?: boolean }>();
const { t } = useI18n();

const content = ref("");
const loading = ref(false);
const error = ref("");

const fileName = computed(() => props.filePath.split(/[\\/]/).filter(Boolean).pop() ?? "");

/** Vài extension phổ biến trong project JS/TS/Vue mà tên không khớp trực tiếp với hljs. */
const LANG_ALIAS: Record<string, string> = {
  vue: "xml",
  tsx: "typescript",
  jsx: "javascript",
  mjs: "javascript",
  cjs: "javascript",
  yml: "yaml",
};

const highlightedHtml = computed(() => {
  if (!content.value) return "";
  const ext = fileName.value.split(".").pop()?.toLowerCase() ?? "";
  const lang = LANG_ALIAS[ext] ?? ext;
  try {
    if (lang && hljs.getLanguage(lang)) {
      return hljs.highlight(content.value, { language: lang }).value;
    }
    return hljs.highlightAuto(content.value).value;
  } catch {
    return content.value.replace(/[&<>]/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;" })[c] ?? c);
  }
});

const lineCount = computed(() => (content.value ? content.value.split("\n").length : 0));

watch(
  () => props.filePath,
  async (path) => {
    if (!path) {
      content.value = "";
      return;
    }
    loading.value = true;
    error.value = "";
    try {
      content.value = await explorerReadFile(path);
    } catch (e) {
      error.value = friendlyError(e);
      content.value = "";
    } finally {
      loading.value = false;
    }
  },
  { immediate: true },
);
</script>

<template>
  <div class="flex h-full flex-col overflow-hidden bg-sidebar text-sidebar-text">
    <div v-if="!hideHeader" class="flex shrink-0 items-center gap-1.5 border-b border-sidebar-border px-3 py-2">
      <i class="pi pi-code shrink-0 text-[11px]" />
      <span class="min-w-0 flex-1 truncate text-[11px] font-semibold" :title="filePath">
        {{ fileName || t("workspaces.sidebar.ideEmpty") }}
      </span>
    </div>

    <div v-if="!filePath" class="flex flex-1 items-center justify-center p-4 text-center text-xs text-sidebar-text">
      {{ t("workspaces.sidebar.ideHint") }}
    </div>
    <div v-else-if="loading" class="flex flex-1 items-center justify-center gap-2 text-xs text-sidebar-text">
      <i class="pi pi-spinner pi-spin" /> {{ t("common.loading") }}
    </div>
    <div v-else-if="error" class="flex-1 overflow-auto p-3 text-xs text-red-500">{{ error }}</div>
    <div v-else class="ide-code flex min-h-0 flex-1 overflow-auto">
      <div class="ide-gutter shrink-0 select-none px-2 py-2 text-right">
        <div v-for="n in lineCount" :key="n">{{ n }}</div>
      </div>
      <pre class="ide-pre m-0 flex-1 px-2 py-2"><code v-html="highlightedHtml" /></pre>
    </div>
  </div>
</template>

<style>
.ide-code {
  font-family: "JetBrains Mono", "Fira Code", "Cascadia Code", Consolas, monospace;
  font-size: 12px;
  line-height: 1.6;
  background: #0b0f19;
  color: #e5e9f0;
}
.ide-gutter {
  position: sticky;
  left: 0;
  color: #6b7280;
  border-right: 1px solid rgb(255 255 255 / 0.08);
  background: #0b0f19;
}
.ide-pre {
  white-space: pre;
}

/* Tô màu cú pháp — dùng lại tông màu dark của SQL editor (--sql-kw/str/com/num) để đồng bộ. */
.ide-pre .hljs-keyword,
.ide-pre .hljs-selector-tag,
.ide-pre .hljs-literal {
  color: #93c5fd;
}
.ide-pre .hljs-string,
.ide-pre .hljs-regexp {
  color: #86efac;
}
.ide-pre .hljs-comment,
.ide-pre .hljs-quote {
  color: #9ca3af;
  font-style: italic;
}
.ide-pre .hljs-number {
  color: #fcd34d;
}
.ide-pre .hljs-title,
.ide-pre .hljs-title.function_,
.ide-pre .hljs-section {
  color: #c4b5fd;
}
.ide-pre .hljs-type,
.ide-pre .hljs-class .hljs-title,
.ide-pre .hljs-tag,
.ide-pre .hljs-name {
  color: #f9a8d4;
}
.ide-pre .hljs-attr,
.ide-pre .hljs-attribute,
.ide-pre .hljs-property {
  color: #93c5fd;
}
.ide-pre .hljs-built_in,
.ide-pre .hljs-variable {
  color: #fca5a5;
}
</style>
