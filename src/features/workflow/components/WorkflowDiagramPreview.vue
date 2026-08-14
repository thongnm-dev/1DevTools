<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import type { NodePos, Workflow, WorkflowStep, WorkflowStepType } from "@/models/workflow";
import { STEP_TYPE_META } from "@/models/workflow";
import { workflowStepList } from "@/tauri/commands/workflow";

const props = defineProps<{ workflow: Workflow }>();
const { t } = useI18n();

// Cùng hằng số layout với canvas ở WorkflowPage.vue để diagram xem-trước này
// giữ đúng bố cục node đã lưu (hoặc auto-layout tương tự nếu chưa có layout).
const NODE_W = 208;
const NODE_H = 140;
const H_GAP = 120;
const V_GAP = 100;
const NODES_PER_ROW = 3;

const nodesContainer = ref<HTMLElement | null>(null);
const arrowPaths = ref<{ id: string; d: string }[]>([]);
const steps = ref<WorkflowStep[]>([]);
let resizeObserver: ResizeObserver | null = null;

async function loadSteps() {
  try {
    steps.value = await workflowStepList(props.workflow.id);
  } catch {
    steps.value = [];
  }
}

const nodePositions = computed<Record<string, NodePos>>(() => {
  const layout = props.workflow.layout;
  if (layout && Object.keys(layout).length > 0) return layout;
  const pos: Record<string, NodePos> = {};
  steps.value.forEach((step, i) => {
    const col = i % NODES_PER_ROW;
    const row = Math.floor(i / NODES_PER_ROW);
    pos[step.id] = { x: col * (NODE_W + H_GAP), y: row * (NODE_H + V_GAP) };
  });
  return pos;
});

const canvasSize = computed(() => {
  let maxX = 0;
  let maxY = 0;
  for (const pos of Object.values(nodePositions.value)) {
    maxX = Math.max(maxX, pos.x + NODE_W + 60);
    maxY = Math.max(maxY, pos.y + NODE_H + 60);
  }
  return { width: Math.max(maxX, 400), height: Math.max(maxY, 200) };
});

function updateArrows() {
  const container = nodesContainer.value;
  if (!container || steps.value.length < 2) {
    arrowPaths.value = [];
    return;
  }

  const containerRect = container.getBoundingClientRect();
  const paths: { id: string; d: string }[] = [];

  for (let i = 0; i < steps.value.length - 1; i++) {
    const from = steps.value[i];
    const to = steps.value[i + 1];
    const fromEl = container.querySelector(`[data-step-id="${from.id}"]`) as HTMLElement | null;
    const toEl = container.querySelector(`[data-step-id="${to.id}"]`) as HTMLElement | null;
    if (!fromEl || !toEl) continue;

    const fromRect = fromEl.getBoundingClientRect();
    const toRect = toEl.getBoundingClientRect();

    const x1 = fromRect.right - containerRect.left;
    const y1 = fromRect.top + fromRect.height / 2 - containerRect.top;
    const x2 = toRect.left - containerRect.left;
    const y2 = toRect.top + toRect.height / 2 - containerRect.top;

    const sameRow = Math.abs(y1 - y2) < 30;
    let d: string;
    if (sameRow) {
      const cx = (x1 + x2) / 2;
      d = `M ${x1} ${y1} C ${cx} ${y1}, ${cx} ${y2}, ${x2} ${y2}`;
    } else {
      d = `M ${x1} ${y1} C ${x1 + 50} ${y1}, ${x2 - 50} ${y2}, ${x2} ${y2}`;
    }
    paths.push({ id: `arrow-${from.id}-${to.id}`, d });
  }

  arrowPaths.value = paths;
}

watch(
  () => props.workflow.id,
  async () => {
    await loadSteps();
    await nextTick();
    updateArrows();
  },
);

onMounted(async () => {
  await loadSteps();
  void nextTick(() => updateArrows());
  if (nodesContainer.value) {
    resizeObserver = new ResizeObserver(() => updateArrows());
    resizeObserver.observe(nodesContainer.value);
  }
});

onBeforeUnmount(() => resizeObserver?.disconnect());

function stepTypeBadgeClass(type: WorkflowStepType): string {
  return STEP_TYPE_META[type]?.badgeClass ?? "bg-canvas text-muted";
}

function stepTypeLabel(type: WorkflowStepType): string {
  return t(`workflow.stepType.${type}`);
}
</script>

<template>
  <div class="relative h-full flex-1 overflow-auto rounded-lg border border-divider bg-canvas">
    <div ref="nodesContainer" class="relative" :style="{ minWidth: canvasSize.width + 'px', minHeight: canvasSize.height + 'px' }">
      <svg
        class="pointer-events-none absolute inset-0 overflow-visible"
        :style="{ width: canvasSize.width + 'px', height: canvasSize.height + 'px' }"
        style="z-index: 0"
      >
        <defs>
          <marker id="wf-preview-arrowhead" markerWidth="10" markerHeight="7" refX="10" refY="3.5" orient="auto">
            <polygon points="0 0, 10 3.5, 0 7" style="fill: rgb(var(--color-text-muted))" />
          </marker>
        </defs>
        <path
          v-for="arrow in arrowPaths"
          :key="arrow.id"
          :d="arrow.d"
          fill="none"
          stroke-width="2"
          stroke-linecap="round"
          marker-end="url(#wf-preview-arrowhead)"
          style="stroke: rgb(var(--color-text-muted))"
        />
      </svg>

      <div
        v-for="(step, index) in steps"
        :key="step.id"
        :data-step-id="step.id"
        class="absolute z-10 w-52 rounded-lg border border-divider bg-panel p-4 shadow-card"
        :style="{ left: (nodePositions[step.id]?.x ?? 0) + 'px', top: (nodePositions[step.id]?.y ?? 0) + 'px' }"
      >
        <div class="mb-2 flex items-center gap-2">
          <span class="flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-brand/10 text-xs font-bold text-brand">
            {{ index + 1 }}
          </span>
          <i :class="[step.icon, 'text-muted']" />
          <h4 class="section-title min-w-0 flex-1 break-words">{{ step.name }}</h4>
        </div>
        <div class="mt-1 flex flex-wrap items-center gap-1">
          <span :class="['inline-block rounded-full px-2 py-0.5 text-[11px] font-bold', stepTypeBadgeClass(step.step_type)]">
            {{ stepTypeLabel(step.step_type) }}
          </span>
          <span v-if="step.is_latest_step" class="badge-success" :title="t('workflow.step.isLatestHint')">
            <i class="pi pi-flag-fill text-[9px]" />{{ t("workflow.latest") }}
          </span>
        </div>
        <p v-if="step.description" class="mt-2 line-clamp-2 text-xs text-muted">{{ step.description }}</p>
      </div>
    </div>
  </div>
</template>
