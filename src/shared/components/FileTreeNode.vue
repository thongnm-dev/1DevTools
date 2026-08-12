<script setup lang="ts">
import { computed, inject, ref } from "vue";
import { explorerReadDir, explorerOpenFile, type FileEntry } from "@/tauri/commands/explorer";
import { friendlyError } from "@/tauri/commands/_base";

const EXT_ICONS: Record<string, { icon: string; color: string }> = {
  ts: { icon: "pi-code", color: "text-blue-500" },
  tsx: { icon: "pi-code", color: "text-blue-500" },
  js: { icon: "pi-code", color: "text-yellow-500" },
  jsx: { icon: "pi-code", color: "text-yellow-500" },
  mjs: { icon: "pi-code", color: "text-yellow-500" },
  cjs: { icon: "pi-code", color: "text-yellow-500" },
  vue: { icon: "pi-code", color: "text-green-500" },
  svelte: { icon: "pi-code", color: "text-orange-500" },
  html: { icon: "pi-code", color: "text-orange-600" },
  htm: { icon: "pi-code", color: "text-orange-600" },
  css: { icon: "pi-palette", color: "text-blue-400" },
  scss: { icon: "pi-palette", color: "text-pink-500" },
  sass: { icon: "pi-palette", color: "text-pink-500" },
  less: { icon: "pi-palette", color: "text-indigo-400" },
  json: { icon: "pi-brackets-curly", color: "text-yellow-600" },
  yaml: { icon: "pi-list", color: "text-red-400" },
  yml: { icon: "pi-list", color: "text-red-400" },
  toml: { icon: "pi-list", color: "text-gray-500" },
  xml: { icon: "pi-code", color: "text-orange-400" },
  md: { icon: "pi-file-edit", color: "text-sky-500" },
  mdx: { icon: "pi-file-edit", color: "text-sky-500" },
  txt: { icon: "pi-file", color: "text-gray-400" },
  csv: { icon: "pi-table", color: "text-green-600" },
  rs: { icon: "pi-cog", color: "text-amber-700" },
  go: { icon: "pi-bolt", color: "text-cyan-500" },
  py: { icon: "pi-code", color: "text-yellow-500" },
  rb: { icon: "pi-code", color: "text-red-600" },
  java: { icon: "pi-code", color: "text-red-500" },
  kt: { icon: "pi-code", color: "text-purple-500" },
  swift: { icon: "pi-code", color: "text-orange-500" },
  dart: { icon: "pi-code", color: "text-sky-400" },
  c: { icon: "pi-code", color: "text-blue-700" },
  cpp: { icon: "pi-code", color: "text-blue-700" },
  h: { icon: "pi-code", color: "text-blue-600" },
  hpp: { icon: "pi-code", color: "text-blue-600" },
  cs: { icon: "pi-code", color: "text-purple-600" },
  php: { icon: "pi-code", color: "text-indigo-500" },
  sh: { icon: "pi-hashtag", color: "text-green-700" },
  bash: { icon: "pi-hashtag", color: "text-green-700" },
  zsh: { icon: "pi-hashtag", color: "text-green-700" },
  ps1: { icon: "pi-hashtag", color: "text-blue-600" },
  bat: { icon: "pi-hashtag", color: "text-gray-500" },
  cmd: { icon: "pi-hashtag", color: "text-gray-500" },
  sql: { icon: "pi-database", color: "text-blue-500" },
  db: { icon: "pi-database", color: "text-gray-500" },
  sqlite: { icon: "pi-database", color: "text-blue-400" },
  png: { icon: "pi-image", color: "text-purple-500" },
  jpg: { icon: "pi-image", color: "text-purple-500" },
  jpeg: { icon: "pi-image", color: "text-purple-500" },
  gif: { icon: "pi-image", color: "text-purple-500" },
  svg: { icon: "pi-image", color: "text-orange-500" },
  ico: { icon: "pi-image", color: "text-yellow-500" },
  webp: { icon: "pi-image", color: "text-purple-400" },
  bmp: { icon: "pi-image", color: "text-purple-400" },
  mp4: { icon: "pi-video", color: "text-pink-500" },
  mov: { icon: "pi-video", color: "text-pink-500" },
  avi: { icon: "pi-video", color: "text-pink-500" },
  webm: { icon: "pi-video", color: "text-pink-500" },
  mp3: { icon: "pi-volume-up", color: "text-pink-400" },
  wav: { icon: "pi-volume-up", color: "text-pink-400" },
  ogg: { icon: "pi-volume-up", color: "text-pink-400" },
  flac: { icon: "pi-volume-up", color: "text-pink-400" },
  pdf: { icon: "pi-file-pdf", color: "text-red-600" },
  doc: { icon: "pi-file-word", color: "text-blue-600" },
  docx: { icon: "pi-file-word", color: "text-blue-600" },
  xls: { icon: "pi-file-excel", color: "text-green-600" },
  xlsx: { icon: "pi-file-excel", color: "text-green-600" },
  ppt: { icon: "pi-file", color: "text-orange-600" },
  pptx: { icon: "pi-file", color: "text-orange-600" },
  zip: { icon: "pi-box", color: "text-yellow-700" },
  tar: { icon: "pi-box", color: "text-yellow-700" },
  gz: { icon: "pi-box", color: "text-yellow-700" },
  rar: { icon: "pi-box", color: "text-yellow-700" },
  "7z": { icon: "pi-box", color: "text-yellow-700" },
  lock: { icon: "pi-lock", color: "text-gray-400" },
  env: { icon: "pi-shield", color: "text-yellow-600" },
  log: { icon: "pi-list", color: "text-gray-400" },
  woff: { icon: "pi-at", color: "text-gray-500" },
  woff2: { icon: "pi-at", color: "text-gray-500" },
  ttf: { icon: "pi-at", color: "text-gray-500" },
  otf: { icon: "pi-at", color: "text-gray-500" },
  eot: { icon: "pi-at", color: "text-gray-500" },
};

const NAME_ICONS: Record<string, { icon: string; color: string }> = {
  dockerfile: { icon: "pi-box", color: "text-blue-500" },
  "docker-compose.yml": { icon: "pi-box", color: "text-blue-500" },
  "docker-compose.yaml": { icon: "pi-box", color: "text-blue-500" },
  ".gitignore": { icon: "pi-github", color: "text-gray-500" },
  ".gitattributes": { icon: "pi-github", color: "text-gray-500" },
  ".editorconfig": { icon: "pi-sliders-h", color: "text-gray-500" },
  ".prettierrc": { icon: "pi-sliders-h", color: "text-purple-400" },
  ".eslintrc": { icon: "pi-sliders-h", color: "text-purple-500" },
  "makefile": { icon: "pi-wrench", color: "text-gray-600" },
  "cargo.toml": { icon: "pi-cog", color: "text-amber-700" },
  "package.json": { icon: "pi-box", color: "text-green-600" },
  "tsconfig.json": { icon: "pi-cog", color: "text-blue-500" },
};

const DEFAULT_ICON = { icon: "pi-file", color: "text-sidebar-text" };

function fileIcon(entry: FileEntry) {
  const nameMatch = NAME_ICONS[entry.name.toLowerCase()];
  if (nameMatch) return nameMatch;
  const ext = entry.extension?.toLowerCase();
  if (ext) return EXT_ICONS[ext] ?? DEFAULT_ICON;
  return DEFAULT_ICON;
}

const props = defineProps<{ entry: FileEntry; depth: number }>();

const onFileClick = inject<((entry: FileEntry) => void) | undefined>("onFileClick", undefined);

const icon = computed(() => fileIcon(props.entry));

const expanded = ref(false);
const loaded = ref(false);
const loading = ref(false);
const children = ref<FileEntry[]>([]);
const error = ref("");

async function toggle() {
  if (!props.entry.is_dir) {
    if (onFileClick) {
      onFileClick(props.entry);
    } else {
      void explorerOpenFile(props.entry.path).catch(() => undefined);
    }
    return;
  }
  expanded.value = !expanded.value;
  if (!expanded.value || loaded.value) return;

  loading.value = true;
  error.value = "";
  try {
    const result = await explorerReadDir(props.entry.path);
    children.value = result.entries;
    loaded.value = true;
  } catch (e) {
    error.value = friendlyError(e);
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <div>
    <button
      type="button"
      class="flex w-full items-center gap-1 rounded px-1 py-0.5 text-left text-xs text-sidebar-text hover:bg-sidebar-hover hover:text-sidebar-text-active"
      :style="{ paddingLeft: `${depth * 14 + 6}px` }"
      :title="entry.path"
      @click="toggle"
    >
      <i
        v-if="entry.is_dir"
        class="pi shrink-0 text-[9px] text-sidebar-text"
        :class="expanded ? 'pi-chevron-down' : 'pi-chevron-right'"
      />
      <span v-else class="inline-block w-[9px] shrink-0" />
      <i class="pi shrink-0 text-[11px]" :class="entry.is_dir ? 'pi-folder text-amber-500' : [icon.icon, icon.color]" />
      <span class="truncate">{{ entry.name }}</span>
    </button>

    <div v-if="entry.is_dir && expanded">
      <div v-if="loading" class="px-1 py-1 text-[11px] text-sidebar-text" :style="{ paddingLeft: `${(depth + 1) * 14 + 6}px` }">
        <i class="pi pi-spinner pi-spin" />
      </div>
      <div v-else-if="error" class="truncate px-1 py-1 text-xs text-red-500" :style="{ paddingLeft: `${(depth + 1) * 14 + 6}px` }" :title="error">
        {{ error }}
      </div>
      <div v-else-if="!children.length" class="px-1 py-1 text-[11px] text-sidebar-text" :style="{ paddingLeft: `${(depth + 1) * 14 + 6}px` }">
        (empty)
      </div>
      <FileTreeNode v-for="child in children" :key="child.path" :entry="child" :depth="depth + 1" />
    </div>
  </div>
</template>
