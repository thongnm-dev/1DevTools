<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import Button from "primevue/button";
import Tab from "primevue/tab";
import TabList from "primevue/tablist";
import Tabs from "primevue/tabs";
import WorkspaceMainArea from "./WorkspaceMainArea.vue";
import WorkspaceEditDialog from "./WorkspaceEditDialog.vue";
import { useWorkspace } from "../composables/useWorkspace";
import { useWorkspaceTerminal } from "../composables/useWorkspaceTerminal";
import type { GitRepo } from "@/models/git";
import type { Workspace } from "@/models/workspace";

const { t } = useI18n();
const ctrl = useWorkspace();
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

// --- Quick actions (fire-and-forget helpers to existing tools) ---
function repoFor(ws: Workspace): GitRepo | null {
  return ctrl.gitRepos.value.find((r) => r.path === ws.project_path) ?? null;
}

/** Đóng workspace — dọn theo cả phiên terminal riêng của nó ở sidebar (nếu có). */
async function closeWorkspace(ws: Workspace) {
  await workspaceTerminal.closeAllTabsFor(ws.id);
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
        <div v-show="ws.id === ctrl.activeId.value" class="flex min-h-0 flex-1 overflow-hidden">
          <WorkspaceMainArea
            :workspace="ws"
            :repo="repoFor(ws)"
            @edit="openEditWorkspaceDialog(ws)"
            @delete="closeWorkspace(ws)"
          />
        </div>
      </template>
    </template>

    <WorkspaceEditDialog
      v-model:visible="showWorkspaceDialog"
      :workspace-ctrl="ctrl"
      :editing-workspace="editingWorkspace"
    />
  </div>
</template>
