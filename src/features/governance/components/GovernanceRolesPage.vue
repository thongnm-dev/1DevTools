<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import DataTable from "primevue/datatable";
import Column from "primevue/column";
import Fieldset from "primevue/fieldset";
import Button from "primevue/button";
import InputText from "primevue/inputtext";
import IconActionButton from "@/shared/components/IconActionButton.vue";
import GovernanceRoleFormDialog from "./GovernanceRoleFormDialog.vue";
import GovernanceRoleDeleteDialog from "./GovernanceRoleDeleteDialog.vue";
import { useGovernanceRoles } from "../composables/useGovernanceRoles";
import { useDataTablePagination } from "@/shared/composables/useDataTablePagination";

const { t } = useI18n();
const ctrl = useGovernanceRoles();
const { pagination } = useDataTablePagination();
const isDialogOpen = ref(false);
const deleteDialogVisible = ref(false);
const deleteTargetId = ref<number | null>(null);

function openCreate() {
  ctrl.startCreate();
  isDialogOpen.value = true;
}

function openEdit(id: number) {
  ctrl.selectRole(id);
  isDialogOpen.value = true;
}

function confirmDelete(id: number) {
  deleteTargetId.value = id;
  deleteDialogVisible.value = true;
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
    <section class="flex items-center justify-end rounded-lg border border-divider bg-panel p-4 shadow-sm">
      <Button icon="pi pi-plus" :label="t('governance.roles.actions.addRole')" size="small" @click="openCreate" />
    </section>

    <!-- Search fieldset -->
    <Fieldset class="rounded-lg border border-divider bg-panel p-4 shadow-md fieldset-nested" :legend="t('governance.roles.search.legend')" toggleable>
      <div class="grid gap-3">
        <div class="grid gap-3 lg:grid-cols-2">
          <label>
            <span class="text-xs font-bold text-muted">{{ t("governance.roles.form.name") }}</span>
            <InputText
              class="mt-1 w-full"
              :placeholder="t('governance.roles.search.namePlaceholder')"
              :model-value="ctrl.filters.value.name"
              @update:model-value="ctrl.filters.value = { ...ctrl.filters.value, name: $event as string }"
            />
          </label>
          <label>
            <span class="text-xs font-bold text-muted">{{ t("governance.roles.form.description") }}</span>
            <InputText
              class="mt-1 w-full"
              :placeholder="t('governance.roles.form.description')"
              :model-value="ctrl.filters.value.description"
              @update:model-value="ctrl.filters.value = { ...ctrl.filters.value, description: $event as string }"
            />
          </label>
        </div>
        <div class="flex items-center justify-end gap-2">
          <Button icon="pi pi-refresh" :label="t('governance.roles.actions.reset')" severity="secondary" outlined size="small" @click="ctrl.resetFilters()" />
          <Button icon="pi pi-search" :label="t('governance.roles.actions.search')" size="small" @click="ctrl.search()" />
        </div>
      </div>
    </Fieldset>

    <!-- Roles table -->
    <section class="flex min-h-0 flex-1 flex-col overflow-hidden rounded-lg border border-divider bg-panel shadow-sm">
      <div class="flex items-center justify-between gap-4 border-b border-divider px-4 py-3">
        <h3 class="section-title">{{ t("governance.roles.title") }}</h3>
        <span class="text-xs text-muted">{{ t("governance.roles.table.count", { count: ctrl.filteredRoles.value.length.toLocaleString("en-US") }) }}</span>
      </div>
      <DataTable
        class="app-data-table min-h-0"
        :empty-message="ctrl.loading.value ? t('governance.roles.table.loading') : t('governance.roles.table.empty')"
        :row-class="() => 'cursor-pointer'"
        scrollable
        scroll-height="flex"
        :table-style="{ minWidth: '640px' }"
        :value="ctrl.filteredRoles.value"
        paginator
        :rows="pagination.rows"
        :rows-per-page-options="pagination.rowsPerPageOptions"
        :paginator-template="pagination.paginatorTemplate"
        :current-page-report-template="pagination.currentPageReportTemplate"
        @row-click="(e: any) => openEdit(e.data.id)"
      >
        <Column field="id" :header="t('governance.roles.table.id')" body-class="font-mono text-xs text-muted" :style="{ width: '60px' }" />
        <Column field="name" :header="t('governance.roles.table.role')">
          <template #body="{ data }">
            <div class="flex items-center gap-2.5">
              <span class="flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-brand/10 text-xs font-bold text-brand">
                <i class="pi pi-shield text-xs" />
              </span>
              <span class="font-semibold text-ink">{{ data.name }}</span>
            </div>
          </template>
        </Column>
        <Column field="description" :header="t('governance.roles.table.description')">
          <template #body="{ data }">
            <span class="text-xs text-secondary">{{ data.description || "—" }}</span>
          </template>
        </Column>
        <Column field="user_count" :header="t('governance.roles.table.users')" header-class="text-center" body-class="text-center" :style="{ width: '90px' }">
          <template #body="{ data }">
            <span class="inline-flex items-center gap-1 rounded-md bg-canvas px-2 py-0.5 text-[11px] font-bold text-secondary">
              <i class="pi pi-users text-[10px]" />
              {{ data.user_count.toLocaleString("en-US") }}
            </span>
          </template>
        </Column>
        <Column field="created_at" :header="t('governance.roles.table.created')" body-class="text-xs text-muted" :style="{ width: '170px' }">
          <template #body="{ data }">
            <span class="text-xs text-muted">{{ data.created_at || "—" }}</span>
          </template>
        </Column>
        <Column :header="t('governance.roles.table.actions')" header-class="text-center" body-class="text-center" :style="{ width: '70px' }">
          <template #body="{ data }">
            <div class="flex items-center justify-center gap-1">
              <IconActionButton icon="pi pi-trash" severity="danger" :title="t('governance.roles.actions.delete')" @click.stop="confirmDelete(data.id)" />
            </div>
          </template>
        </Column>
      </DataTable>
    </section>

    <!-- Add / Edit dialog -->
    <GovernanceRoleFormDialog v-model:visible="isDialogOpen" :ctrl="ctrl" />

    <!-- Delete confirmation dialog -->
    <GovernanceRoleDeleteDialog v-model:visible="deleteDialogVisible" :ctrl="ctrl" :role-id="deleteTargetId" />
  </section>
</template>
