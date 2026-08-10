<script setup lang="ts">
import { watch } from "vue";
import { useI18n } from "vue-i18n";
import Button from "primevue/button";
import Dialog from "primevue/dialog";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { GitApi } from "../composables/useGit";

const { t } = useI18n();

const props = defineProps<{ git: GitApi }>();
const visible = defineModel<boolean>("visible", { default: false });

watch(visible, (v) => {
  if (v) props.git.refreshStashes();
});
</script>

<template>
  <Dialog v-model:visible="visible" modal :header="t('git.dialogs.stashList.title')" :style="{ width: '620px' }">
    <div class="flex flex-col gap-2">
      <div
        v-for="s in git.stashes.value"
        :key="s.reference"
        class="flex items-center gap-2 rounded-md border border-divider px-3 py-2"
      >
        <i class="pi pi-inbox shrink-0 text-sm text-muted" />
        <div class="min-w-0 flex-1">
          <p class="truncate text-xs text-ink">{{ s.message || s.reference }}</p>
          <p class="font-mono text-[11px] text-muted">{{ s.reference }}</p>
        </div>
        <Button
          size="small"
          text
          rounded
          severity="secondary"
          class="shrink-0"
          :title="t('git.dialogs.stashList.applyKeep')"
          @click="git.stashApply(s.reference, false)"
        >
          <i class="pi pi-replay" />
        </Button>
        <Button
          size="small"
          text
          rounded
          class="shrink-0"
          :title="t('git.dialogs.stashList.applyPop')"
          @click="git.stashApply(s.reference, true)"
        >
          <i class="pi pi-upload" />
        </Button>
        <Button
          size="small"
          text
          rounded
          severity="danger"
          class="shrink-0"
          :title="t('git.dialogs.stashList.drop')"
          @click="git.stashDrop(s.reference)"
        >
          <i class="pi pi-trash" />
        </Button>
      </div>
      <div v-if="!git.stashes.value.length" class="p-4 text-center text-sm text-muted">
        {{ t('git.dialogs.stashList.empty') }}
      </div>
    </div>
    <template #footer>
      <DialogFooter :cancel-label="t('git.dialogs.stashList.close')" cancel-icon="pi pi-times" cancel-severity="danger" hide-confirm @cancel="visible = false" />
    </template>
  </Dialog>
</template>
