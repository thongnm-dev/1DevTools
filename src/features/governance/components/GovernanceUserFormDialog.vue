<script setup lang="ts">
import { useI18n } from "vue-i18n";
import Dialog from "primevue/dialog";
import Button from "primevue/button";
import Password from "primevue/password";
import InputText from "primevue/inputtext";
import ToggleChip from "@/shared/components/ToggleChip.vue";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import { useToast } from "@/shared/composables/useToast";
import type { GovernanceUsersApi } from "../composables/useGovernanceUsers";

const props = defineProps<{ ctrl: GovernanceUsersApi }>();
const visible = defineModel<boolean>("visible", { default: false });

const { t } = useI18n();
const toast = useToast();

async function saveAndClose() {
  if (await props.ctrl.saveDraft()) {
    toast.success(props.ctrl.isCreating.value ? t("governance.users.toast.created") : t("governance.users.toast.updated"));
    visible.value = false;
  }
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
        <h3 class="section-title">{{ ctrl.isCreating.value ? t("governance.users.dialog.addTitle") : t("governance.users.dialog.editTitle") }}</h3>
        <p v-if="ctrl.draft.value && !ctrl.isCreating.value" class="mt-1 text-sm text-muted">
          {{ t("governance.users.dialog.idSubtitle", { id: ctrl.draft.value.id, username: ctrl.draft.value.username }) }}
        </p>
      </div>
    </template>

    <div v-if="ctrl.draft.value" class="space-y-4">
      <div class="grid gap-4 md:grid-cols-2">
        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("governance.users.form.username") }} <span class="text-red-500">*</span></span>
          <InputText
            class="mt-1 w-full"
            :model-value="ctrl.draft.value.username"
            :placeholder="t('governance.users.form.usernamePlaceholder')"
            autofocus
            :disabled="!ctrl.isCreating.value"
            @update:model-value="ctrl.updateDraft('username', $event as string)"
          />
        </label>
        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("governance.users.form.fullName") }} <span class="text-red-500">*</span></span>
          <InputText
            class="mt-1 w-full"
            :model-value="ctrl.draft.value.fullName"
            :placeholder="t('governance.users.form.fullNamePlaceholder')"
            @update:model-value="ctrl.updateDraft('fullName', $event as string)"
          />
        </label>
      </div>

      <label v-if="ctrl.isCreating.value" class="block">
        <span class="text-xs font-bold text-muted">{{ t("governance.users.form.password") }} <span class="text-red-500">*</span></span>
        <Password
          class="mt-1 w-full"
          input-class="w-full"
          :model-value="ctrl.draft.value.password"
          :placeholder="t('governance.users.form.password')"
          :feedback="false"
          toggle-mask
          @update:model-value="ctrl.updateDraft('password', $event as string)"
        />
      </label>

      <div class="grid gap-4 md:grid-cols-2">
        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("governance.users.form.email") }}</span>
          <InputText
            class="mt-1 w-full"
            :model-value="ctrl.draft.value.email"
            placeholder="mail@example.com"
            @update:model-value="ctrl.updateDraft('email', $event as string)"
          />
        </label>
        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("governance.users.form.phone") }}</span>
          <InputText
            class="mt-1 w-full"
            :model-value="ctrl.draft.value.phone"
            :placeholder="t('governance.users.form.phonePlaceholder')"
            @update:model-value="ctrl.updateDraft('phone', $event as string)"
          />
        </label>
      </div>

      <label class="block">
        <span class="text-xs font-bold text-muted">{{ t("governance.users.form.position") }}</span>
        <InputText
          class="mt-1 w-full"
          :model-value="ctrl.draft.value.position"
          :placeholder="t('governance.users.form.positionPlaceholder')"
          @update:model-value="ctrl.updateDraft('position', $event as string)"
        />
      </label>

      <div>
        <span class="text-xs font-bold text-muted">{{ t("governance.users.form.roles") }}</span>
        <div class="mt-1 flex flex-wrap gap-2">
          <Button
            v-for="role in ctrl.availableRoles.value"
            :key="role"
            :class="[
              'flex h-9 items-center gap-1.5 rounded-md border px-3 text-sm font-semibold transition',
              ctrl.draft.value.roles.includes(role)
                ? 'border-brand bg-emerald-50 text-brand dark:bg-emerald-950'
                : 'border-divider bg-panel text-secondary hover:border-brand',
            ]"
            unstyled
            @click="ctrl.toggleDraftRole(role)"
          >
            <i :class="['pi text-xs', ctrl.draft.value.roles.includes(role) ? 'pi-check-circle' : 'pi-circle']" />
            {{ role }}
          </Button>
        </div>
      </div>

      <div v-if="!ctrl.isCreating.value">
        <span class="text-xs font-bold text-muted">{{ t("governance.users.form.status") }}</span>
        <div class="mt-1 grid grid-cols-2 rounded-md border border-divider bg-canvas p-1">
          <ToggleChip
            variant="segment"
            :active="ctrl.draft.value.isActive"
            icon="pi-check-circle"
            :label="t('governance.users.status.active')"
            @click="ctrl.updateDraft('isActive', true)"
          />
          <ToggleChip
            variant="segment"
            :active="!ctrl.draft.value.isActive"
            icon="pi-minus-circle"
            :label="t('governance.users.status.inactive')"
            @click="ctrl.updateDraft('isActive', false)"
          />
        </div>
      </div>
    </div>

    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="ctrl.isCreating.value ? t('governance.users.actions.create') : t('common.save')"
        :confirm-icon="ctrl.isCreating.value ? 'pi pi-plus' : 'pi pi-save'"
        @cancel="visible = false"
        @confirm="saveAndClose"
      />
    </template>
  </Dialog>
</template>
