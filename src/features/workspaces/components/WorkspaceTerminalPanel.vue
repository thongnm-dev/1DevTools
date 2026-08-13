<script setup lang="ts">
import { useWorkspaceTerminal } from "../composables/useWorkspaceTerminal";

const props = defineProps<{ workspaceId: number; startDir: string; title: string }>();

const { term, ensureTab } = useWorkspaceTerminal();
const tabKey = ensureTab(props.workspaceId, props.title, props.startDir);

/** Gắn xterm vào container — nếu panel này bị unmount/remount (đóng/mở lại),
 * phiên PTY vẫn sống ở `useTerminal`'s module-scope state; `bindContainer` chỉ
 * "chuyển nhà" DOM container hiện có, không tạo phiên mới. */
function bind(el: Element | null) {
  if (el instanceof HTMLElement && tabKey) void term.bindContainer(tabKey, el);
}
</script>

<template>
  <div class="h-full w-full bg-[#0b0f19]" :ref="(el) => bind(el as Element | null)" />
</template>
