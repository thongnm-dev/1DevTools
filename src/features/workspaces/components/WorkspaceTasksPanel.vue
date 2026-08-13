<script setup lang="ts">
import { ref, computed } from "vue";
import { useI18n } from "vue-i18n";
import Select from "primevue/select";
import type { Workspace } from "@/models/workspace";
import { useWorkspaceTask } from "../composables/useWorkspaceTask";
import { useWorkflow } from "../composables/useWorkflow";
import { useWorkflowRunner } from "../composables/useWorkflowRunner";

const props = defineProps<{ workspace: Workspace }>();
const { t } = useI18n();

const taskCtrl = useWorkspaceTask(props.workspace.id);
const workflowCtrl = useWorkflow();
const runner = useWorkflowRunner();

const newTaskName = ref("");
const selectedWorkflowId = ref<number | null>(null);

const selectedWorkflow = computed(() =>
  workflowCtrl.workflows.value.find((w) => w.id === selectedWorkflowId.value) ?? null,
);

const workflowOptions = computed(() =>
  workflowCtrl.workflows.value.map((w) => ({ label: w.name, value: w.id })),
);

function submitAdd() {
  taskCtrl.addTask(newTaskName.value);
  newTaskName.value = "";
}

async function runForTask() {
  if (!selectedWorkflow.value) return;
  await runner.runWorkflow(selectedWorkflow.value, props.workspace);
}
</script>

<template>
  <div class="flex h-full flex-col overflow-hidden bg-sidebar text-sidebar-text">
    <!-- Header -->
    <div class="flex shrink-0 items-center border-b border-sidebar-border px-3 py-2">
      <span class="flex-1 truncate text-[11px] font-semibold uppercase tracking-wide">
        {{ t("workspaces.tasks.title") }}
      </span>
      <span class="text-[10px] text-sidebar-text/50">{{ taskCtrl.tasks.value.length }}</span>
    </div>

    <!-- Workflow selector -->
    <div class="shrink-0 border-b border-sidebar-border px-3 py-2">
      <Select
        v-model="selectedWorkflowId"
        :options="workflowOptions"
        option-label="label"
        option-value="value"
        :placeholder="t('workspaces.tasks.selectWorkflow')"
        :empty-message="t('workspaces.tasks.noWorkflows')"
        class="w-full !text-xs"
        size="small"
      />
    </div>

    <!-- Add task input -->
    <form class="shrink-0 border-b border-sidebar-border px-3 py-2" @submit.prevent="submitAdd">
      <div class="flex gap-1.5">
        <input
          v-model="newTaskName"
          type="text"
          class="min-w-0 flex-1 rounded-md border border-sidebar-border bg-transparent px-2 py-1 text-xs text-sidebar-text placeholder:text-sidebar-text/40 focus:border-brand focus:outline-none"
          :placeholder="t('workspaces.tasks.addPlaceholder')"
        />
        <button
          type="submit"
          :disabled="!newTaskName.trim()"
          class="flex items-center justify-center rounded-md bg-brand px-2 py-1 text-xs text-white transition-colors hover:brightness-110 disabled:opacity-40"
        >
          <i class="pi pi-plus text-[10px]" />
        </button>
      </div>
    </form>

    <!-- Task list -->
    <div class="min-h-0 flex-1 overflow-y-auto">
      <p v-if="!taskCtrl.tasks.value.length" class="px-3 py-6 text-center text-xs text-sidebar-text/50">
        {{ t("workspaces.tasks.empty") }}
      </p>

      <div
        v-for="task in taskCtrl.tasks.value"
        :key="task.id"
        class="group flex items-center gap-2 border-b border-sidebar-border/50 px-3 py-2 transition-colors hover:bg-sidebar-hover"
      >
        <!-- Done checkbox -->
        <input
          type="checkbox"
          :checked="task.done"
          class="shrink-0 cursor-pointer accent-brand"
          @change="taskCtrl.toggleDone(task.id)"
        />

        <!-- Task name -->
        <span
          class="min-w-0 flex-1 truncate text-xs"
          :class="task.done ? 'line-through text-sidebar-text/40' : 'text-sidebar-text'"
        >
          {{ task.name }}
        </span>

        <!-- Actions -->
        <div class="flex shrink-0 items-center gap-0.5 opacity-0 transition-opacity group-hover:opacity-100">
          <button
            v-if="selectedWorkflow"
            class="flex items-center justify-center rounded p-1 text-sidebar-text/60 transition-colors hover:bg-sidebar-border hover:text-brand"
            :title="t('workspaces.tasks.runFor', { workflow: selectedWorkflow.name })"
            @click="runForTask"
          >
            <i class="pi pi-play text-[10px]" />
          </button>
          <button
            class="flex items-center justify-center rounded p-1 text-sidebar-text/60 transition-colors hover:bg-sidebar-border hover:text-red-500"
            :title="t('workspaces.tasks.deleteTask')"
            @click="taskCtrl.removeTask(task.id)"
          >
            <i class="pi pi-trash text-[10px]" />
          </button>
        </div>
      </div>
    </div>

    <!-- Footer: Run all -->
    <div v-if="taskCtrl.tasks.value.length && selectedWorkflow" class="shrink-0 border-t border-sidebar-border px-3 py-2">
      <button
        class="flex w-full items-center justify-center gap-1.5 rounded-md bg-brand px-3 py-1.5 text-xs font-medium text-white transition-colors hover:brightness-110"
        @click="runForTask"
      >
        <i class="pi pi-play text-[10px]" />
        {{ t("workspaces.tasks.run") }} · {{ selectedWorkflow.name }}
      </button>
    </div>
  </div>
</template>
