<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import Checkbox from "primevue/checkbox";
import Dialog from "primevue/dialog";
import InputText from "primevue/inputtext";
import Select from "primevue/select";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { GitApi } from "../composables/useGit";

const props = defineProps<{ git: GitApi }>();
const visible = defineModel<boolean>("visible", { default: false });
const { t } = useI18n();

const mergeBranchSel = ref("");
const mergeSquash = ref(true);
const mergeMessage = ref("");

const mergeableBranches = computed(() =>
  props.git.branches.value
    .filter((b) => !b.is_current && !b.name.endsWith("/HEAD"))
    .map((b) => ({ label: b.is_remote ? `${b.name} (remote)` : b.name, value: b.name })),
);

watch(visible, (v) => {
  if (v) {
    mergeBranchSel.value = "";
    mergeSquash.value = true;
    mergeMessage.value = "";
  }
});

async function doMerge() {
  if (!mergeBranchSel.value) return;
  const ok = await props.git.mergeBranch(mergeBranchSel.value, mergeSquash.value, mergeMessage.value);
  if (ok) visible.value = false;
}
</script>

<template>
  <Dialog v-model:visible="visible" modal :header="t('git.dialogs.merge.title')" :style="{ width: '480px' }">
    <div class="flex flex-col gap-3">
      <p class="text-sm text-secondary">
        {{ t('git.dialogs.merge.descriptionPrefix') }} <strong class="text-ink">{{ git.info.value?.current_branch }}</strong>:
      </p>
      <Select
        v-model="mergeBranchSel"
        :options="mergeableBranches"
        option-label="label"
        option-value="value"
        :placeholder="t('git.dialogs.merge.selectBranchPlaceholder')"
        filter
        class="w-full"
      />
      <div class="flex items-center gap-2">
        <Checkbox v-model="mergeSquash" binary input-id="merge-squash" />
        <label for="merge-squash" class="text-sm text-ink">{{ t('git.dialogs.merge.squashLabel') }}</label>
      </div>
      <div v-if="mergeSquash">
        <label class="mb-1 block text-xs font-bold text-muted">{{ t('git.dialogs.merge.messageLabel') }}</label>
        <InputText
          v-model="mergeMessage"
          :placeholder="t('git.dialogs.merge.squashMessagePlaceholder', { branch: mergeBranchSel || '...' })"
          class="w-full"
        />
      </div>
      <p class="text-xs text-muted">
        {{ t('git.dialogs.merge.conflictHint') }}
      </p>
    </div>
    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="mergeSquash ? t('git.dialogs.merge.confirmSquash') : t('git.dialogs.merge.confirm')"
        confirm-icon="pi pi-code-branch"
        :confirm-disabled="!mergeBranchSel || !!git.busyMessage.value"
        @cancel="visible = false"
        @confirm="doMerge"
      />
    </template>
  </Dialog>
</template>
