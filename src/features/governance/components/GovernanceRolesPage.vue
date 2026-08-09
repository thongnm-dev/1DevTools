<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import DataTable from "primevue/datatable";
import Column from "primevue/column";
import Fieldset from "primevue/fieldset";
import Dialog from "primevue/dialog";
import Button from "primevue/button";
import InputText from "primevue/inputtext";
import Textarea from "primevue/textarea";
import IconActionButton from "@/shared/components/IconActionButton.vue";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import { useGovernanceRoles } from "../composables/useGovernanceRoles";
import { useDataTablePagination } from "@/shared/composables/useDataTablePagination";
import { useToast } from "@/shared/composables/useToast";

const { t } = useI18n();
const ctrl = useGovernanceRoles();
const toast = useToast();
const { pagination } = useDataTablePagination();
const isDialogOpen = ref(false);
const confirmDeleteId = ref<number | null>(null);

function openCreate() {
  ctrl.startCreate();
  isDialogOpen.value = true;
}

function openEdit(id: number) {
  ctrl.selectRole(id);
  isDialogOpen.value = true;
}

function closeDialog() {
  isDialogOpen.value = false;
}

async function saveAndClose() {
  if (await ctrl.saveDraft()) {
    toast.success(ctrl.isCreating.value ? t("governance.roles.toast.created") : t("governance.roles.toast.updated"));
    closeDialog();
  }
}

function confirmDelete(id: number) {
  confirmDeleteId.value = id;
}

async function executeDelete() {
  if (confirmDeleteId.value !== null) {
    const ok = await ctrl.removeRole(confirmDeleteId.value);
    if (ok) toast.success(t("governance.roles.toast.deleted"));
    confirmDeleteId.value = null;
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
    <Dialog
      :visible="isDialogOpen"
      class="w-full max-w-lg rounded-lg bg-panel shadow-xl"
      :closable="true"
      modal
      @update:visible="isDialogOpen = $event"
    >
      <template #header>
        <div>
          <h3 class="section-title">{{ ctrl.isCreating.value ? t("governance.roles.dialog.addTitle") : t("governance.roles.dialog.editTitle") }}</h3>
          <p v-if="ctrl.draft.value && !ctrl.isCreating.value" class="mt-1 text-sm text-muted">
            {{ t("governance.roles.dialog.idLabel", { id: ctrl.draft.value.id }) }}
          </p>
        </div>
      </template>

      <div v-if="ctrl.draft.value" class="space-y-4">
        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("governance.roles.form.name") }} <span class="text-red-500">*</span></span>
          <InputText
            class="mt-1 w-full"
            :model-value="ctrl.draft.value.name"
            :placeholder="t('governance.roles.form.namePlaceholder')"
            autofocus
            @update:model-value="ctrl.updateDraft('name', $event as string)"
          />
        </label>

        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("governance.roles.form.description") }}</span>
          <Textarea
            class="mt-1 w-full"
            :model-value="ctrl.draft.value.description"
            :placeholder="t('governance.roles.form.descriptionPlaceholder')"
            rows="3"
            auto-resize
            @update:model-value="ctrl.updateDraft('description', $event as string)"
          />
        </label>
      </div>

      <template #footer>
        <DialogFooter :confirm-label="ctrl.isCreating.value ? t('governance.roles.actions.create') : t('governance.roles.actions.save')" @cancel="closeDialog" @confirm="saveAndClose" />
      </template>
    </Dialog>

    <!-- Delete confirmation dialog -->
    <Dialog
      :visible="confirmDeleteId !== null"
      class="w-full max-w-sm rounded-lg bg-panel shadow-xl"
      :closable="true"
      modal
      @update:visible="confirmDeleteId = null"
    >
      <template #header>
        <h3 class="section-title">{{ t("governance.roles.dialog.deleteTitle") }}</h3>
      </template>
      <p class="text-sm text-secondary">{{ t("governance.roles.dialog.deleteMessage") }}</p>
      <template #footer>
        <DialogFooter :confirm-label="t('governance.roles.actions.delete')" confirm-severity="danger" @cancel="confirmDeleteId = null" @confirm="executeDelete" />
      </template>
    </Dialog>
  </section>
</template>
