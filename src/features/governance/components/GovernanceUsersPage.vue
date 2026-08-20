<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import DataTable from "primevue/datatable";
import Column from "primevue/column";
import Fieldset from "primevue/fieldset";
import Button from "primevue/button";
import IconActionButton from "@/shared/components/IconActionButton.vue";
import InputText from "primevue/inputtext";
import GovernanceUserFormDialog from "./GovernanceUserFormDialog.vue";
import GovernanceUserDeleteDialog from "./GovernanceUserDeleteDialog.vue";
import GovernanceUserResetPasswordDialog from "./GovernanceUserResetPasswordDialog.vue";
import { useGovernanceUsers } from "../composables/useGovernanceUsers";
import { useDataTablePagination } from "@/shared/composables/useDataTablePagination";

const { t } = useI18n();
const ctrl = useGovernanceUsers();
const { pagination } = useDataTablePagination();
const isDialogOpen = ref(false);
const deleteDialogVisible = ref(false);
const deleteTargetId = ref<number | null>(null);
const resetPwDialogVisible = ref(false);
const resetPwTargetId = ref<number | null>(null);

const roleBadgeClass = (role: string) => {
  const map: Record<string, string> = {
    admin: "badge-danger",
    manager: "badge-info",
    member: "badge-success",
    viewer: "badge-neutral",
  };
  return map[role] ?? "badge-neutral";
};

function openCreate() {
  ctrl.startCreate();
  isDialogOpen.value = true;
}

function openEdit(id: number) {
  ctrl.selectUser(id);
  isDialogOpen.value = true;
}

function confirmDelete(id: number) {
  deleteTargetId.value = id;
  deleteDialogVisible.value = true;
}

function openResetPassword(id: number) {
  resetPwTargetId.value = id;
  resetPwDialogVisible.value = true;
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
      <Button icon="pi pi-plus" :label="t('governance.users.actions.addUser')" size="small" @click="openCreate" />
    </section>

    <!-- Search fieldset -->
    <Fieldset class="rounded-lg border border-divider bg-panel p-4 shadow-md fieldset-nested" :legend="t('common.searchLegend')" toggleable>
      <div class="grid gap-3">
        <div class="grid gap-3 lg:grid-cols-2">
          <label>
            <span class="text-xs font-bold text-muted">{{ t("governance.users.form.username") }}</span>
            <InputText
              class="mt-1 w-full"
              :placeholder="t('governance.users.form.username')"
              :model-value="ctrl.filters.value.username"
              @update:model-value="ctrl.filters.value = { ...ctrl.filters.value, username: $event as string }"
            />
          </label>
          <label>
            <span class="text-xs font-bold text-muted">{{ t("governance.users.form.fullName") }}</span>
            <InputText
              class="mt-1 w-full"
              :placeholder="t('governance.users.form.fullNamePlaceholder')"
              :model-value="ctrl.filters.value.fullName"
              @update:model-value="ctrl.filters.value = { ...ctrl.filters.value, fullName: $event as string }"
            />
          </label>
        </div>
        <div class="grid gap-3 lg:grid-cols-2">
          <label>
            <span class="text-xs font-bold text-muted">{{ t("governance.users.form.email") }}</span>
            <InputText
              class="mt-1 w-full"
              :placeholder="t('governance.users.form.email')"
              :model-value="ctrl.filters.value.email"
              @update:model-value="ctrl.filters.value = { ...ctrl.filters.value, email: $event as string }"
            />
          </label>
          <label>
            <span class="text-xs font-bold text-muted">{{ t("governance.users.form.phone") }}</span>
            <InputText
              class="mt-1 w-full"
              :placeholder="t('governance.users.form.phonePlaceholder')"
              :model-value="ctrl.filters.value.phone"
              @update:model-value="ctrl.filters.value = { ...ctrl.filters.value, phone: $event as string }"
            />
          </label>
        </div>
        <div class="grid gap-3 lg:grid-cols-2">
          <label>
            <span class="text-xs font-bold text-muted">{{ t("governance.users.form.role") }}</span>
            <select
              class="mt-1 flex h-10 w-full items-center rounded-md border border-divider bg-panel px-3 text-sm"
              :value="ctrl.filters.value.role"
              @change="ctrl.filters.value = { ...ctrl.filters.value, role: ($event.target as HTMLSelectElement).value }"
            >
              <option v-for="r in ctrl.roles.value" :key="r" :value="r">{{ r }}</option>
            </select>
          </label>
          <label>
            <span class="text-xs font-bold text-muted">{{ t("governance.users.form.status") }}</span>
            <select
              class="mt-1 flex h-10 w-full items-center rounded-md border border-divider bg-panel px-3 text-sm"
              :value="ctrl.filters.value.status"
              @change="ctrl.filters.value = { ...ctrl.filters.value, status: ($event.target as HTMLSelectElement).value }"
            >
              <option v-for="s in ctrl.statuses.value" :key="s" :value="s">{{ s }}</option>
            </select>
          </label>
        </div>
        <div class="flex items-center justify-end gap-2">
          <Button icon="pi pi-refresh" :label="t('common.reset')" severity="secondary" outlined size="small" @click="ctrl.resetFilters()" />
          <Button icon="pi pi-search" :label="t('common.search')" size="small" @click="ctrl.search()" />
        </div>
      </div>
    </Fieldset>

    <!-- Users table -->
    <section class="flex min-h-0 flex-1 flex-col overflow-hidden rounded-lg border border-divider bg-panel shadow-sm">
      <div class="flex items-center justify-between gap-4 border-b border-divider px-4 py-3">
        <h3 class="section-title">{{ t("governance.users.table.title") }}</h3>
        <span class="text-xs text-muted">{{ t("governance.users.table.count", { count: ctrl.filteredUsers.value.length.toLocaleString("en-US") }) }}</span>
      </div>
      <DataTable
        class="app-data-table min-h-0"
        :empty-message="ctrl.loading.value ? t('governance.users.table.loading') : t('governance.users.table.empty')"
        :row-class="() => 'cursor-pointer'"
        scrollable
        scroll-height="flex"
        :table-style="{ minWidth: '860px' }"
        :value="ctrl.filteredUsers.value"
        paginator
        :rows="pagination.rows"
        :rows-per-page-options="pagination.rowsPerPageOptions"
        :paginator-template="pagination.paginatorTemplate"
        :current-page-report-template="pagination.currentPageReportTemplate"
        @row-click="(e: any) => openEdit(e.data.id)"
      >
        <Column field="id" :header="t('governance.users.table.id')" body-class="font-mono text-xs text-muted" :style="{ width: '60px' }" />
        <Column field="full_name" :header="t('governance.users.table.user')">
          <template #body="{ data }">
            <div class="flex items-center gap-2.5">
              <span class="flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-brand/10 text-xs font-bold text-brand">
                {{ data.full_name.charAt(0).toUpperCase() }}
              </span>
              <div>
                <span class="font-semibold text-ink">{{ data.full_name }}</span>
                <span class="block text-xs text-muted">{{ data.username }}</span>
              </div>
            </div>
          </template>
        </Column>
        <Column field="email" :header="t('governance.users.table.email')">
          <template #body="{ data }">
            <span class="text-xs text-secondary">{{ data.email || "—" }}</span>
          </template>
        </Column>
        <Column field="phone" :header="t('governance.users.table.phone')">
          <template #body="{ data }">
            <span class="text-xs text-secondary">{{ data.phone || "—" }}</span>
          </template>
        </Column>
        <Column field="position" :header="t('governance.users.table.position')">
          <template #body="{ data }">
            <span class="text-xs text-secondary">{{ data.position || "—" }}</span>
          </template>
        </Column>
        <Column field="roles" :header="t('governance.users.table.roles')">
          <template #body="{ data }">
            <div class="flex flex-wrap gap-1">
              <span
                v-for="role in data.roles"
                :key="role"
                :class="roleBadgeClass(role)"
              >
                {{ role }}
              </span>
            </div>
          </template>
        </Column>
        <Column field="is_active" :header="t('governance.users.table.status')" header-class="text-center" body-class="text-center">
          <template #body="{ data }">
            <span :class="data.is_active ? 'badge-success' : 'badge-neutral'">
              <i :class="['pi text-[10px]', data.is_active ? 'pi-check-circle' : 'pi-minus-circle']" />
              {{ data.is_active ? t("governance.users.status.active") : t("governance.users.status.inactive") }}
            </span>
          </template>
        </Column>
        <Column :header="t('governance.users.table.actions')" header-class="text-center" body-class="text-center" :style="{ width: '90px' }">
          <template #body="{ data }">
            <div class="flex items-center justify-center gap-1">
              <IconActionButton icon="pi pi-key" severity="secondary" :title="t('governance.users.actions.resetPassword')" @click.stop="openResetPassword(data.id)" />
              <IconActionButton icon="pi pi-trash" severity="danger" :title="t('governance.users.actions.deleteUser')" @click.stop="confirmDelete(data.id)" />
            </div>
          </template>
        </Column>
      </DataTable>
    </section>

    <!-- Add / Edit dialog -->
    <GovernanceUserFormDialog v-model:visible="isDialogOpen" :ctrl="ctrl" />

    <!-- Delete confirmation dialog -->
    <GovernanceUserDeleteDialog v-model:visible="deleteDialogVisible" :ctrl="ctrl" :user-id="deleteTargetId" />

    <!-- Reset password dialog -->
    <GovernanceUserResetPasswordDialog v-model:visible="resetPwDialogVisible" :ctrl="ctrl" :user-id="resetPwTargetId" />
  </section>
</template>
