<script setup lang="ts">
import { computed } from "vue";
import "@xterm/xterm/css/xterm.css";
import { useWorkspaceTerminal } from "../composables/useWorkspaceTerminal";

const props = defineProps<{ workspaceId: number; startDir: string; title: string }>();

const wsTerm = useWorkspaceTerminal();
wsTerm.ensureTab(props.workspaceId, props.title, props.startDir);

const tabs = computed(() => wsTerm.tabsFor(props.workspaceId));
const activeKey = computed(() => wsTerm.activeKeyFor(props.workspaceId));

/** Gắn xterm vào container — nếu panel này bị unmount/remount (đóng/mở lại),
 * phiên PTY vẫn sống ở `useTerminal`'s module-scope state; `bindContainer` chỉ
 * "chuyển nhà" DOM container hiện có, không tạo phiên mới. */
function bind(key: string, el: Element | null) {
  if (el instanceof HTMLElement) void wsTerm.term.bindContainer(key, el);
}

/** Mở thêm một tab terminal song song trong cùng thư mục làm việc của workspace. */
function addTab() {
  wsTerm.addTab(props.workspaceId, `${props.title} ${tabs.value.length + 1}`, props.startDir);
}

function closeTab(key: string) {
  void wsTerm.closeTab(props.workspaceId, key);
}
</script>

<template>
  <div class="flex h-full w-full flex-col overflow-hidden bg-[#0b0f19]">
    <div v-if="tabs.length" class="flex shrink-0 items-center gap-1 border-b border-white/10 px-1 py-1 text-xs">
      <div
        v-for="tab in tabs"
        :key="tab.key"
        class="group flex h-6 shrink-0 cursor-pointer items-center gap-1.5 rounded px-2 transition-colors"
        :class="tab.key === activeKey ? 'bg-brand/20 text-white' : 'text-white/60 hover:bg-white/5'"
        @click="wsTerm.setActive(workspaceId, tab.key)"
      >
        <i class="pi pi-desktop text-[9px]" />
        <span>{{ tab.title }}</span>
        <span v-if="tab.exited" class="text-[9px] text-white/40">(exited)</span>
        <i
          class="pi pi-times text-[9px] opacity-0 transition-opacity group-hover:opacity-70 hover:!opacity-100"
          @click.stop="closeTab(tab.key)"
        />
      </div>
      <button
        type="button"
        class="flex h-6 shrink-0 items-center justify-center rounded px-1.5 text-white/60 transition-colors hover:bg-white/5 hover:text-brand"
        title="New Terminal"
        @click="addTab"
      >
        <i class="pi pi-plus text-[9px]" />
      </button>
    </div>

    <div class="relative min-h-0 flex-1">
      <template v-if="tabs.length">
        <div v-for="tab in tabs" v-show="tab.key === activeKey" :key="tab.key" class="absolute inset-0 p-1">
          <div class="h-full w-full" :ref="(el) => bind(tab.key, el as Element | null)" />
        </div>
      </template>
      <div v-else class="flex h-full flex-col items-center justify-center gap-2 text-xs text-white/40">
        <span>No terminal open.</span>
        <button
          type="button"
          class="flex h-7 items-center gap-1.5 rounded-md px-2 text-white/60 transition-colors hover:bg-white/5 hover:text-brand"
          @click="addTab"
        >
          <i class="pi pi-plus text-[10px]" />
          <span>New Terminal</span>
        </button>
      </div>
    </div>
  </div>
</template>
