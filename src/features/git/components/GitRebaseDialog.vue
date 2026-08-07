<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import Dialog from "primevue/dialog";
import Select from "primevue/select";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { GitApi } from "../composables/useGit";

const { t } = useI18n();
const props = defineProps<{ git: GitApi }>();
const visible = defineModel<boolean>("visible", { default: false });

const rebaseTarget = ref("");

const rebaseOptions = computed(() =>
  props.git.branches.value
    .filter((b) => !b.is_current)
    .map((b) => ({
      label: b.is_remote ? `${b.name} ${t("git.dialogs.rebase.remoteLabel")}` : b.name,
      value: b.name,
    })),
);

watch(visible, (v) => {
  if (v) rebaseTarget.value = "";
});

async function doRebase() {
  if (!rebaseTarget.value) return;
  await props.git.rebaseOnto(rebaseTarget.value);
  visible.value = false;
  rebaseTarget.value = "";
}
</script>

<template>
  <Dialog v-model:visible="visible" modal :header="t('git.dialogs.rebase.title')" :style="{ width: '460px' }">
    <div class="flex flex-col gap-3">
      <p class="text-sm text-secondary">
        {{ t('git.dialogs.rebase.ontoPrefix') }} <strong class="text-ink">{{ git.info.value?.current_branch }}</strong> {{ t('git.dialogs.rebase.ontoSuffix') }}
      </p>
      <Select
        v-model="rebaseTarget"
        :options="rebaseOptions"
        option-label="label"
        option-value="value"
        :placeholder="t('git.dialogs.rebase.targetPlaceholder')"
        filter
        class="w-full"
      />
      <p class="text-xs text-muted">
        {{ t('git.dialogs.rebase.conflictHint') }}
      </p>
    </div>
    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="t('git.dialogs.rebase.confirmLabel')"
        confirm-icon="pi pi-arrows-v"
        :confirm-disabled="!rebaseTarget || !!git.busyMessage.value"
        @cancel="visible = false"
        @confirm="doRebase"
      />
    </template>
  </Dialog>
</template>
