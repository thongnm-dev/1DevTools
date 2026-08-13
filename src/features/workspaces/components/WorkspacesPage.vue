<script setup lang="ts">
import { onMounted, onUnmounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import Button from "primevue/button";
import Tab from "primevue/tab";
import TabList from "primevue/tablist";
import Tabs from "primevue/tabs";
import WorkspaceMainArea from "./WorkspaceMainArea.vue";
import WorkspaceEditDialog from "./WorkspaceEditDialog.vue";
import WorkflowAutoRunPreviewDialog from "./WorkflowAutoRunPreviewDialog.vue";
import { useWorkspace } from "../composables/useWorkspace";
import { useWorkflow } from "../composables/useWorkflow";
import { useWorkflowRunner } from "../composables/useWorkflowRunner";
import { useWorkspaceTerminal } from "../composables/useWorkspaceTerminal";
import { onGitRepoChanged } from "@/tauri/events";
import type { UnlistenFn } from "@tauri-apps/api/event";
import type { GitRepo } from "@/models/git";
import type { Workspace } from "@/models/workspace";
import type { Workflow } from "@/models/workflow";

const { t } = useI18n();
const ctrl = useWorkspace();
const workflowCtrl = useWorkflow();
const runner = useWorkflowRunner();
const workspaceTerminal = useWorkspaceTerminal();

// --- New/Edit workspace dialog (shared fields, only one open at a time) ---
const showWorkspaceDialog = ref(false);
const editingWorkspace = ref<Workspace | null>(null);

function openNewWorkspaceDialog() {
  editingWorkspace.value = null;
  showWorkspaceDialog.value = true;
}

function openEditWorkspaceDialog(ws: Workspace) {
  editingWorkspace.value = ws;
  showWorkspaceDialog.value = true;
}

function onTabChange(value: string | number) {
  ctrl.selectWorkspace(Number(value));
}

// --- Auto-trigger: chạy workflow tự động khi file trong workspace thay đổi ---
let unlistenGitRepoChanged: UnlistenFn | null = null;

onMounted(async () => {
  unlistenGitRepoChanged = await onGitRepoChanged((path) => {
    for (const ws of ctrl.workspaces.value) {
      if (ws.project_path !== path || ws.auto_workflow_id === null) continue;
      const workflow = workflowCtrl.workflows.value.find((w) => w.id === ws.auto_workflow_id);
      if (!workflow) continue;
      void runner.runWorkflow(workflow, ws);
    }
  });
});

onUnmounted(() => {
  unlistenGitRepoChanged?.();
});

// --- Xem nhanh workflow tự động (diagram read-only, giống canvas ở WorkflowPage.vue) ---
const showWorkflowPreview = ref(false);
const previewWorkflow = ref<Workflow | null>(null);

function autoWorkflowFor(ws: Workspace): Workflow | null {
  if (ws.auto_workflow_id === null) return null;
  return workflowCtrl.workflows.value.find((w) => w.id === ws.auto_workflow_id) ?? null;
}

function openAutoWorkflowPreview(ws: Workspace) {
  const wf = autoWorkflowFor(ws);
  if (!wf) return;
  previewWorkflow.value = wf;
  showWorkflowPreview.value = true;
}

// --- Quick actions (fire-and-forget helpers to existing tools) ---
function repoFor(ws: Workspace): GitRepo | null {
  return ctrl.gitRepos.value.find((r) => r.path === ws.project_path) ?? null;
}

/** Đóng workspace — dọn theo cả phiên terminal riêng của nó ở sidebar (nếu có). */
async function closeWorkspace(ws: Workspace) {
  await workspaceTerminal.closeTabFor(ws.id);
  await ctrl.removeWorkspace(ws.id);
}
</script>

<template>
  <div class="flex flex-1 flex-col gap-3 overflow-hidden">
    <!-- Tab bar -->
    <div class="flex shrink-0 items-center gap-1 rounded-lg border border-divider bg-panel p-1.5 shadow-sm">
      <Tabs :value="ctrl.activeId.value ?? -1" scrollable class="min-w-0 flex-1" @update:value="onTabChange">
        <TabList>
          <Tab v-for="ws in ctrl.workspaces.value" :key="ws.id" :value="ws.id" as="div" class="cursor-pointer">
            <span class="flex items-center gap-2">
              <i :class="ws.icon" />
              <span class="max-w-[140px] truncate text-xs font-bold">{{ ws.name }}</span>
              <Button
                icon="pi pi-times"
                text
                rounded
                size="small"
                class="!h-5 !w-5"
                :title="t('workspaces.close')"
                @click.stop="closeWorkspace(ws)"
              />
            </span>
          </Tab>
        </TabList>
      </Tabs>
      <Button icon="pi pi-plus" text rounded size="small" :title="t('workspaces.newWorkspace')" @click="openNewWorkspaceDialog" />
    </div>

    <!-- Empty state -->
    <div v-if="ctrl.workspaces.value.length === 0" class="flex flex-1 items-center justify-center rounded-lg border border-dashed border-divider bg-panel/50 p-12">
      <div class="text-center">
        <i class="pi pi-th-large text-4xl text-muted/60" />
        <p class="mt-2 text-sm text-muted">{{ t("workspaces.emptyState") }}</p>
        <Button icon="pi pi-plus" :label="t('workspaces.newWorkspace')" class="mt-4" @click="openNewWorkspaceDialog" />
      </div>
    </div>

    <!-- One block per open workspace, kept mounted via v-show (not v-if) so each
         workspace's Git panel — and its file watcher — keeps running in the
         background when you switch to another tab, instead of being torn down
         and reloaded every time. -->
    <template v-else>
      <template v-for="ws in ctrl.workspaces.value" :key="ws.id">
        <div v-show="ws.id === ctrl.activeId.value" class="flex min-h-0 flex-1 flex-col gap-3 overflow-hidden">
          <div class="shrink-0 rounded-lg border border-divider bg-panel p-4 shadow-sm">
            <div class="flex flex-wrap items-center gap-3">
              <i :class="[ws.icon, 'text-xl text-muted']" />
              <div class="min-w-0">
                <div class="flex items-center gap-2">
                  <h2 class="section-title">{{ ws.name }}</h2>
                  <!-- Nút branch được WorkspaceGitPanel.vue teleport vào đây -->
                  <span :id="`ws-branch-slot-${ws.id}`" class="inline-flex" />
                  <button
                    v-if="autoWorkflowFor(ws)"
                    class="flex items-center gap-1 rounded-full bg-amber-500/10 px-2 py-0.5 text-[11px] font-bold text-amber-600 transition-colors hover:bg-amber-500/20"
                    :title="t('workspaces.dialog.autoWorkflowActive')"
                    @click="openAutoWorkflowPreview(ws)"
                  >
                    <i class="pi pi-bolt" />
                    {{ autoWorkflowFor(ws)!.name }}
                  </button>
                </div>
              </div>
              <div class="ml-auto flex shrink-0 items-center gap-1">
                <Button icon="pi pi-pencil" text rounded size="small" :title="t('workspaces.edit')" @click="openEditWorkspaceDialog(ws)" />
                <Button icon="pi pi-times" text rounded size="small" severity="danger" :title="t('workspaces.close')" @click="closeWorkspace(ws)" />
              </div>
            </div>
          </div>

          <WorkspaceMainArea :workspace="ws" :repo="repoFor(ws)" />
        </div>
      </template>
    </template>

    <WorkspaceEditDialog
      v-model:visible="showWorkspaceDialog"
      :workspace-ctrl="ctrl"
      :workflow-ctrl="workflowCtrl"
      :editing-workspace="editingWorkspace"
    />

    <WorkflowAutoRunPreviewDialog v-model:visible="showWorkflowPreview" :workflow="previewWorkflow" />
  </div>
</template>
