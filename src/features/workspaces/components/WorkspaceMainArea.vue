<script setup lang="ts">
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import WorkspaceGitPanel from "./WorkspaceGitPanel.vue";
import WorkspaceRightSidebar from "./WorkspaceRightSidebar.vue";
import WorkspaceIdePanel from "./WorkspaceIdePanel.vue";
import WorkspaceAgentsPanel from "./WorkspaceAgentsPanel.vue";
import WorkspaceTerminalPanel from "./WorkspaceTerminalPanel.vue";
import WorkspaceTasksPanel from "./WorkspaceTasksPanel.vue";
import WorkspaceOverviewPanel from "./WorkspaceOverviewPanel.vue";
import type { GitRepo } from "@/models/git";
import type { Workspace, WorkspaceMainPanel } from "@/models/workspace";

const props = defineProps<{ workspace: Workspace; repo: GitRepo | null }>();
const emit = defineEmits<{
  edit: [];
  delete: [];
}>();
const { t } = useI18n();

// Panel đang hiển thị trong vùng nội dung chính — các panel giữ mounted qua
// v-show (không v-if) để git-watch/terminal/explorer không bị reset khi
// chuyển qua lại, đúng nguyên tắc "chạy nền thật sự" đã áp dụng cho cả app.
// Riêng terminal cần thêm v-if trễ (xem `terminalOpened` bên dưới) vì xterm.js
// không được mở vào container đang ẩn.
const activePanel = ref<WorkspaceMainPanel>("git");

// xterm.js đo kích thước ký tự (font metrics) tại thời điểm `open()` — nếu container
// đang ẩn (v-show display:none) lúc đó, phép đo trả về 0 và bị cache lại, khiến
// terminal chỉ hiện một vùng đen vĩnh viễn dù sau này container hiện ra và fit() lại.
// Do đó, panel terminal chỉ được mount (v-if) từ lần đầu người dùng thật sự mở tab
// "terminal" — container lúc này đã visible — sau đó mới chuyển sang v-show để giữ
// phiên PTY sống khi chuyển qua lại các panel khác.
const terminalOpened = ref(false);
watch(
  activePanel,
  (panel) => {
    if (panel === "terminal") terminalOpened.value = true;
  },
  { immediate: true },
);
</script>

<template>
  <div class="flex min-h-0 flex-1 overflow-hidden">
    <div class="min-h-0 flex-1 overflow-hidden rounded-lg border border-divider bg-panel shadow-sm">
      <div v-show="activePanel === 'git'" class="h-full">
        <WorkspaceGitPanel v-if="repo" :repo="repo" />
        <div v-else class="flex h-full items-center justify-center p-6 text-center text-xs text-muted">
          {{ t("workspaces.git.repoNotFound") }}
        </div>
      </div>

      <WorkspaceTerminalPanel
        v-if="terminalOpened"
        v-show="activePanel === 'terminal'"
        :workspace-id="workspace.id"
        :start-dir="workspace.project_path"
        :title="workspace.name"
        class="h-full"
      />

      <WorkspaceAgentsPanel v-show="activePanel === 'agents'" class="h-full" />

      <WorkspaceIdePanel v-show="activePanel === 'ide'" :root="workspace.project_path" class="h-full" />

      <WorkspaceTasksPanel v-show="activePanel === 'tasks'" :workspace="workspace" class="h-full" />

      <WorkspaceOverviewPanel v-show="activePanel === 'overview'" :workspace="workspace" :repo="repo" class="h-full" />
    </div>

    <WorkspaceRightSidebar
      :active-panel="activePanel"
      :workspace="workspace"
      @select="activePanel = $event"
      @edit="emit('edit')"
      @delete="emit('delete')"
    />
  </div>
</template>
