<script setup lang="ts">
import { useI18n } from "vue-i18n";
import Dialog from "primevue/dialog";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { GitApi } from "../composables/useGit";
import type { GitCommit } from "@/models/git";

const { t } = useI18n();

const props = defineProps<{ git: GitApi; target: GitCommit | null }>();
const visible = defineModel<boolean>("visible", { default: false });

async function doRevert() {
  if (!props.target) return;
  await props.git.revert(props.target.hash);
  visible.value = false;
}
</script>

<template>
  <Dialog v-model:visible="visible" modal :header="t('git.dialogs.revert.title')" :style="{ width: '460px' }">
    <p class="text-sm text-secondary">
      {{ t('git.dialogs.revert.description') }}
    </p>
    <div v-if="target" class="mt-2 rounded-md border border-divider bg-canvas p-2.5">
      <p class="text-sm font-medium text-ink">{{ target.subject }}</p>
      <p class="mt-0.5 font-mono text-[11px] text-muted">{{ target.short_hash }} · {{ target.author_name }}</p>
    </div>
    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="t('git.dialogs.revert.confirm')"
        confirm-icon="pi pi-undo"
        :confirm-disabled="!!git.busyMessage.value"
        @cancel="visible = false"
        @confirm="doRevert"
      />
    </template>
  </Dialog>
</template>
