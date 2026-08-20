<script setup lang="ts">
import { useI18n } from "vue-i18n";
import Dialog from "primevue/dialog";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { GovernanceUsersApi } from "../composables/useGovernanceUsers";

const props = defineProps<{ ctrl: GovernanceUsersApi; userId: number | null }>();
const visible = defineModel<boolean>("visible", { default: false });

const { t } = useI18n();

async function executeDelete() {
  if (props.userId !== null) {
    await props.ctrl.removeUser(props.userId);
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
      <h3 class="section-title">{{ t("governance.users.dialog.deleteTitle") }}</h3>
    </template>
    <p class="text-sm text-secondary">{{ t("governance.users.dialog.deleteMessage") }}</p>
    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="t('common.delete')"
        confirm-icon="pi pi-trash"
        confirm-severity="danger"
        @cancel="visible = false"
        @confirm="executeDelete"
      />
    </template>
  </Dialog>
</template>
