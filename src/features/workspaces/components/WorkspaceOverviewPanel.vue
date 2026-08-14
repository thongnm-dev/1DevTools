<script setup lang="ts">
import { watch } from "vue";
import { useI18n } from "vue-i18n";
import { useWorkspaceOverview } from "../composables/useWorkspaceOverview";
import { TASK_CATEGORY_META, STEP_STATUS_META } from "@/models/task";
import { baseName, statusMeta } from "@/features/git/utils/fileStatus";
import type { Workspace } from "@/models/workspace";
import type { GitRepo } from "@/models/git";

const props = defineProps<{ workspace: Workspace; repo: GitRepo | null }>();
const { t } = useI18n();

const overview = useWorkspaceOverview(props.workspace);

watch(
  () => props.repo,
  (repo) => {
    if (repo) void overview.loadGitData(repo);
  },
  { immediate: true },
);

function categoryBadgeClass(category: string): string {
  return TASK_CATEGORY_META[category as keyof typeof TASK_CATEGORY_META]?.badgeClass ?? "bg-canvas text-muted";
}

function stepStatusBadgeClass(status: string): string {
  return STEP_STATUS_META[status as keyof typeof STEP_STATUS_META]?.badgeClass ?? "bg-canvas text-muted";
}

function formatAt(at: string): string {
  const date = new Date(at);
  return Number.isNaN(date.getTime()) ? at : date.toLocaleString();
}
</script>

<template>
  <div class="h-full overflow-y-auto p-4">
    <div class="grid grid-cols-1 gap-4 lg:grid-cols-2">
      <!-- Tasks -->
      <section class="flex min-h-0 flex-col overflow-hidden rounded-lg border border-divider bg-panel shadow-sm">
        <div class="flex items-center justify-between gap-2 border-b border-divider px-4 py-3">
          <h3 class="section-title">{{ t("workspaces.overview.tasks.title") }}</h3>
          <span class="text-xs text-muted">{{ overview.linkedTasks.value.length }}</span>
        </div>
        <div class="flex max-h-72 flex-col gap-1.5 overflow-y-auto p-3">
          <p v-if="!overview.linkedTasks.value.length" class="px-1 py-4 text-center text-xs text-muted">
            {{ t("workspaces.overview.tasks.empty") }}
          </p>
          <div
            v-for="task in overview.linkedTasks.value"
            :key="task.id"
            class="flex items-center gap-2 rounded-md border border-divider bg-canvas px-3 py-2"
          >
            <span :class="['shrink-0 rounded-full px-2 py-0.5 text-[10px] font-bold', categoryBadgeClass(task.category_id)]">
              {{ task.category_id || "—" }}
            </span>
            <span class="font-mono text-xs text-ink">{{ task.task_cd }}</span>
            <span class="min-w-0 flex-1 truncate text-xs text-secondary">{{ task.task_name }}</span>
            <span
              v-if="task.current_step_status"
              :class="['shrink-0 rounded-full px-2 py-0.5 text-[10px] font-bold', stepStatusBadgeClass(task.current_step_status)]"
            >
              {{ task.current_step_status }}
            </span>
          </div>
        </div>
      </section>

      <!-- Sessions -->
      <section class="flex min-h-0 flex-col overflow-hidden rounded-lg border border-divider bg-panel shadow-sm">
        <div class="flex items-center justify-between gap-2 border-b border-divider px-4 py-3">
          <h3 class="section-title">{{ t("workspaces.overview.sessions.title") }}</h3>
          <span class="text-xs text-muted">{{ overview.sessions.value.length }}</span>
        </div>
        <div class="flex max-h-72 flex-col gap-1.5 overflow-y-auto p-3">
          <p v-if="!overview.sessions.value.length" class="px-1 py-4 text-center text-xs text-muted">
            {{ t("workspaces.overview.sessions.empty") }}
          </p>
          <div
            v-for="tab in overview.sessions.value"
            :key="tab.key"
            class="flex items-center gap-2 rounded-md border border-divider bg-canvas px-3 py-2"
          >
            <span :class="['h-2 w-2 shrink-0 rounded-full', tab.exited ? 'bg-muted' : 'bg-emerald-500']" />
            <span class="min-w-0 flex-1 truncate text-xs text-ink">{{ tab.title }}</span>
            <span class="shrink-0 text-[10px] text-muted">
              {{ tab.exited ? t("workspaces.overview.sessions.exited") : t("workspaces.overview.sessions.running") }}
            </span>
          </div>
        </div>
      </section>

      <!-- Plans -->
      <section class="flex min-h-0 flex-col overflow-hidden rounded-lg border border-divider bg-panel shadow-sm">
        <div class="flex items-center justify-between gap-2 border-b border-divider px-4 py-3">
          <h3 class="section-title">{{ t("workspaces.overview.plans.title") }}</h3>
          <span class="text-xs text-muted">{{ overview.plans.value.length }}</span>
        </div>
        <div class="flex max-h-72 flex-col gap-2 overflow-y-auto p-3">
          <p v-if="!overview.plans.value.length" class="px-1 py-4 text-center text-xs text-muted">
            {{ t("workspaces.overview.plans.empty") }}
          </p>
          <div v-for="plan in overview.plans.value" :key="plan.task.id" class="rounded-md border border-divider bg-canvas p-2.5">
            <div class="mb-1.5 flex items-center gap-2 text-xs">
              <span class="font-mono text-ink">{{ plan.task.task_cd }}</span>
              <span class="text-muted">{{ plan.workflowName }}</span>
            </div>
            <div class="flex flex-col gap-1">
              <div v-for="step in plan.steps" :key="step.id" class="flex items-center gap-2 text-[11px]">
                <span class="min-w-0 flex-1 truncate text-secondary">{{ step.order + 1 }}. {{ step.name }}</span>
                <span :class="['shrink-0 rounded-full px-1.5 py-0.5 text-[9px] font-bold', stepStatusBadgeClass(step.status)]">
                  {{ step.status }}
                </span>
              </div>
            </div>
          </div>
        </div>
      </section>

      <!-- Activity -->
      <section class="flex min-h-0 flex-col overflow-hidden rounded-lg border border-divider bg-panel shadow-sm">
        <div class="flex items-center justify-between gap-2 border-b border-divider px-4 py-3">
          <h3 class="section-title">{{ t("workspaces.overview.activity.title") }}</h3>
        </div>
        <div class="flex max-h-72 flex-col gap-1.5 overflow-y-auto p-3">
          <p v-if="!overview.activity.value.length" class="px-1 py-4 text-center text-xs text-muted">
            {{ t("workspaces.overview.activity.empty") }}
          </p>
          <div v-for="(entry, index) in overview.activity.value" :key="index" class="flex items-start gap-2 text-xs">
            <span class="mt-1 h-1.5 w-1.5 shrink-0 rounded-full bg-brand" />
            <div class="min-w-0 flex-1">
              <p class="truncate text-ink">{{ entry.text }}</p>
              <p class="text-[10px] text-muted">{{ formatAt(entry.at) }}</p>
            </div>
          </div>
        </div>
      </section>

      <!-- Files changed -->
      <section class="flex min-h-0 flex-col overflow-hidden rounded-lg border border-divider bg-panel shadow-sm lg:col-span-2">
        <div class="flex items-center justify-between gap-2 border-b border-divider px-4 py-3">
          <h3 class="section-title">{{ t("workspaces.overview.files.title") }}</h3>
          <span class="text-xs text-muted">{{ overview.files.value.length }}</span>
        </div>
        <p v-if="overview.gitError.value" class="banner-danger mx-4 mt-3">{{ overview.gitError.value }}</p>
        <div v-else-if="!props.repo" class="px-3 py-8 text-center text-xs text-muted">
          {{ t("workspaces.overview.files.notARepo") }}
        </div>
        <div v-else class="flex max-h-60 flex-col gap-1 overflow-y-auto p-3">
          <p v-if="!overview.files.value.length" class="px-1 py-4 text-center text-xs text-muted">
            {{ t("workspaces.overview.files.empty") }}
          </p>
          <div
            v-for="file in overview.files.value"
            :key="`${file.staged}-${file.path}`"
            class="flex items-center gap-2 rounded-md border border-divider bg-canvas px-3 py-1.5 text-xs"
          >
            <span :class="['shrink-0 rounded px-1.5 py-0.5 text-[10px] font-bold', statusMeta(file.status).badge]">
              {{ file.status }}
            </span>
            <span class="min-w-0 flex-1 truncate text-ink">{{ baseName(file.path) }}</span>
            <span class="shrink-0 text-[10px] text-muted">
              {{ file.staged ? t("workspaces.overview.files.staged") : t("workspaces.overview.files.unstaged") }}
            </span>
          </div>
        </div>
      </section>
    </div>
  </div>
</template>
