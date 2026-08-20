<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import Button from "primevue/button";
import InputText from "primevue/inputtext";
import IconPickerDialog from "@/shared/components/IconPickerDialog.vue";
import IconActionButton from "@/shared/components/IconActionButton.vue";
import GovernanceMenuEditDialog from "./GovernanceMenuEditDialog.vue";
import GovernanceMenuCreateDialog from "./GovernanceMenuCreateDialog.vue";
import { useGovernanceMenus } from "../composables/useGovernanceMenus";

const { t } = useI18n();
const ctrl = useGovernanceMenus();
const isEditing = ref(false);
const isCreating = ref(false);
const showIconPicker = ref(false);

const groupBadgeClass = (group: string) =>
  group === "—"
    ? "badge-neutral"
    : group === "Tools"
      ? "badge-info"
      : "badge-success";

function openEdit(key: string) {
  ctrl.selectItem(key);
  isEditing.value = true;
}

function openCreate() {
  ctrl.startCreate();
  isCreating.value = true;
}
</script>

<template>
  <section class="flex min-h-0 flex-1 flex-col gap-4 overflow-hidden">
    <!-- Top bar -->
    <section class="flex flex-wrap items-end gap-3 rounded-lg border border-divider bg-panel p-4 shadow-sm">
      <label class="block min-w-0 flex-1">
        <span class="text-xs font-bold text-muted">{{ t("common.search") }}</span>
        <span class="mt-1 flex h-10 items-center gap-2 rounded-md border border-divider bg-panel px-3 focus-within:border-brand focus-within:ring-2 focus-within:ring-brand/20">
          <i class="pi pi-search shrink-0 text-muted" />
          <InputText
            class="embedded-input min-w-0 flex-1 border-0 bg-transparent p-0 text-sm text-ink outline-none shadow-none"
            :placeholder="t('governance.menus.form.searchPlaceholder')"
            :model-value="ctrl.searchQuery.value"
            @update:model-value="ctrl.searchQuery.value = $event as string"
          />
        </span>
      </label>
      <label class="block w-44">
        <span class="text-xs font-bold text-muted">{{ t("governance.menus.form.group") }}</span>
        <select
          class="mt-1 flex h-10 w-full items-center rounded-md border border-divider bg-panel px-3 text-sm"
          :value="ctrl.filterGroup.value"
          @change="ctrl.filterGroup.value = ($event.target as HTMLSelectElement).value"
        >
          <option v-for="g in ctrl.groups.value" :key="g" :value="g">{{ g }}</option>
        </select>
      </label>
      <Button icon="pi pi-refresh" :label="t('common.reset')" severity="secondary" outlined size="small" :title="t('governance.menus.actions.resetTitle')" @click="ctrl.resetToDefault()" />
      <Button icon="pi pi-plus" :label="t('governance.menus.actions.add')" size="small" :title="t('governance.menus.actions.addTitle')" @click="openCreate()" />
    </section>

    <!-- Menu table -->
    <section class="min-h-0 flex-1 overflow-auto rounded-lg border border-divider bg-panel shadow-sm">
      <table class="w-full text-sm">
        <thead class="sticky top-0 z-10 bg-panel">
          <tr class="border-b border-divider text-left text-xs font-bold text-ink">
            <th class="px-3 py-3">{{ t("governance.menus.table.menu") }}</th>
            <th class="px-3 py-3">{{ t("governance.menus.table.path") }}</th>
            <th class="px-3 py-3">{{ t("governance.menus.table.group") }}</th>
            <th class="px-3 py-3 text-center">{{ t("governance.menus.table.visible") }}</th>
            <th class="px-3 py-3 text-center">{{ t("governance.menus.table.actions") }}</th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="item in ctrl.filteredItems.value"
            :key="item.key"
            class="cursor-pointer border-b border-divider transition hover:bg-canvas"
            @click="openEdit(item.key)"
          >
            <td class="px-3 py-2.5">
              <div class="flex items-center gap-2">
                <i :class="`pi ${item.icon} text-muted`" />
                <span class="text-sm font-semibold text-ink">{{ item.title }}</span>
              </div>
              <span class="text-xs text-muted">{{ item.key }}</span>
            </td>
            <td class="px-3 py-2.5 font-mono text-sm text-secondary">{{ item.path }}</td>
            <td class="px-3 py-2.5">
              <span :class="groupBadgeClass(item.group)">
                {{ item.group }}
              </span>
            </td>
            <td class="px-3 py-2.5 text-center">
              <IconActionButton
                :icon="item.visible ? 'pi pi-eye' : 'pi pi-eye-slash'"
                :title="item.visible ? t('governance.menus.table.hideMenu') : t('governance.menus.table.showMenu')"
                :class="item.visible ? 'text-brand' : 'text-muted'"
                @click.stop="ctrl.toggleVisibility(item.key)"
              />
            </td>
            <td class="px-3 py-2.5 text-center">
              <div class="flex items-center justify-center gap-1">
                <IconActionButton icon="pi pi-chevron-up" :title="t('governance.menus.table.moveUp')" @click.stop="ctrl.moveUp(item.key)" />
                <IconActionButton icon="pi pi-chevron-down" :title="t('governance.menus.table.moveDown')" @click.stop="ctrl.moveDown(item.key)" />
              </div>
            </td>
          </tr>
        </tbody>
      </table>
      <p v-if="ctrl.filteredItems.value.length === 0" class="p-6 text-center text-sm text-muted">{{ t("governance.menus.table.empty") }}</p>
    </section>

    <!-- Edit dialog -->
    <GovernanceMenuEditDialog v-model:visible="isEditing" :ctrl="ctrl" @browse-icons="showIconPicker = true" />

    <!-- Create dialog -->
    <GovernanceMenuCreateDialog v-model:visible="isCreating" :ctrl="ctrl" @browse-icons="showIconPicker = true" />

    <!-- Icon Picker Dialog -->
    <IconPickerDialog
      :visible="showIconPicker"
      :selected="ctrl.draft.value?.icon"
      @update:visible="showIconPicker = $event"
      @select="(icon: string) => ctrl.updateDraft('icon', icon)"
    />
  </section>
</template>
