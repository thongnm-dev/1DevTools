<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import Dialog from "primevue/dialog";
import Select from "primevue/select";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { GitApi } from "../composables/useGit";
import { guessBase } from "../utils/gitRefs";

const { t } = useI18n();
const props = defineProps<{ git: GitApi }>();
const visible = defineModel<boolean>("visible", { default: false });

const updateBranchSel = ref("");

const allBranchRefs = computed(() =>
  props.git.branches.value
    .filter((b) => !b.name.endsWith("/HEAD"))
    .map((b) => ({ label: b.is_remote ? `${b.name} (remote)` : b.name, value: b.name })),
);

watch(visible, (v) => {
  if (v) {
    updateBranchSel.value = guessBase(
      props.git.branches.value.map((b) => b.name),
      props.git.info.value?.current_branch || "",
      props.git.info.value?.upstream,
    );
  }
});

async function doUpdateFromMain() {
  if (!updateBranchSel.value) return;
  const ok = await props.git.mergeBranch(updateBranchSel.value, false, "");
  if (ok) visible.value = false;
}
</script>

<template>
  <Dialog v-model:visible="visible" modal :header="t('git.dialogs.updateFromMain.title')" :style="{ width: '460px' }">
    <div class="flex flex-col gap-3">
      <p class="text-sm text-secondary">
        <i18n-t keypath="git.dialogs.updateFromMain.mergeDescription" tag="span">
          <template #branch><strong class="text-ink">{{ git.info.value?.current_branch }}</strong></template>
        </i18n-t>
      </p>
      <Select
        v-model="updateBranchSel"
        :options="allBranchRefs"
        option-label="label"
        option-value="value"
        filter
        class="w-full"
      />
      <p class="text-xs text-muted">
        {{ t('git.dialogs.updateFromMain.conflictHint') }}
      </p>
    </div>
    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="t('git.dialogs.updateFromMain.confirm')"
        confirm-icon="pi pi-arrow-circle-down"
        :confirm-disabled="!updateBranchSel || !!git.busyMessage.value"
        @cancel="visible = false"
        @confirm="doUpdateFromMain"
      />
    </template>
  </Dialog>
</template>
