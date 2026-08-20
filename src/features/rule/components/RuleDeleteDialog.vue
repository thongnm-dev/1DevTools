<script setup lang="ts">
import { useI18n } from "vue-i18n";
import Dialog from "primevue/dialog";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { Rule } from "@/models/rule";

const props = defineProps<{
  visible: boolean;
  rule: Rule | null;
}>();

const emit = defineEmits<{
  "update:visible": [value: boolean];
  confirm: [];
}>();

const { t } = useI18n();
</script>

<template>
  <Dialog
    :visible="visible"
    class="w-full max-w-sm rounded-lg bg-panel shadow-xl"
    :closable="true"
    modal
    @update:visible="emit('update:visible', $event)"
  >
    <template #header>
      <h3 class="section-title">{{ t("rule.deleteConfirm.title") }}</h3>
    </template>

    <p class="text-sm text-ink">{{ t("rule.deleteConfirm.message", { name: props.rule?.name ?? "" }) }}</p>

    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="t('common.delete')"
        confirm-icon="pi pi-trash"
        confirm-severity="danger"
        @cancel="emit('update:visible', false)"
        @confirm="emit('confirm')"
      />
    </template>
  </Dialog>
</template>
