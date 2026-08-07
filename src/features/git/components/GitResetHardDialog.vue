<script setup lang="ts">
import { useI18n } from "vue-i18n";
import Dialog from "primevue/dialog";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { GitApi } from "../composables/useGit";
import type { GitCommit } from "@/models/git";

const { t } = useI18n();
const props = defineProps<{ git: GitApi; target: GitCommit | null }>();
const visible = defineModel<boolean>("visible", { default: false });

async function doResetHard() {
  if (props.target) await props.git.resetTo(props.target.hash, "hard");
  visible.value = false;
}
</script>

<template>
  <Dialog v-model:visible="visible" modal :header="t('git.dialogs.resetHard.title')" :style="{ width: '470px' }">
    <p class="text-sm text-secondary">
      {{ t('git.dialogs.resetHard.warning', { hash: target?.short_hash }) }}
    </p>
    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="t('git.dialogs.resetHard.confirmLabel')"
        confirm-icon="pi pi-exclamation-triangle"
        confirm-severity="danger"
        :confirm-disabled="!!git.busyMessage.value"
        @cancel="visible = false"
        @confirm="doResetHard"
      />
    </template>
  </Dialog>
</template>
