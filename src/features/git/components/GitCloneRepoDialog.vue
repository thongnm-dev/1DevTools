<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { open } from "@tauri-apps/plugin-dialog";
import Dialog from "primevue/dialog";
import InputText from "primevue/inputtext";
import InputGroup from "primevue/inputgroup";
import Button from "primevue/button";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { GitApi } from "../composables/useGit";

const { t } = useI18n();

const props = defineProps<{ git: GitApi }>();
const visible = defineModel<boolean>("visible", { default: false });

const cloneUrl = ref("");
const cloneDestParent = ref("");

watch(visible, (v) => {
  if (v) {
    cloneUrl.value = "";
    cloneDestParent.value = "";
  }
});

async function pickCloneDest() {
  const picked = await open({ directory: true, title: t("git.dialogs.cloneRepo.pickDestDialogTitle") });
  if (picked && typeof picked === "string") cloneDestParent.value = picked;
}

const canClone = computed(() => !!cloneUrl.value.trim() && !!cloneDestParent.value.trim());

async function doClone() {
  if (!canClone.value) return;
  const ok = await props.git.cloneRepo(cloneUrl.value, cloneDestParent.value);
  if (ok) {
    visible.value = false;
    cloneUrl.value = "";
    cloneDestParent.value = "";
  }
}
</script>

<template>
  <Dialog v-model:visible="visible" modal :header="t('git.dialogs.cloneRepo.title')" :style="{ width: '460px' }">
    <div class="flex flex-col gap-3">
      <div>
        <label class="mb-1 block text-xs font-bold text-muted">{{ t('git.dialogs.cloneRepo.urlLabel') }}</label>
        <InputText
          v-model="cloneUrl"
          placeholder="https://github.com/user/repo.git"
          class="w-full"
          @keydown.enter="doClone"
        />
      </div>
      <div>
        <label class="mb-1 block text-xs font-bold text-muted">{{ t('git.dialogs.cloneRepo.destLabel') }}</label>
        <InputGroup class="h-8">
          <InputText
            :model-value="cloneDestParent"
            readonly
            :placeholder="t('git.dialogs.cloneRepo.selectDestPlaceholder')"
          />
          <Button
            icon="pi pi-folder-open"
            severity="secondary"
            outlined
            :title="t('git.dialogs.cloneRepo.pickDestButtonTitle')"
            @click="pickCloneDest"
          />
          <Button
            v-if="cloneDestParent"
            icon="pi pi-times"
            severity="danger"
            text
            :title="t('git.dialogs.cloneRepo.clearPathTitle')"
            @click="cloneDestParent = ''"
          />
        </InputGroup>
      </div>
      <p class="text-xs text-muted">{{ t('git.dialogs.cloneRepo.hint') }}</p>
    </div>
    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="t('git.dialogs.cloneRepo.confirm')"
        confirm-icon="pi pi-cloud-download"
        :busy="git.syncing.value"
        :confirm-disabled="!canClone"
        @cancel="visible = false"
        @confirm="doClone"
      />
    </template>
  </Dialog>
</template>
