<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import DataTable from "primevue/datatable";
import Column from "primevue/column";
import Fieldset from "primevue/fieldset";
import Button from "primevue/button";
import InputText from "primevue/inputtext";
import Select from "primevue/select";
import TaskDialog from "./TaskDialog.vue";
import { useTasks } from "../composables/useTasks";
import { useDataTablePagination } from "@/shared/composables/useDataTablePagination";
import { TASK_CATEGORY_META, STEP_STATUS_META } from "@/models/task";

const { t } = useI18n();
const ctrl = useTasks();
const { pagination } = useDataTablePagination();
const isDialogOpen = ref(false);

const completeOptions = [
  { label: t("aiTasks.search.allStatus"), value: null },
  { label: t("aiTasks.status.pending"), value: false },
  { label: t("aiTasks.status.complete"), value: true },
];

function openCreate() {
  ctrl.startCreate();
  isDialogOpen.value = true;
}

function openEdit(id: number) {
  ctrl.selectTask(id);
  isDialogOpen.value = true;
}

function categoryBadgeClass(category: string): string {
  return TASK_CATEGORY_META[category as keyof typeof TASK_CATEGORY_META]?.badgeClass ?? "bg-canvas text-muted";
}

function stepStatusBadgeClass(status: string): string {
  return STEP_STATUS_META[status as keyof typeof STEP_STATUS_META]?.badgeClass ?? "bg-canvas text-muted";
}

onMounted(() => ctrl.fetchTasks());
</script>

<template>
  <section class="flex min-h-0 flex-1 flex-col gap-4 overflow-hidden">
    <p v-if="ctrl.error.value" class="banner-danger">{{ ctrl.error.value }}</p>

    <section class="flex items-center justify-end rounded-lg border border-divider bg-panel p-4 shadow-sm">
      <Button icon="pi pi-plus" :label="t('aiTasks.actions.addTask')" size="small" @click="openCreate" />
    </section>

    <Fieldset class="rounded-lg border border-divider bg-panel p-4 shadow-md fieldset-nested" :legend="t('aiTasks.search.legend')" toggleable>
      <div class="grid gap-3">
        <div class="grid gap-3 lg:grid-cols-2">
          <label>
            <span class="text-xs font-bold text-muted">{{ t("aiTasks.search.keyword") }}</span>
            <InputText
              class="mt-1 w-full"
              :placeholder="t('aiTasks.search.keywordPlaceholder')"
              :model-value="ctrl.filters.value.keyword"
              @update:model-value="ctrl.filters.value.keyword = String($event)"
              @keyup.enter="ctrl.search()"
            />
          </label>
          <label>
            <span class="text-xs font-bold text-muted">{{ t("aiTasks.search.status") }}</span>
            <Select
              class="mt-1 w-full"
              :options="completeOptions"
              option-label="label"
              option-value="value"
              :model-value="ctrl.filters.value.isComplete"
              @update:model-value="ctrl.filters.value.isComplete = $event"
            />
          </label>
        </div>
        <div class="flex items-center justify-end gap-2">
          <Button icon="pi pi-refresh" :label="t('aiTasks.actions.reset')" severity="secondary" outlined size="small" @click="ctrl.resetFilters()" />
          <Button icon="pi pi-search" :label="t('aiTasks.actions.search')" size="small" @click="ctrl.search()" />
        </div>
      </div>
    </Fieldset>

    <section class="flex min-h-0 flex-1 flex-col overflow-hidden rounded-lg border border-divider bg-panel shadow-sm">
      <div class="flex items-center justify-between gap-4 border-b border-divider px-4 py-3">
        <h3 class="section-title">{{ t("aiTasks.table.title") }}</h3>
        <span class="text-xs text-muted">{{ t("aiTasks.table.count", { count: ctrl.filteredTasks.value.length.toLocaleString("en-US") }) }}</span>
      </div>
      <DataTable
        class="app-data-table min-h-0"
        :empty-message="ctrl.loading.value ? t('aiTasks.table.loading') : t('aiTasks.table.empty')"
        :row-class="() => 'cursor-pointer'"
        scrollable
        scroll-height="flex"
        :table-style="{ minWidth: '900px' }"
        :value="ctrl.filteredTasks.value"
        paginator
        :rows="pagination.rows"
        :rows-per-page-options="pagination.rowsPerPageOptions"
        :paginator-template="pagination.paginatorTemplate"
        :current-page-report-template="pagination.currentPageReportTemplate"
        @row-click="(e: any) => openEdit(e.data.id)"
      >
        <Column field="task_cd" :header="t('aiTasks.table.taskCd')" body-class="font-mono text-xs text-ink" :style="{ width: '160px' }" />
        <Column field="task_name" :header="t('aiTasks.table.taskName')">
          <template #body="{ data }">
            <span class="text-sm text-ink">{{ data.task_name || "—" }}</span>
          </template>
        </Column>
        <Column field="category_id" :header="t('aiTasks.table.category')" :style="{ width: '110px' }">
          <template #body="{ data }">
            <span :class="['inline-block rounded-full px-2 py-0.5 text-[11px] font-bold', categoryBadgeClass(data.category_id)]">
              {{ data.category_id || "—" }}
            </span>
          </template>
        </Column>
        <Column :header="t('aiTasks.table.progress')" :style="{ width: '260px' }">
          <template #body="{ data }">
            <div v-if="data.current_wf_name" class="flex items-center gap-1.5 text-xs">
              <span class="truncate text-secondary">{{ data.current_wf_name }} › {{ data.current_step_name }}</span>
              <span :class="['shrink-0 rounded-full px-2 py-0.5 text-[10px] font-bold', stepStatusBadgeClass(data.current_step_status)]">
                {{ data.current_step_status }}
              </span>
            </div>
            <span v-else class="text-xs text-muted">{{ t("aiTasks.table.noProgress") }}</span>
          </template>
        </Column>
        <Column field="is_complete" :header="t('aiTasks.table.status')" header-class="text-center" body-class="text-center" :style="{ width: '110px' }">
          <template #body="{ data }">
            <span :class="data.is_complete ? 'badge-success' : 'badge-neutral'">
              {{ data.is_complete ? t("aiTasks.status.complete") : t("aiTasks.status.pending") }}
            </span>
          </template>
        </Column>
        <Column field="created_at" :header="t('aiTasks.table.created')" body-class="text-xs text-muted" :style="{ width: '170px' }" />
      </DataTable>
    </section>

    <TaskDialog v-model:visible="isDialogOpen" :ctrl="ctrl" />
  </section>
</template>
