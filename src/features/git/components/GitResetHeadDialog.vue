<script setup lang="ts">
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import Dialog from "primevue/dialog";
import InputText from "primevue/inputtext";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { GitApi } from "../composables/useGit";

const { t } = useI18n();
const props = defineProps<{ git: GitApi }>();
const visible = defineModel<boolean>("visible", { default: false });

const resetTarget = ref("HEAD");
const resetMode = ref<"soft" | "mixed" | "hard">("mixed");

watch(visible, (v) => {
  if (v) {
    resetTarget.value = "HEAD";
    resetMode.value = "mixed";
  }
});

async function doResetHead() {
  await props.git.resetTo(resetTarget.value.trim() || "HEAD", resetMode.value);
  visible.value = false;
}
</script>

<template>
  <Dialog v-model:visible="visible" modal :header="t('git.dialogs.resetHead.title')" :style="{ width: '460px' }">
    <div class="flex flex-col gap-3">
      <div>
        <label class="mb-1 block text-xs font-bold text-muted">{{ t('git.dialogs.resetHead.targetLabel') }}</label>
        <InputText v-model="resetTarget" :placeholder="t('git.dialogs.resetHead.targetPlaceholder')" class="w-full" />
      </div>
      <div>
        <label class="mb-1 block text-xs font-bold text-muted">{{ t('git.dialogs.resetHead.modeLabel') }}</label>
        <div class="flex overflow-hidden rounded-md border border-divider text-xs">
          <button
            v-for="m in (['soft','mixed','hard'] as const)"
            :key="m"
            class="flex-1 px-2 py-1.5 font-medium transition-colors"
            :class="resetMode === m ? 'bg-brand text-white' : 'text-secondary hover:bg-canvas'"
            @click="resetMode = m"
          >
            {{ m }}
          </button>
        </div>
        <p class="mt-1 text-xs text-muted">
          <template v-if="resetMode === 'soft'">{{ t('git.dialogs.resetHead.modeSoftHint') }}</template>
          <template v-else-if="resetMode === 'mixed'">{{ t('git.dialogs.resetHead.modeMixedHint') }}</template>
          <template v-else class="text-red-600">{{ t('git.dialogs.resetHead.modeHardHint') }}</template>
        </p>
      </div>
    </div>
    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="t('git.dialogs.resetHead.confirmLabel', { mode: resetMode })"
        confirm-icon="pi pi-backward"
        :confirm-severity="resetMode === 'hard' ? 'danger' : undefined"
        :confirm-disabled="!!git.busyMessage.value"
        @cancel="visible = false"
        @confirm="doResetHead"
      />
    </template>
  </Dialog>
</template>
