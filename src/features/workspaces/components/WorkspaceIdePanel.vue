<script setup lang="ts">
import { onBeforeUnmount, ref } from "vue";
import FileTreePanel from "@/shared/components/FileTreePanel.vue";
import WorkspaceIdeViewerPanel from "./WorkspaceIdeViewerPanel.vue";
import { explorerOpenFile } from "@/tauri/commands/explorer";
import type { FileEntry } from "@/tauri/commands/explorer";

defineProps<{ root: string }>();

/** Các extension xem được ngay trong panel (code/text) — còn lại mở bằng app mặc định của OS. */
const VIEWABLE_EXTENSIONS = new Set([
  "ts", "tsx", "js", "jsx", "mjs", "cjs", "vue", "svelte", "html", "htm",
  "css", "scss", "sass", "less", "json", "yaml", "yml", "toml", "xml",
  "md", "mdx", "txt", "csv", "rs", "go", "py", "rb", "java", "kt", "swift",
  "dart", "c", "cpp", "h", "hpp", "cs", "php", "sh", "bash", "zsh", "ps1",
  "bat", "cmd", "sql", "log", "env",
]);

// --- Tab: file đã mở, xem nhiều file cùng lúc (giống VSCode) ---
interface OpenTab {
  path: string;
  name: string;
}
const openTabs = ref<OpenTab[]>([]);
const activeTabPath = ref("");

function openInTab(entry: FileEntry) {
  if (!openTabs.value.some((tab) => tab.path === entry.path)) {
    openTabs.value.push({ path: entry.path, name: entry.name });
  }
  activeTabPath.value = entry.path;
}

function closeTab(path: string) {
  const idx = openTabs.value.findIndex((tab) => tab.path === path);
  if (idx === -1) return;
  openTabs.value.splice(idx, 1);
  if (activeTabPath.value === path) {
    const fallback = openTabs.value[idx] ?? openTabs.value[idx - 1];
    activeTabPath.value = fallback?.path ?? "";
  }
}

function onOpenFile(entry: FileEntry) {
  const ext = entry.extension?.toLowerCase() ?? "";
  if (VIEWABLE_EXTENSIONS.has(ext)) {
    openInTab(entry);
  } else {
    void explorerOpenFile(entry.path).catch(() => undefined);
  }
}

// --- Resize cột Explorer bằng cách kéo cạnh phải ---
const EXPLORER_WIDTH_KEY = "workspaces.idePanel.explorerWidth";
const MIN_WIDTH = 160;
const MAX_WIDTH = 420;

function loadExplorerWidth(): number {
  const raw = Number(localStorage.getItem(EXPLORER_WIDTH_KEY) ?? "");
  return Number.isFinite(raw) && raw > 0 ? Math.max(MIN_WIDTH, Math.min(MAX_WIDTH, raw)) : 240;
}

const explorerWidth = ref(loadExplorerWidth());
const isResizing = ref(false);
let activeMove: ((e: MouseEvent) => void) | null = null;

function startResize(e: MouseEvent) {
  e.preventDefault();
  isResizing.value = true;
  const startX = e.clientX;
  const startWidth = explorerWidth.value;
  const move = (ev: MouseEvent) => {
    explorerWidth.value = Math.max(MIN_WIDTH, Math.min(MAX_WIDTH, startWidth + (ev.clientX - startX)));
  };
  activeMove = move;
  document.addEventListener("mousemove", move);
  document.addEventListener("mouseup", endResize);
}

function endResize() {
  isResizing.value = false;
  if (activeMove) document.removeEventListener("mousemove", activeMove);
  document.removeEventListener("mouseup", endResize);
  activeMove = null;
  localStorage.setItem(EXPLORER_WIDTH_KEY, String(Math.round(explorerWidth.value)));
}

onBeforeUnmount(() => {
  if (activeMove) document.removeEventListener("mousemove", activeMove);
  document.removeEventListener("mouseup", endResize);
});
</script>

<template>
  <div class="flex h-full min-h-0 overflow-hidden" :class="isResizing ? 'select-none' : ''">
    <div class="shrink-0 overflow-hidden" :style="{ width: explorerWidth + 'px' }">
      <FileTreePanel :root="root" intercept-clicks class="h-full" @open-file="onOpenFile" />
    </div>

    <div
      class="flex w-1.5 shrink-0 cursor-col-resize items-center justify-center self-stretch hover:bg-sidebar-hover"
      :class="isResizing ? 'bg-sidebar-hover' : ''"
      @mousedown="startResize"
    >
      <div class="h-8 w-0.5 rounded-full bg-sidebar-border" :class="isResizing ? 'bg-brand' : ''" />
    </div>

    <div class="flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden">
      <!-- Tab bar (file đang mở) -->
      <div v-if="openTabs.length" class="flex shrink-0 items-center gap-0.5 overflow-x-auto bg-[#0b0f19] px-1 pt-1">
        <div
          v-for="tab in openTabs"
          :key="tab.path"
          class="group flex shrink-0 cursor-pointer items-center gap-1.5 rounded-t-md px-3 py-1.5 text-xs transition-colors"
          :class="tab.path === activeTabPath ? 'bg-[#151a29] text-[#e5e9f0]' : 'text-[#6b7280] hover:bg-[#11162280]'"
          :title="tab.path"
          @click="activeTabPath = tab.path"
        >
          <i class="pi pi-file shrink-0 text-[10px]" />
          <span class="max-w-[140px] truncate">{{ tab.name }}</span>
          <i
            class="pi pi-times shrink-0 text-[9px] opacity-0 transition-opacity group-hover:opacity-70 hover:!opacity-100"
            @click.stop="closeTab(tab.path)"
          />
        </div>
      </div>

      <WorkspaceIdeViewerPanel :file-path="activeTabPath" hide-header class="min-h-0 flex-1" />
    </div>
  </div>
</template>
