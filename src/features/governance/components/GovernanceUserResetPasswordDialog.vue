<script setup lang="ts">
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import Dialog from "primevue/dialog";
import Password from "primevue/password";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { GovernanceUsersApi } from "../composables/useGovernanceUsers";

const props = defineProps<{ ctrl: GovernanceUsersApi; userId: number | null }>();
const visible = defineModel<boolean>("visible", { default: false });

const { t } = useI18n();
const resetPwValue = ref("");

watch(visible, (v) => {
  if (v) resetPwValue.value = "";
});

async function executeResetPassword() {
  if (props.userId !== null && resetPwValue.value.trim()) {
    if (await props.ctrl.resetPassword(props.userId, resetPwValue.value)) {
      visible.value = false;
      resetPwValue.value = "";
    }
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
      <h3 class="section-title">{{ t("governance.users.dialog.resetPasswordTitle") }}</h3>
    </template>
    <label class="block">
      <span class="text-xs font-bold text-muted">{{ t("governance.users.form.newPassword") }} <span class="text-red-500">*</span></span>
      <Password
        class="mt-1 w-full"
        input-class="w-full"
        :placeholder="t('governance.users.form.newPasswordPlaceholder')"
        :model-value="resetPwValue"
        :feedback="false"
        toggle-mask
        @update:model-value="resetPwValue = $event as string"
      />
    </label>
    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="t('common.reset')"
        confirm-icon="pi pi-key"
        :confirm-disabled="!resetPwValue.trim()"
        @cancel="visible = false"
        @confirm="executeResetPassword"
      />
    </template>
  </Dialog>
</template>
