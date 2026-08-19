<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import DataTable from "primevue/datatable";
import Column from "primevue/column";
import Fieldset from "primevue/fieldset";
import Button from "primevue/button";
import InputText from "primevue/inputtext";
import Select from "primevue/select";
import IconActionButton from "@/shared/components/IconActionButton.vue";
import AgentProviderModelFormDialog from "./AgentProviderModelFormDialog.vue";
import AgentProviderModelDeleteDialog from "./AgentProviderModelDeleteDialog.vue";
import { useAgentProviderModel } from "../composables/useAgentProviderModel";
import { useDataTablePagination } from "@/shared/composables/useDataTablePagination";
import type { AgentProviderModel } from "@/models/agent-provider-model";

const { t } = useI18n();
const ctrl = useAgentProviderModel();
const { pagination } = useDataTablePagination();

const isDialogOpen = ref(false);
const deleteDialogVisible = ref(false);
const deleteTarget = ref<AgentProviderModel | null>(null);

const providerFilterOptions = computed(() => [
  { label: t("agentProviderModel.filters.allProviders"), value: "All" as const },
  ...ctrl.providers.value.map((p) => ({ label: p.name, value: p.id })),
]);

const statusOptions = computed(() => [
  { label: t("agentProviderModel.filters.allStatus"), value: "All" },
  { label: t("agentProviderModel.status.enabled"), value: "enabled" },
  { label: t("agentProviderModel.status.disabled"), value: "disabled" },
]);

function openCreate() {
  ctrl.startCreate();
  isDialogOpen.value = true;
}

function openEdit(id: number) {
  ctrl.selectModel(id);
  isDialogOpen.value = true;
}

function confirmDelete(model: AgentProviderModel) {
  deleteTarget.value = model;
  deleteDialogVisible.value = true;
}

async function executeDelete() {
  if (!deleteTarget.value) return;
  if (await ctrl.removeModel(deleteTarget.value.id)) {
    deleteDialogVisible.value = false;
    deleteTarget.value = null;
  }
}

onMounted(() => ctrl.init());
</script>

<template>
  <section class="flex min-h-0 flex-1 flex-col gap-4 overflow-hidden">
    <!-- Error banner -->
    <p v-if="ctrl.error.value" class="banner-danger">
      {{ ctrl.error.value }}
    </p>

    <!-- Action bar -->
    <section class="flex items-center justify-between rounded-lg border border-divider bg-panel p-4 shadow-sm">
      <div>
        <h3 class="section-title">{{ t("agentProviderModel.title") }}</h3>
        <p class="mt-0.5 text-xs text-muted">{{ t("agentProviderModel.subtitle") }}</p>
      </div>
      <Button icon="pi pi-plus" :label="t('agentProviderModel.actions.register')" size="small" @click="openCreate" />
    </section>

    <!-- Search fieldset -->
    <Fieldset
      class="rounded-lg border border-divider bg-panel p-4 shadow-md fieldset-nested"
      :legend="t('agentProviderModel.search.legend')"
      toggleable
    >
      <div class="grid gap-3">
        <div class="grid gap-3 lg:grid-cols-3">
          <label>
            <span class="text-xs font-bold text-muted">{{ t("agentProviderModel.search.keyword") }}</span>
            <InputText
              class="mt-1 w-full"
              :placeholder="t('agentProviderModel.search.keywordPlaceholder')"
              :model-value="ctrl.filters.value.keyword"
              @update:model-value="ctrl.filters.value = { ...ctrl.filters.value, keyword: ($event as string) ?? '' }"
              @keyup.enter="ctrl.search()"
            />
          </label>
          <label>
            <span class="text-xs font-bold text-muted">{{ t("agentProviderModel.form.provider") }}</span>
            <Select
              class="mt-1 w-full"
              :options="providerFilterOptions"
              option-label="label"
              option-value="value"
              :model-value="ctrl.filters.value.providerId"
              @update:model-value="ctrl.filters.value = { ...ctrl.filters.value, providerId: $event }"
            />
          </label>
          <label>
            <span class="text-xs font-bold text-muted">{{ t("agentProviderModel.form.status") }}</span>
            <Select
              class="mt-1 w-full"
              :options="statusOptions"
              option-label="label"
              option-value="value"
              :model-value="ctrl.filters.value.status"
              @update:model-value="ctrl.filters.value = { ...ctrl.filters.value, status: $event }"
            />
          </label>
        </div>
        <div class="flex items-center justify-end gap-2">
          <Button icon="pi pi-refresh" :label="t('agentProviderModel.actions.reset')" severity="secondary" outlined size="small" @click="ctrl.resetFilters()" />
          <Button icon="pi pi-search" :label="t('agentProviderModel.actions.search')" size="small" @click="ctrl.search()" />
        </div>
      </div>
    </Fieldset>

    <!-- Models table -->
    <section class="flex min-h-0 flex-1 flex-col overflow-hidden rounded-lg border border-divider bg-panel shadow-sm">
      <div class="flex items-center justify-between gap-4 border-b border-divider px-4 py-3">
        <h3 class="section-title">{{ t("agentProviderModel.table.title") }}</h3>
        <span class="text-xs text-muted">{{ t("agentProviderModel.table.count", { count: ctrl.filteredModels.value.length.toLocaleString("en-US") }) }}</span>
      </div>
      <DataTable
        class="app-data-table min-h-0"
        :empty-message="ctrl.loading.value ? t('agentProviderModel.table.loading') : t('agentProviderModel.table.empty')"
        :row-class="() => 'cursor-pointer'"
        scrollable
        scroll-height="flex"
        :table-style="{ minWidth: '900px' }"
        :value="ctrl.filteredModels.value"
        paginator
        :rows="pagination.rows"
        :rows-per-page-options="pagination.rowsPerPageOptions"
        :paginator-template="pagination.paginatorTemplate"
        :current-page-report-template="pagination.currentPageReportTemplate"
        @row-click="(e: any) => openEdit(e.data.id)"
      >
        <Column field="name" :header="t('agentProviderModel.table.name')">
          <template #body="{ data }">
            <div class="min-w-0">
              <span class="font-semibold text-ink">{{ data.name }}</span>
              <span v-if="data.code" class="block font-mono text-[11px] text-muted">{{ data.code }}</span>
            </div>
          </template>
        </Column>
        <Column field="provider_name" :header="t('agentProviderModel.table.provider')">
          <template #body="{ data }">
            <span class="inline-flex items-center gap-1 rounded-full bg-brand/10 px-2 py-0.5 text-[10px] font-bold text-brand">
              <i class="pi pi-android text-[10px]" />
              {{ data.provider_name }}
            </span>
          </template>
        </Column>
        <Column field="version" :header="t('agentProviderModel.table.version')">
          <template #body="{ data }">
            <span class="text-xs text-secondary">{{ data.version || "—" }}</span>
          </template>
        </Column>
        <Column field="description" :header="t('agentProviderModel.table.description')">
          <template #body="{ data }">
            <span class="text-xs text-secondary">{{ data.description || "—" }}</span>
          </template>
        </Column>
        <Column field="enabled" :header="t('agentProviderModel.table.status')" header-class="text-center" body-class="text-center" :style="{ width: '130px' }">
          <template #body="{ data }">
            <button
              type="button"
              :class="[
                'inline-flex items-center gap-1.5 rounded-full px-2.5 py-1 text-[10px] font-bold transition-colors',
                data.enabled ? 'badge-success' : 'badge-neutral',
              ]"
              :title="data.enabled ? t('agentProviderModel.actions.disable') : t('agentProviderModel.actions.enable')"
              @click.stop="ctrl.toggleEnabled(data)"
            >
              <i :class="['pi text-[10px]', data.enabled ? 'pi-check-circle' : 'pi-minus-circle']" />
              {{ data.enabled ? t("agentProviderModel.status.enabled") : t("agentProviderModel.status.disabled") }}
            </button>
          </template>
        </Column>
        <Column :header="t('agentProviderModel.table.actions')" header-class="text-center" body-class="text-center" :style="{ width: '90px' }">
          <template #body="{ data }">
            <div class="flex items-center justify-center gap-1">
              <IconActionButton icon="pi pi-pencil" severity="secondary" :title="t('agentProviderModel.actions.edit')" @click.stop="openEdit(data.id)" />
              <IconActionButton icon="pi pi-trash" severity="danger" :title="t('agentProviderModel.actions.delete')" @click.stop="confirmDelete(data)" />
            </div>
          </template>
        </Column>
      </DataTable>
    </section>

    <!-- Register / Edit dialog -->
    <AgentProviderModelFormDialog v-model:visible="isDialogOpen" :ctrl="ctrl" />

    <!-- Delete confirmation dialog -->
    <AgentProviderModelDeleteDialog
      v-model:visible="deleteDialogVisible"
      :model="deleteTarget"
      @confirm="executeDelete"
    />
  </section>
</template>
