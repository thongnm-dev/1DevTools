<script setup lang="ts">
import { watch } from "vue";
import { useI18n } from "vue-i18n";
import Dialog from "primevue/dialog";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { GitApi } from "../composables/useGit";

const { t } = useI18n();

const props = defineProps<{ git: GitApi }>();
const visible = defineModel<boolean>("visible", { default: false });

watch(visible, (v) => {
  if (v) props.git.loadConflicts();
});

async function doFinishConflict() {
  await props.git.finishConflict();
  if (!props.git.conflicts.value.length) visible.value = false;
}
</script>

<template>
  <Dialog v-model:visible="visible" modal :header="t('git.dialogs.conflict.title')" :style="{ width: '640px' }">
    <div class="flex flex-col gap-2">
      <p class="text-xs text-muted">
        {{ t('git.dialogs.conflict.helpText') }}
      </p>
      <div v-if="!git.conflicts.value.length" class="p-5 text-center text-sm text-muted">
        <i class="pi pi-check-circle mr-1.5 text-emerald-500" /> {{ t('git.dialogs.conflict.noConflicts') }}
      </div>
      <div v-else class="max-h-80 overflow-y-auto rounded-md border border-divider">
        <div
          v-for="f in git.conflicts.value"
          :key="f"
          class="flex items-center gap-2 border-b border-divider-light px-2.5 py-2 last:border-0"
        >
          <i class="pi pi-exclamation-triangle shrink-0 text-xs text-red-500" />
          <span class="min-w-0 flex-1 truncate font-mono text-xs text-ink" :title="f">{{ f }}</span>
          <button
            class="flex shrink-0 items-center gap-1 rounded border border-divider px-2 py-0.5 text-[11px] text-secondary transition-colors hover:border-brand hover:text-brand"
            :title="t('git.dialogs.conflict.keepHeadTitle')"
            @click="git.resolveConflict(f, 'ours')"
          >
            <i class="pi pi-arrow-left text-[10px]" /> {{ t('git.dialogs.conflict.keepHead') }}
          </button>
          <button
            class="flex shrink-0 items-center gap-1 rounded border border-divider px-2 py-0.5 text-[11px] text-secondary transition-colors hover:border-brand hover:text-brand"
            :title="t('git.dialogs.conflict.keepTheirsTitle')"
            @click="git.resolveConflict(f, 'theirs')"
          >
            <i class="pi pi-arrow-right text-[10px]" /> {{ t('git.dialogs.conflict.keepTheirs') }}
          </button>
          <button
            class="flex shrink-0 items-center gap-1 rounded border border-divider px-2 py-0.5 text-[11px] text-secondary transition-colors hover:border-brand hover:text-brand"
            :title="t('git.dialogs.conflict.markResolvedTitle')"
            @click="git.markResolved(f)"
          >
            <i class="pi pi-check text-[10px]" /> {{ t('git.dialogs.conflict.markResolved') }}
          </button>
        </div>
      </div>
    </div>
    <template #footer>
      <DialogFooter
        :cancel-label="t('git.dialogs.conflict.close')"
        cancel-icon="pi pi-times"
        cancel-severity="warn"
        :confirm-label="t('git.dialogs.conflict.confirm')"
        confirm-icon="pi pi-check"
        :confirm-disabled="!!git.conflicts.value.length || !!git.busyMessage.value"
        @cancel="visible = false"
        @confirm="doFinishConflict"
      />
    </template>
  </Dialog>
</template>
