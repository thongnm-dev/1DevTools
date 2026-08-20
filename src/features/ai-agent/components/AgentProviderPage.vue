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
import AgentProviderFormDialog from "./AgentProviderFormDialog.vue";
import AgentProviderDeleteDialog from "./AgentProviderDeleteDialog.vue";
import { useAgentProvider } from "../composables/useAgentProvider";
import { useDataTablePagination } from "@/shared/composables/useDataTablePagination";
import type { AgentProvider, AgentProviderType } from "@/models/agent-provider";
import { AGENT_PROVIDER_TYPES, AGENT_PROVIDER_TYPE_META } from "@/models/agent-provider";

const { t } = useI18n();
const ctrl = useAgentProvider();
const { pagination } = useDataTablePagination();

const isDialogOpen = ref(false);
const deleteDialogVisible = ref(false);
const deleteTarget = ref<AgentProvider | null>(null);

const typeOptions = computed(() => [
  { label: t("agentProvider.filters.allTypes"), value: "All" },
  ...AGENT_PROVIDER_TYPES.map((v) => ({ label: t(`agentProvider.type.${v}`), value: v })),
]);

const statusOptions = computed(() => [
  { label: t("agentProvider.filters.allStatus"), value: "All" },
  { label: t("agentProvider.status.enabled"), value: "enabled" },
  { label: t("agentProvider.status.disabled"), value: "disabled" },
]);

function typeBadgeClass(type: AgentProviderType): string {
  return AGENT_PROVIDER_TYPE_META[type]?.badgeClass ?? "bg-canvas text-muted";
}

function typeIcon(type: AgentProviderType): string {
  return AGENT_PROVIDER_TYPE_META[type]?.icon ?? "pi pi-cog";
}

function openCreate() {
  ctrl.startCreate();
  isDialogOpen.value = true;
}

function openEdit(id: number) {
  ctrl.selectProvider(id);
  isDialogOpen.value = true;
}

function confirmDelete(provider: AgentProvider) {
  deleteTarget.value = provider;
  deleteDialogVisible.value = true;
}

async function executeDelete() {
  if (!deleteTarget.value) return;
  if (await ctrl.removeProvider(deleteTarget.value.id)) {
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
        <h3 class="section-title">{{ t("agentProvider.title") }}</h3>
        <p class="mt-0.5 text-xs text-muted">{{ t("agentProvider.subtitle") }}</p>
      </div>
      <Button icon="pi pi-plus" :label="t('common.register')" size="small" @click="openCreate" />
    </section>

    <!-- Search fieldset -->
    <Fieldset
      class="rounded-lg border border-divider bg-panel p-4 shadow-md fieldset-nested"
      :legend="t('common.searchLegend')"
      toggleable
    >
      <div class="grid gap-3">
        <div class="grid gap-3 lg:grid-cols-3">
          <label>
            <span class="text-xs font-bold text-muted">{{ t("agentProvider.search.keyword") }}</span>
            <InputText
              class="mt-1 w-full"
              :placeholder="t('agentProvider.search.keywordPlaceholder')"
              :model-value="ctrl.filters.value.keyword"
              @update:model-value="ctrl.filters.value = { ...ctrl.filters.value, keyword: ($event as string) ?? '' }"
              @keyup.enter="ctrl.search()"
            />
          </label>
          <label>
            <span class="text-xs font-bold text-muted">{{ t("agentProvider.form.type") }}</span>
            <Select
              class="mt-1 w-full"
              :options="typeOptions"
              option-label="label"
              option-value="value"
              :model-value="ctrl.filters.value.providerType"
              @update:model-value="ctrl.filters.value = { ...ctrl.filters.value, providerType: $event }"
            />
          </label>
          <label>
            <span class="text-xs font-bold text-muted">{{ t("agentProvider.form.status") }}</span>
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
          <Button icon="pi pi-refresh" :label="t('common.reset')" severity="secondary" outlined size="small" @click="ctrl.resetFilters()" />
          <Button icon="pi pi-search" :label="t('common.search')" size="small" @click="ctrl.search()" />
        </div>
      </div>
    </Fieldset>

    <!-- Providers table -->
    <section class="flex min-h-0 flex-1 flex-col overflow-hidden rounded-lg border border-divider bg-panel shadow-sm">
      <div class="flex items-center justify-between gap-4 border-b border-divider px-4 py-3">
        <h3 class="section-title">{{ t("agentProvider.table.title") }}</h3>
        <span class="text-xs text-muted">{{ t("agentProvider.table.count", { count: ctrl.filteredProviders.value.length.toLocaleString("en-US") }) }}</span>
      </div>
      <DataTable
        class="app-data-table min-h-0"
        :empty-message="ctrl.loading.value ? t('agentProvider.table.loading') : t('agentProvider.table.empty')"
        :row-class="() => 'cursor-pointer'"
        scrollable
        scroll-height="flex"
        :table-style="{ minWidth: '900px' }"
        :value="ctrl.filteredProviders.value"
        paginator
        :rows="pagination.rows"
        :rows-per-page-options="pagination.rowsPerPageOptions"
        :paginator-template="pagination.paginatorTemplate"
        :current-page-report-template="pagination.currentPageReportTemplate"
        @row-click="(e: any) => openEdit(e.data.id)"
      >
        <Column field="name" :header="t('agentProvider.table.name')">
          <template #body="{ data }">
            <div class="flex items-center gap-2.5">
              <span class="flex h-8 w-8 shrink-0 items-center justify-center rounded-md bg-brand/10 text-brand">
                <i :class="[data.icon || typeIcon(data.provider_type), 'text-sm']" />
              </span>
              <div class="min-w-0">
                <span class="font-semibold text-ink">{{ data.name }}</span>
                <span v-if="data.code" class="block font-mono text-[11px] text-muted">{{ data.code }}</span>
              </div>
            </div>
          </template>
        </Column>
        <Column field="provider_type" :header="t('agentProvider.table.type')">
          <template #body="{ data }">
            <span :class="['inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-[10px] font-bold', typeBadgeClass(data.provider_type)]">
              <i :class="[typeIcon(data.provider_type), 'text-[10px]']" />
              {{ t(`agentProvider.type.${data.provider_type}`) }}
            </span>
          </template>
        </Column>
        <Column field="command" :header="t('agentProvider.table.command')">
          <template #body="{ data }">
            <code v-if="data.command" class="rounded bg-canvas px-1.5 py-0.5 font-mono text-[11px] text-secondary">{{ data.command }}</code>
            <span v-else class="text-xs text-muted">—</span>
          </template>
        </Column>
        <Column field="models" :header="t('agentProvider.table.models')">
          <template #body="{ data }">
            <span class="text-xs text-secondary">{{ data.models.length ? data.models.join(", ") : "—" }}</span>
          </template>
        </Column>
        <Column field="enabled" :header="t('agentProvider.table.status')" header-class="text-center" body-class="text-center" :style="{ width: '130px' }">
          <template #body="{ data }">
            <button
              type="button"
              :class="[
                'inline-flex items-center gap-1.5 rounded-full px-2.5 py-1 text-[10px] font-bold transition-colors',
                data.enabled ? 'badge-success' : 'badge-neutral',
              ]"
              :title="data.enabled ? t('agentProvider.actions.disable') : t('agentProvider.actions.enable')"
              @click.stop="ctrl.toggleEnabled(data)"
            >
              <i :class="['pi text-[10px]', data.enabled ? 'pi-check-circle' : 'pi-minus-circle']" />
              {{ data.enabled ? t("agentProvider.status.enabled") : t("agentProvider.status.disabled") }}
            </button>
          </template>
        </Column>
        <Column :header="t('agentProvider.table.actions')" header-class="text-center" body-class="text-center" :style="{ width: '90px' }">
          <template #body="{ data }">
            <div class="flex items-center justify-center gap-1">
              <IconActionButton icon="pi pi-pencil" severity="secondary" :title="t('common.edit')" @click.stop="openEdit(data.id)" />
              <IconActionButton icon="pi pi-trash" severity="danger" :title="t('common.delete')" @click.stop="confirmDelete(data)" />
            </div>
          </template>
        </Column>
      </DataTable>
    </section>

    <!-- Register / Edit dialog -->
    <AgentProviderFormDialog v-model:visible="isDialogOpen" :ctrl="ctrl" />

    <!-- Delete confirmation dialog -->
    <AgentProviderDeleteDialog
      v-model:visible="deleteDialogVisible"
      :provider="deleteTarget"
      @confirm="executeDelete"
    />
  </section>
</template>
