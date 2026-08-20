<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import Dialog from "primevue/dialog";
import InputText from "primevue/inputtext";
import Textarea from "primevue/textarea";
import Select from "primevue/select";
import ToggleChip from "@/shared/components/ToggleChip.vue";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { AgentProviderModelApi } from "../composables/useAgentProviderModel";

const props = defineProps<{ ctrl: AgentProviderModelApi }>();
const visible = defineModel<boolean>("visible", { default: false });

const { t } = useI18n();

const providerOptions = computed(() =>
  props.ctrl.providers.value.map((p) => ({ label: p.name, value: p.id })),
);

async function saveAndClose() {
  if (await props.ctrl.saveDraft()) {
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
        <h3 class="section-title">
          {{ ctrl.isCreating.value ? t("agentProviderModel.dialog.registerTitle") : t("agentProviderModel.dialog.editTitle") }}
        </h3>
        <p class="mt-0.5 text-xs text-muted">{{ t("agentProviderModel.dialog.subtitle") }}</p>
      </div>
    </template>

    <div class="space-y-4">
      <label class="block">
        <span class="text-xs font-bold text-muted">{{ t("agentProviderModel.form.provider") }} <span class="text-red-500">*</span></span>
        <Select
          class="mt-1 w-full"
          :options="providerOptions"
          option-label="label"
          option-value="value"
          :placeholder="t('agentProviderModel.form.providerPlaceholder')"
          :model-value="ctrl.draft.value.provider_id || null"
          @update:model-value="ctrl.updateDraft('provider_id', $event as number)"
        />
      </label>

      <div class="grid gap-4 md:grid-cols-2">
        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("agentProviderModel.form.name") }} <span class="text-red-500">*</span></span>
          <InputText
            class="mt-1 w-full"
            :model-value="ctrl.draft.value.name"
            :placeholder="t('agentProviderModel.form.namePlaceholder')"
            autofocus
            @update:model-value="ctrl.updateDraft('name', ($event as string) ?? '')"
          />
        </label>
        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("agentProviderModel.form.version") }}</span>
          <InputText
            class="mt-1 w-full"
            :model-value="ctrl.draft.value.version"
            :placeholder="t('agentProviderModel.form.versionPlaceholder')"
            @update:model-value="ctrl.updateDraft('version', ($event as string) ?? '')"
          />
        </label>
      </div>

      <label class="block">
        <span class="text-xs font-bold text-muted">{{ t("agentProviderModel.form.code") }}</span>
        <InputText
          class="mt-1 w-full font-mono"
          :model-value="ctrl.draft.value.code"
          :placeholder="t('agentProviderModel.form.codePlaceholder')"
          @update:model-value="ctrl.updateDraft('code', ($event as string) ?? '')"
        />
        <span class="mt-1 block text-[11px] text-muted">{{ t("agentProviderModel.form.codeHint") }}</span>
      </label>

      <label class="block">
        <span class="text-xs font-bold text-muted">{{ t("agentProviderModel.form.description") }}</span>
        <Textarea
          :model-value="ctrl.draft.value.description"
          :rows="3"
          class="mt-1 w-full !text-xs"
          :placeholder="t('agentProviderModel.form.descriptionPlaceholder')"
          @update:model-value="ctrl.updateDraft('description', ($event as string) ?? '')"
        />
      </label>

      <div>
        <span class="text-xs font-bold text-muted">{{ t("agentProviderModel.form.status") }}</span>
        <div class="mt-1 grid grid-cols-2 rounded-md border border-divider bg-canvas p-1">
          <ToggleChip
            variant="segment"
            :active="ctrl.draft.value.enabled"
            icon="pi-check-circle"
            :label="t('agentProviderModel.status.enabled')"
            @click="ctrl.updateDraft('enabled', true)"
          />
          <ToggleChip
            variant="segment"
            :active="!ctrl.draft.value.enabled"
            icon="pi-minus-circle"
            :label="t('agentProviderModel.status.disabled')"
            @click="ctrl.updateDraft('enabled', false)"
          />
        </div>
      </div>
    </div>

    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="ctrl.isCreating.value ? t('common.register') : t('common.save')"
        :confirm-icon="ctrl.isCreating.value ? 'pi pi-plus' : 'pi pi-save'"
        :confirm-disabled="!ctrl.draft.value.name.trim() || !ctrl.draft.value.provider_id"
        @cancel="visible = false"
        @confirm="saveAndClose"
      />
    </template>
  </Dialog>
</template>
