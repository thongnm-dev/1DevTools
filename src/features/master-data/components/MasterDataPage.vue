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
import MasterDataFormDialog from "./MasterDataFormDialog.vue";
import MasterDataDeleteDialog from "./MasterDataDeleteDialog.vue";
import { useMasterData } from "../composables/useMasterData";
import { useDataTablePagination } from "@/shared/composables/useDataTablePagination";
import type { MasterData } from "@/models/master-data";

const { t } = useI18n();
const ctrl = useMasterData();
const { pagination } = useDataTablePagination();

const isDialogOpen = ref(false);
const deleteDialogVisible = ref(false);
const deleteTarget = ref<MasterData | null>(null);

const keygroupOptions = computed(() => [
  { label: t("masterData.filters.allKeygroups"), value: "All" },
  ...ctrl.keygroupOptions.value.map((v) => ({ label: v, value: v })),
]);

function openCreate() {
  ctrl.startCreate();
  isDialogOpen.value = true;
}

function openEdit(id: number) {
  ctrl.selectItem(id);
  isDialogOpen.value = true;
}

function confirmDelete(item: MasterData) {
  deleteTarget.value = item;
  deleteDialogVisible.value = true;
}

async function executeDelete() {
  if (!deleteTarget.value) return;
  if (await ctrl.removeItem(deleteTarget.value.id)) {
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
        <h3 class="section-title">{{ t("masterData.title") }}</h3>
        <p class="mt-0.5 text-xs text-muted">{{ t("masterData.subtitle") }}</p>
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
        <div class="grid gap-3 lg:grid-cols-2">
          <label>
            <span class="text-xs font-bold text-muted">{{ t("masterData.search.keyword") }}</span>
            <InputText
              class="mt-1 w-full"
              :placeholder="t('masterData.search.keywordPlaceholder')"
              :model-value="ctrl.filters.value.keyword"
              @update:model-value="ctrl.filters.value = { ...ctrl.filters.value, keyword: ($event as string) ?? '' }"
              @keyup.enter="ctrl.search()"
            />
          </label>
          <label>
            <span class="text-xs font-bold text-muted">{{ t("masterData.form.keygroup") }}</span>
            <Select
              class="mt-1 w-full"
              :options="keygroupOptions"
              option-label="label"
              option-value="value"
              :model-value="ctrl.filters.value.keygroup"
              @update:model-value="ctrl.filters.value = { ...ctrl.filters.value, keygroup: $event }"
            />
          </label>
        </div>
        <div class="flex items-center justify-end gap-2">
          <Button icon="pi pi-refresh" :label="t('common.reset')" severity="secondary" outlined size="small" @click="ctrl.resetFilters()" />
          <Button icon="pi pi-search" :label="t('common.search')" size="small" @click="ctrl.search()" />
        </div>
      </div>
    </Fieldset>

    <!-- Results table -->
    <section class="flex min-h-0 flex-1 flex-col overflow-hidden rounded-lg border border-divider bg-panel shadow-sm">
      <div class="flex items-center justify-between gap-4 border-b border-divider px-4 py-3">
        <h3 class="section-title">{{ t("masterData.table.title") }}</h3>
        <span class="text-xs text-muted">{{ t("masterData.table.count", { count: ctrl.filteredItems.value.length.toLocaleString("en-US") }) }}</span>
      </div>
      <DataTable
        class="app-data-table min-h-0"
        :empty-message="ctrl.loading.value ? t('masterData.table.loading') : t('masterData.table.empty')"
        :row-class="() => 'cursor-pointer'"
        scrollable
        scroll-height="flex"
        :table-style="{ minWidth: '760px' }"
        :value="ctrl.filteredItems.value"
        paginator
        :rows="pagination.rows"
        :rows-per-page-options="pagination.rowsPerPageOptions"
        :paginator-template="pagination.paginatorTemplate"
        :current-page-report-template="pagination.currentPageReportTemplate"
        @row-click="(e: any) => openEdit(e.data.id)"
      >
        <Column field="name" :header="t('masterData.table.name')">
          <template #body="{ data }">
            <div class="flex items-center gap-2.5">
              <span class="flex h-8 w-8 shrink-0 items-center justify-center rounded-md bg-brand/10 text-brand">
                <i :class="[data.icon || 'pi pi-tag', 'text-sm']" />
              </span>
              <span class="font-semibold text-ink">{{ data.name }}</span>
            </div>
          </template>
        </Column>
        <Column field="keygroup" :header="t('masterData.table.keygroup')">
          <template #body="{ data }">
            <span v-if="data.keygroup" class="inline-flex items-center rounded-full badge-neutral px-2 py-0.5 text-[10px] font-bold">{{ data.keygroup }}</span>
            <span v-else class="text-xs text-muted">—</span>
          </template>
        </Column>
        <Column field="display_order" :header="t('masterData.table.displayOrder')" header-class="text-center" body-class="text-center" :style="{ width: '110px' }">
          <template #body="{ data }">
            <span class="text-xs text-secondary">{{ data.display_order }}</span>
          </template>
        </Column>
        <Column field="description" :header="t('masterData.table.description')">
          <template #body="{ data }">
            <span class="text-xs text-secondary">{{ data.description || "—" }}</span>
          </template>
        </Column>
        <Column :header="t('masterData.table.actions')" header-class="text-center" body-class="text-center" :style="{ width: '90px' }">
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
    <MasterDataFormDialog v-model:visible="isDialogOpen" :ctrl="ctrl" />

    <!-- Delete confirmation dialog -->
    <MasterDataDeleteDialog
      v-model:visible="deleteDialogVisible"
      :item="deleteTarget"
      @confirm="executeDelete"
    />
  </section>
</template>
