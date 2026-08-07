<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import Dialog from "primevue/dialog";
import InputText from "primevue/inputtext";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { GitApi } from "../composables/useGit";

const props = defineProps<{ git: GitApi }>();
const visible = defineModel<boolean>("visible", { default: false });
const { t } = useI18n();

const newBranchName = ref("");

async function doCreateBranch() {
  const name = newBranchName.value.trim();
  if (!name) return;
  await props.git.createBranch(name);
  visible.value = false;
  newBranchName.value = "";
}
</script>

<template>
  <Dialog v-model:visible="visible" modal :header="t('git.dialogs.newBranch.title')" :style="{ width: '420px' }">
    <div class="flex flex-col gap-3">
      <label class="text-xs font-bold text-muted">{{ t('git.dialogs.newBranch.nameLabel') }}</label>
      <InputText
        v-model="newBranchName"
        :placeholder="t('git.dialogs.newBranch.namePlaceholder')"
        class="w-full"
        @keydown.enter="doCreateBranch"
      />
      <p class="text-xs text-muted">
        {{ t('git.dialogs.newBranch.fromCurrent', { name: git.info.value?.current_branch }) }}
      </p>
    </div>
    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="t('git.dialogs.newBranch.confirm')"
        confirm-icon="pi pi-plus"
        :confirm-disabled="!newBranchName.trim()"
        @cancel="visible = false"
        @confirm="doCreateBranch"
      />
    </template>
  </Dialog>
</template>
