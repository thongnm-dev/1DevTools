<script setup lang="ts">
import { useI18n } from "vue-i18n";
import Dialog from "primevue/dialog";
import InputText from "primevue/inputtext";
import Textarea from "primevue/textarea";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import { useToast } from "@/shared/composables/useToast";
import type { GovernanceRolesApi } from "../composables/useGovernanceRoles";

const props = defineProps<{ ctrl: GovernanceRolesApi }>();
const visible = defineModel<boolean>("visible", { default: false });

const { t } = useI18n();
const toast = useToast();

async function saveAndClose() {
  if (await props.ctrl.saveDraft()) {
    toast.success(props.ctrl.isCreating.value ? t("governance.roles.toast.created") : t("governance.roles.toast.updated"));
    visible.value = false;
  }
}
</script>

<template>
  <Dialog
    :visible="visible"
    class="w-full max-w-lg rounded-lg bg-panel shadow-xl"
    :closable="true"
    modal
    @update:visible="visible = $event"
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
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="ctrl.isCreating.value ? t('common.create') : t('common.save')"
        :confirm-icon="ctrl.isCreating.value ? 'pi pi-plus' : 'pi pi-save'"
        @cancel="visible = false"
        @confirm="saveAndClose"
      />
    </template>
  </Dialog>
</template>
