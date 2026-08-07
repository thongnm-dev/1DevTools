<script setup lang="ts">
import Dialog from "primevue/dialog";
import { useI18n } from "vue-i18n";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { GitApi } from "../composables/useGit";

const props = defineProps<{
  git: GitApi;
  target: { files: string[]; label: string } | null;
}>();
const visible = defineModel<boolean>("visible", { default: false });

const { t } = useI18n();

async function confirmDiscard() {
  if (props.target) await props.git.discardFiles(props.target.files);
  visible.value = false;
}
</script>

<template>
  <Dialog v-model:visible="visible" modal :header="t('git.dialogs.discardConfirm.title')" :style="{ width: '420px' }">
    <p class="text-sm text-secondary">
      {{ t('git.dialogs.discardConfirm.confirmPrefix') }} <strong class="text-ink">{{ target?.label }}</strong>{{ t('git.dialogs.discardConfirm.confirmSuffix') }}
    </p>
    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="t('git.dialogs.discardConfirm.confirm')"
        confirm-icon="pi pi-trash"
        confirm-severity="danger"
        @cancel="visible = false"
        @confirm="confirmDiscard"
      />
    </template>
  </Dialog>
</template>
