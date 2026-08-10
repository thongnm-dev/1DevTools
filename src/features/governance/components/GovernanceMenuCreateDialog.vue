<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import Button from "primevue/button";
import Dialog from "primevue/dialog";
import InputNumber from "primevue/inputnumber";
import InputText from "primevue/inputtext";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { GovernanceMenusApi } from "../composables/useGovernanceMenus";

const props = defineProps<{ ctrl: GovernanceMenusApi }>();
const visible = defineModel<boolean>("visible", { default: false });
defineEmits<{ "browse-icons": [] }>();

const { t } = useI18n();

const createKeyInvalid = computed(() => {
  const key = props.ctrl.draft.value?.key.trim() ?? "";
  return key.length > 0 && props.ctrl.keyExists(key);
});

const canCreate = computed(() => {
  const d = props.ctrl.draft.value;
  return !!d && d.key.trim().length > 0 && d.title.trim().length > 0 && !createKeyInvalid.value;
});

async function createAndClose() {
  const ok = await props.ctrl.createDraft();
  if (ok) visible.value = false;
}
</script>

<template>
  <Dialog
    :visible="visible"
    class="w-full max-w-xl rounded-lg bg-panel shadow-xl"
    :closable="true"
    modal
    @update:visible="visible = $event"
  >
    <template #header>
      <div>
        <h3 class="section-title">{{ t("governance.menus.dialog.createTitle") }}</h3>
        <p class="mt-1 text-sm text-muted">{{ t("governance.menus.dialog.createSubtitle") }}</p>
      </div>
    </template>

    <div v-if="ctrl.draft.value" class="space-y-4">
      <div class="grid gap-4 md:grid-cols-2">
        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("governance.menus.form.key") }} <span class="text-red-500">*</span></span>
          <InputText
            class="mt-1 w-full"
            :class="createKeyInvalid ? 'border-red-500' : ''"
            :model-value="ctrl.draft.value.key"
            :placeholder="t('governance.menus.form.keyPlaceholder')"
            autofocus
            @update:model-value="ctrl.updateDraft('key', ($event as string) ?? '')"
          />
          <span v-if="createKeyInvalid" class="mt-1 block text-xs text-red-500">{{ t("governance.menus.form.keyExists") }}</span>
        </label>

        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("governance.menus.form.title") }} <span class="text-red-500">*</span></span>
          <InputText
            class="mt-1 w-full"
            :model-value="ctrl.draft.value.title"
            :placeholder="t('governance.menus.form.titlePlaceholder')"
            @update:model-value="ctrl.updateDraft('title', ($event as string) ?? '')"
          />
        </label>
      </div>

      <label class="block">
        <span class="text-xs font-bold text-muted">{{ t("governance.menus.form.path") }}</span>
        <InputText
          class="mt-1 w-full"
          :model-value="ctrl.draft.value.path"
          :placeholder="t('governance.menus.form.pathPlaceholder')"
          @update:model-value="ctrl.updateDraft('path', ($event as string) ?? '')"
        />
      </label>

      <div class="grid gap-4 md:grid-cols-2">
        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("governance.menus.form.icon") }}</span>
          <div class="mt-1 flex items-center gap-2">
            <div class="flex h-10 flex-1 items-center gap-2 rounded-md border border-divider bg-panel px-3">
              <i :class="`pi ${ctrl.draft.value.icon} text-muted`" />
              <InputText
                class="embedded-input min-w-0 flex-1 border-0 !bg-transparent !p-0 !text-sm"
                :model-value="ctrl.draft.value.icon"
                placeholder="pi-home"
                @update:model-value="ctrl.updateDraft('icon', ($event as string) ?? '')"
              />
            </div>
            <Button
              icon="pi pi-th-large"
              severity="secondary"
              outlined
              :title="t('governance.menus.form.browseIcons')"
              @click="$emit('browse-icons')"
            />
          </div>
        </label>

        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("governance.menus.form.group") }}</span>
          <InputText
            class="mt-1 w-full"
            :model-value="ctrl.draft.value.group"
            :placeholder="t('governance.menus.form.groupPlaceholder')"
            @update:model-value="ctrl.updateDraft('group', ($event as string) ?? '')"
          />
        </label>

        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("governance.menus.form.order") }}</span>
          <InputNumber
            class="mt-1 w-full"
            :model-value="ctrl.draft.value.order"
            :min="0"
            :useGrouping="false"
            @update:model-value="ctrl.updateDraft('order', $event ?? 0)"
          />
        </label>

        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("governance.menus.form.visible") }}</span>
          <Button
            :icon="ctrl.draft.value.visible ? 'pi pi-eye' : 'pi pi-eye-slash'"
            :label="ctrl.draft.value.visible ? t('governance.menus.form.shownInSidebar') : t('governance.menus.form.hiddenFromSidebar')"
            :class="[
              'mt-1 w-full',
              ctrl.draft.value.visible ? 'border-brand bg-emerald-50 text-brand' : '',
            ]"
            :severity="ctrl.draft.value.visible ? undefined : 'secondary'"
            :outlined="!ctrl.draft.value.visible"
            @click="ctrl.updateDraft('visible', !ctrl.draft.value.visible)"
          />
        </label>
      </div>
    </div>

    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="t('governance.menus.actions.create')"
        confirm-icon="pi pi-plus"
        :confirm-disabled="!canCreate"
        @cancel="visible = false"
        @confirm="createAndClose"
      />
    </template>
  </Dialog>
</template>
