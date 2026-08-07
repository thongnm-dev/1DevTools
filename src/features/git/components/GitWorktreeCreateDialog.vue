<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import Button from "primevue/button";
import Checkbox from "primevue/checkbox";
import Dialog from "primevue/dialog";
import InputText from "primevue/inputtext";
import InputGroup from "primevue/inputgroup";
import Select from "primevue/select";
import { open } from "@tauri-apps/plugin-dialog";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { GitApi } from "../composables/useGit";

const { t } = useI18n();
const props = defineProps<{ git: GitApi }>();
const visible = defineModel<boolean>("visible", { default: false });

const wtParent = ref("");
const wtFolder = ref("");
const wtCreateNewBranch = ref(false);
const wtExistingBranch = ref("");
const wtNewBranch = ref("");
const wtOpenAfter = ref(true);

const worktreeBranchOptions = computed(() =>
  props.git.localBranches.value.map((b) => ({ label: b.name, value: b.name })),
);

function resetWorktreeForm() {
  wtParent.value = "";
  wtFolder.value = "";
  wtCreateNewBranch.value = false;
  wtExistingBranch.value = props.git.info.value?.current_branch ?? "";
  wtNewBranch.value = "";
  wtOpenAfter.value = true;
}

watch(visible, (v) => {
  if (v) resetWorktreeForm();
});

async function pickWorktreeParent() {
  const picked = await open({ directory: true, title: t("git.dialogs.worktreeCreate.pickParentDialogTitle") });
  if (picked && typeof picked === "string") wtParent.value = picked;
}

function joinPath(parent: string, name: string) {
  const sep = parent.includes("\\") ? "\\" : "/";
  return `${parent.replace(/[/\\]+$/, "")}${sep}${name}`;
}

const worktreeCanCreate = computed(() => {
  if (!wtParent.value.trim()) return false;
  return wtCreateNewBranch.value ? !!wtNewBranch.value.trim() : !!wtExistingBranch.value;
});

async function doWorktreeCreate() {
  if (!worktreeCanCreate.value) return;
  const branchRef = wtCreateNewBranch.value ? wtNewBranch.value.trim() : wtExistingBranch.value;
  const defaultFolder = (branchRef.split(/[/\\]/).pop() || "worktree").trim();
  const folder = (wtFolder.value.trim() || defaultFolder) || "worktree";
  const fullPath = joinPath(wtParent.value, folder);
  const created = await props.git.worktreeAdd(
    fullPath,
    wtCreateNewBranch.value ? "" : wtExistingBranch.value,
    wtCreateNewBranch.value ? wtNewBranch.value.trim() : "",
  );
  if (created) {
    visible.value = false;
    if (wtOpenAfter.value) await props.git.openPathAsRepo(created);
  }
}
</script>

<template>
  <Dialog v-model:visible="visible" modal :header="t('git.dialogs.worktreeCreate.title')" :style="{ width: '520px' }">
    <div class="flex flex-col gap-3">
      <div>
        <label class="mb-1 block text-xs font-bold text-muted">{{ t('git.dialogs.worktreeCreate.parentFolderLabel') }}</label>
        <InputGroup class="h-8">
          <InputText :model-value="wtParent" readonly :placeholder="t('git.dialogs.worktreeCreate.selectFolderPlaceholder')" />
          <Button icon="pi pi-folder-open" severity="secondary" outlined :title="t('git.dialogs.worktreeCreate.pickParentButtonTitle')" @click="pickWorktreeParent" />
          <Button v-if="wtParent" icon="pi pi-times" severity="danger" text :title="t('git.dialogs.worktreeCreate.clearPathTitle')" @click="wtParent = ''" />
        </InputGroup>
      </div>
      <div>
        <label class="mb-1 block text-xs font-bold text-muted">{{ t('git.dialogs.worktreeCreate.folderNameLabel') }}</label>
        <InputText v-model="wtFolder" :placeholder="t('git.dialogs.worktreeCreate.folderNamePlaceholder')" class="w-full" />
      </div>
      <div class="flex items-center gap-2">
        <Checkbox v-model="wtCreateNewBranch" binary input-id="wt-new-branch" />
        <label for="wt-new-branch" class="text-sm text-ink">{{ t('git.dialogs.worktreeCreate.createNewBranchLabel') }}</label>
      </div>
      <div v-if="wtCreateNewBranch">
        <label class="mb-1 block text-xs font-bold text-muted">{{ t('git.dialogs.worktreeCreate.newBranchLabel') }}</label>
        <InputText v-model="wtNewBranch" :placeholder="t('git.dialogs.worktreeCreate.newBranchPlaceholder')" class="w-full" />
      </div>
      <div v-else>
        <label class="mb-1 block text-xs font-bold text-muted">{{ t('git.dialogs.worktreeCreate.existingBranchLabel') }}</label>
        <Select
          v-model="wtExistingBranch"
          :options="worktreeBranchOptions"
          option-label="label"
          option-value="value"
          :placeholder="t('git.dialogs.worktreeCreate.selectBranchPlaceholder')"
          filter
          class="w-full"
        />
      </div>
      <div class="flex items-center gap-2">
        <Checkbox v-model="wtOpenAfter" binary input-id="wt-open-after" />
        <label for="wt-open-after" class="text-sm text-ink">{{ t('git.dialogs.worktreeCreate.openAfterLabel') }}</label>
      </div>
    </div>
    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="t('git.dialogs.worktreeCreate.confirm')"
        confirm-icon="pi pi-clone"
        :confirm-disabled="!worktreeCanCreate || !!git.busyMessage.value"
        @cancel="visible = false"
        @confirm="doWorktreeCreate"
      />
    </template>
  </Dialog>
</template>
