<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import WorkspaceGitPanel from "./WorkspaceGitPanel.vue";
import WorkspaceRightSidebar from "./WorkspaceRightSidebar.vue";
import WorkspaceIdePanel from "./WorkspaceIdePanel.vue";
import WorkspaceAgentsPanel from "./WorkspaceAgentsPanel.vue";
import WorkspaceTerminalPanel from "./WorkspaceTerminalPanel.vue";
import type { GitRepo } from "@/models/git";
import type { Workspace, WorkspaceMainPanel } from "@/models/workspace";

const props = defineProps<{ workspace: Workspace; repo: GitRepo | null }>();
const { t } = useI18n();

// Panel đang hiển thị trong vùng nội dung chính — mọi panel đều giữ mounted
// qua v-show (không v-if) để git-watch/terminal/explorer không bị reset khi
// chuyển qua lại, đúng nguyên tắc "chạy nền thật sự" đã áp dụng cho cả app.
const activePanel = ref<WorkspaceMainPanel>("git");
</script>

<template>
  <div class="flex min-h-0 flex-1 overflow-hidden">
    <div class="min-h-0 flex-1 overflow-hidden rounded-lg border border-divider bg-panel shadow-sm">
      <div v-show="activePanel === 'git'" class="h-full">
        <WorkspaceGitPanel v-if="repo" :repo="repo" :workspace-id="workspace.id" />
        <div v-else class="flex h-full items-center justify-center p-6 text-center text-xs text-muted">
          {{ t("workspaces.git.repoNotFound") }}
        </div>
      </div>

      <WorkspaceTerminalPanel
        v-show="activePanel === 'terminal'"
        :workspace-id="workspace.id"
        :start-dir="workspace.project_path"
        :title="workspace.name"
        class="h-full"
      />

      <WorkspaceAgentsPanel v-show="activePanel === 'agents'" class="h-full" />

      <WorkspaceIdePanel v-show="activePanel === 'ide'" :root="workspace.project_path" class="h-full" />
    </div>

    <WorkspaceRightSidebar :active-panel="activePanel" @select="activePanel = $event" />
  </div>
</template>
