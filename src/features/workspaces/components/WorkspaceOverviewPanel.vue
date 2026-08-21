<script setup lang="ts">
import { computed, reactive, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { useWorkspaceOverview } from "../composables/useWorkspaceOverview";
import { useWorkspaceTerminal } from "../composables/useWorkspaceTerminal";
import { TASK_CATEGORY_META, STEP_STATUS_META } from "@/models/task";
import type { Workspace } from "@/models/workspace";
import type { GitRepo } from "@/models/git";

const props = defineProps<{ workspace: Workspace; repo: GitRepo | null }>();
const { t } = useI18n();

const overview = useWorkspaceOverview(props.workspace);
const wsTerm = useWorkspaceTerminal();

watch(
  () => props.repo,
  (repo) => {
    if (repo) void overview.loadGitData(repo);
  },
  { immediate: true },
);

const collapsed = reactive({ tasks: false, plans: false, sessions: false });
const topRowCollapsed = computed(() => collapsed.tasks && collapsed.plans);

function toggleCollapse(key: keyof typeof collapsed): void {
  collapsed[key] = !collapsed[key];
}

function categoryBadgeClass(category: string): string {
  return TASK_CATEGORY_META[category as keyof typeof TASK_CATEGORY_META]?.badgeClass ?? "bg-canvas text-muted";
}

function stepStatusBadgeClass(status: string): string {
  return STEP_STATUS_META[status as keyof typeof STEP_STATUS_META]?.badgeClass ?? "bg-canvas text-muted";
}

// === Sessions: nhúng terminal sống trực tiếp trong lưới, thay cho panel Terminal riêng ===
type SessionsLayout = "list" | "grid2" | "grid3";
const sessionsLayout = ref<SessionsLayout>("grid2");
const sessionsGridClass = computed(() => {
  if (sessionsLayout.value === "list") return "flex flex-col gap-2";
  if (sessionsLayout.value === "grid3") return "grid grid-cols-3 gap-2 auto-rows-[22rem]";
  return "grid grid-cols-2 gap-2 auto-rows-[26rem]";
});

// Chỉ 1 tab được phóng to toàn màn hình tại một thời điểm. Tile phóng to KHÔNG đổi DOM
// container (không Teleport, không v-if lên phần tử chứa xterm) — chỉ đổi class sang
// `fixed` để chiếm toàn màn hình, tránh gọi lại `term.open()` vào container khác (việc
// "chuyển nhà" này từng khiến phiên hiện có bị mất trạng thái đang thao tác khi phóng to).
const maximizedKey = ref<string | null>(null);

function toggleMaximize(key: string): void {
  maximizedKey.value = maximizedKey.value === key ? null : key;
}

/** Gắn xterm vào container của 1 session. */
function bindSessionTerminal(key: string, el: Element | null): void {
  if (el instanceof HTMLElement) void wsTerm.term.bindContainer(key, el);
}

function openNewTerminal(): void {
  const title = `${props.workspace.name} ${overview.sessions.value.length + 1}`;
  wsTerm.addTab(props.workspace.id, title, props.workspace.project_path);
}

function closeSessionTab(key: string): void {
  if (maximizedKey.value === key) maximizedKey.value = null;
  void wsTerm.closeTab(props.workspace.id, key);
}
</script>

<template>
  <div class="flex h-full min-h-0 flex-col gap-3 p-3">
    <!-- Hàng 1: Tasks | Plans -->
    <div :class="['flex min-h-0 gap-3', topRowCollapsed ? 'flex-none' : 'flex-1']">
      <!-- Tasks -->
      <section
        :class="[
          'flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden rounded-lg border border-divider bg-panel shadow-sm',
          collapsed.tasks ? 'self-start' : '',
        ]"
      >
        <div class="flex shrink-0 items-center justify-between gap-2 border-b border-divider px-4 py-2">
          <h3 class="section-title">{{ t("workspaces.overview.tasks.title") }}</h3>
          <div class="flex items-center gap-2">
            <span class="text-xs text-muted">{{ overview.linkedTasks.value.length }}</span>
            <button
              type="button"
              class="rounded p-0.5 text-muted transition-colors hover:bg-canvas hover:text-ink"
              :title="collapsed.tasks ? t('workspaces.overview.expand') : t('workspaces.overview.collapse')"
              @click="toggleCollapse('tasks')"
            >
              <svg viewBox="0 0 20 20" fill="none" class="h-3.5 w-3.5 transition-transform" :class="collapsed.tasks ? '-rotate-90' : ''">
                <path d="M5 7.5L10 12.5L15 7.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
              </svg>
            </button>
          </div>
        </div>
        <div v-show="!collapsed.tasks" class="flex min-h-0 flex-1 flex-col gap-1.5 overflow-y-auto p-3">
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

      <!-- Plans -->
      <section
        :class="[
          'flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden rounded-lg border border-divider bg-panel shadow-sm',
          collapsed.plans ? 'self-start' : '',
        ]"
      >
        <div class="flex shrink-0 items-center justify-between gap-2 border-b border-divider px-4 py-2">
          <h3 class="section-title">{{ t("workspaces.overview.plans.title") }}</h3>
          <div class="flex items-center gap-2">
            <span class="text-xs text-muted">{{ overview.plans.value.length }}</span>
            <button
              type="button"
              class="rounded p-0.5 text-muted transition-colors hover:bg-canvas hover:text-ink"
              :title="collapsed.plans ? t('workspaces.overview.expand') : t('workspaces.overview.collapse')"
              @click="toggleCollapse('plans')"
            >
              <svg viewBox="0 0 20 20" fill="none" class="h-3.5 w-3.5 transition-transform" :class="collapsed.plans ? '-rotate-90' : ''">
                <path d="M5 7.5L10 12.5L15 7.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
              </svg>
            </button>
          </div>
        </div>
        <div v-show="!collapsed.plans" class="flex min-h-0 flex-1 flex-col gap-2 overflow-y-auto p-3">
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
    </div>

    <!-- Hàng 2: Sessions (chiếm cả 2 cột) -->
    <section
      :class="[
        'flex flex-col overflow-hidden rounded-lg border border-divider bg-panel shadow-sm',
        collapsed.sessions ? 'flex-none' : 'min-h-0 flex-1',
      ]"
    >
      <div class="flex shrink-0 flex-wrap items-center justify-between gap-2 border-b border-divider px-4 py-2">
        <h3 class="section-title">{{ t("workspaces.overview.sessions.title") }}</h3>
        <div class="flex items-center gap-2">
          <span class="text-xs text-muted">{{ overview.sessions.value.length }}</span>

          <div class="flex items-center gap-0.5 rounded-md border border-divider p-0.5">
            <button
              type="button"
              class="flex h-5 w-5 items-center justify-center rounded text-[11px] transition-colors"
              :class="sessionsLayout === 'list' ? 'bg-brand/10 text-brand' : 'text-muted hover:text-ink'"
              :title="t('workspaces.overview.sessions.layoutList')"
              @click="sessionsLayout = 'list'"
            >
              <i class="pi pi-list text-[11px]" />
            </button>
            <button
              type="button"
              class="flex h-5 w-5 items-center justify-center rounded text-[11px] font-semibold transition-colors"
              :class="sessionsLayout === 'grid2' ? 'bg-brand/10 text-brand' : 'text-muted hover:text-ink'"
              :title="t('workspaces.overview.sessions.layoutGrid2')"
              @click="sessionsLayout = 'grid2'"
            >
              2
            </button>
            <button
              type="button"
              class="flex h-5 w-5 items-center justify-center rounded text-[11px] font-semibold transition-colors"
              :class="sessionsLayout === 'grid3' ? 'bg-brand/10 text-brand' : 'text-muted hover:text-ink'"
              :title="t('workspaces.overview.sessions.layoutGrid3')"
              @click="sessionsLayout = 'grid3'"
            >
              3
            </button>
          </div>

          <button
            type="button"
            class="flex items-center gap-1 rounded-md border border-divider px-2 py-1 text-[11px] text-secondary transition-colors hover:bg-canvas hover:text-ink"
            :title="t('workspaces.overview.sessions.newTerminal')"
            @click="openNewTerminal"
          >
            <i class="pi pi-plus text-[10px]" />
            <span>{{ t("workspaces.overview.sessions.newTerminal") }}</span>
          </button>

          <button
            type="button"
            class="rounded p-0.5 text-muted transition-colors hover:bg-canvas hover:text-ink"
            :title="collapsed.sessions ? t('workspaces.overview.expand') : t('workspaces.overview.collapse')"
            @click="toggleCollapse('sessions')"
          >
            <svg viewBox="0 0 20 20" fill="none" class="h-3.5 w-3.5 transition-transform" :class="collapsed.sessions ? '-rotate-90' : ''">
              <path d="M5 7.5L10 12.5L15 7.5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" />
            </svg>
          </button>
        </div>
      </div>

      <div v-show="!collapsed.sessions" class="min-h-0 flex-1 overflow-y-auto p-3">
        <div v-if="!overview.sessions.value.length" class="flex h-full flex-col items-center justify-center gap-2 text-xs text-muted">
          <span>{{ t("workspaces.overview.sessions.empty") }}</span>
          <button
            type="button"
            class="flex items-center gap-1.5 rounded-md border border-divider px-3 py-1.5 text-xs text-secondary transition-colors hover:bg-canvas hover:text-ink"
            @click="openNewTerminal"
          >
            <i class="pi pi-plus text-[10px]" />
            <span>{{ t("workspaces.overview.sessions.newTerminal") }}</span>
          </button>
        </div>

        <div v-else :class="sessionsGridClass">
          <!-- Nền mờ phía sau khi có 1 session đang phóng to — dùng z-index thấp hơn tile
               đang phóng to (z-[60]) một chút, tự nó không đụng tới DOM của xterm. -->
          <div v-if="maximizedKey" class="fixed inset-0 z-[55] bg-black/70" @click="toggleMaximize(maximizedKey)" />

          <!-- Mỗi tab CHỈ có đúng 1 container DOM (không v-if/Teleport) trong suốt vòng đời —
               "phóng to" chỉ đổi class sang `fixed` để chiếm toàn màn hình, không bao giờ gọi
               lại `term.open()` vào 1 phần tử khác (tránh mất trạng thái đang thao tác của
               phiên khi chuyển qua lại giữa tile nhỏ và toàn màn hình). -->
          <div
            v-for="tab in overview.sessions.value"
            :key="tab.key"
            :class="[
              'flex flex-col overflow-hidden rounded-md border border-divider bg-[#0b0f19]',
              maximizedKey === tab.key
                ? 'fixed inset-6 z-[60] shadow-float'
                : sessionsLayout === 'list'
                  ? 'h-[28rem] w-full shrink-0'
                  : 'h-full w-full',
            ]"
          >
            <div class="flex shrink-0 items-center gap-1.5 border-b border-white/10 px-2 py-1 text-[11px]">
              <span :class="['h-1.5 w-1.5 shrink-0 rounded-full', tab.exited ? 'bg-white/30' : 'bg-emerald-500']" />
              <span class="min-w-0 flex-1 truncate text-white/80">{{ tab.title }}</span>
              <button
                type="button"
                class="flex h-5 w-5 shrink-0 items-center justify-center rounded text-white/50 transition-colors hover:bg-white/10 hover:text-white"
                :title="maximizedKey === tab.key ? t('workspaces.overview.sessions.restore') : t('workspaces.overview.sessions.maximize')"
                @click="toggleMaximize(tab.key)"
              >
                <i :class="maximizedKey === tab.key ? 'pi pi-times' : 'pi pi-expand'" class="text-[10px]" />
              </button>
              <button
                v-if="maximizedKey !== tab.key"
                type="button"
                class="flex h-5 w-5 shrink-0 items-center justify-center rounded text-white/50 transition-colors hover:bg-white/10 hover:text-white"
                :title="t('workspaces.overview.sessions.close')"
                @click="closeSessionTab(tab.key)"
              >
                <i class="pi pi-times text-[10px]" />
              </button>
            </div>
            <div class="min-h-0 flex-1 overflow-hidden p-1" :ref="(el) => bindSessionTerminal(tab.key, el as Element | null)" />
          </div>
        </div>
      </div>
    </section>
  </div>
</template>
