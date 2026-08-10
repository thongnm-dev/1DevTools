<script setup lang="ts">
import { useI18n } from "vue-i18n";
import Dialog from "primevue/dialog";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import { useToast } from "@/shared/composables/useToast";
import type { GovernanceRolesApi } from "../composables/useGovernanceRoles";

const props = defineProps<{ ctrl: GovernanceRolesApi; roleId: number | null }>();
const visible = defineModel<boolean>("visible", { default: false });

const { t } = useI18n();
const toast = useToast();

async function executeDelete() {
  if (props.roleId !== null) {
    const ok = await props.ctrl.removeRole(props.roleId);
    if (ok) toast.success(t("governance.roles.toast.deleted"));
    visible.value = false;
  }
}
</script>

<template>
  <Dialog
    :visible="visible"
    class="w-full max-w-sm rounded-lg bg-panel shadow-xl"
    :closable="true"
    modal
    @update:visible="visible = $event"
  >
    <template #header>
      <h3 class="section-title">{{ t("governance.roles.dialog.deleteTitle") }}</h3>
    </template>
    <p class="text-sm text-secondary">{{ t("governance.roles.dialog.deleteMessage") }}</p>
    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="t('governance.roles.actions.delete')"
        confirm-icon="pi pi-trash"
        confirm-severity="danger"
        @cancel="visible = false"
        @confirm="executeDelete"
      />
    </template>
  </Dialog>
</template>
