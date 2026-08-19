<script setup lang="ts">
import { useI18n } from "vue-i18n";
import Dialog from "primevue/dialog";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { AgentProviderModel } from "@/models/agent-provider-model";

const props = defineProps<{
  model: AgentProviderModel | null;
}>();

const visible = defineModel<boolean>("visible", { default: false });
const emit = defineEmits<{ confirm: [] }>();

const { t } = useI18n();
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
      <h3 class="section-title">{{ t("agentProviderModel.deleteConfirm.title") }}</h3>
    </template>

    <p class="text-sm text-ink">
      {{ t("agentProviderModel.deleteConfirm.message", { name: props.model?.name ?? "" }) }}
    </p>

    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="t('agentProviderModel.actions.delete')"
        confirm-icon="pi pi-trash"
        confirm-severity="danger"
        @cancel="visible = false"
        @confirm="emit('confirm')"
      />
    </template>
  </Dialog>
</template>
