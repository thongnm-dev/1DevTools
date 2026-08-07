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

const message = ref("");
const saving = ref(false);

watch(visible, (v) => {
  if (v) message.value = "";
});

async function confirmStash() {
  saving.value = true;
  try {
    await props.git.stashSave(message.value);
    visible.value = false;
  } finally {
    saving.value = false;
  }
}
</script>

<template>
  <Dialog v-model:visible="visible" modal :header="t('git.dialogs.stashSave.title')" :style="{ width: '460px' }">
    <div class="flex flex-col gap-3">
      <p class="text-sm text-secondary">
        {{ t('git.dialogs.stashSave.description', { count: git.staged.value.length + git.unstaged.value.length }) }}
      </p>
      <div>
        <label class="mb-1 block text-xs font-bold text-muted">{{ t('git.dialogs.stashSave.messageLabel') }}</label>
        <InputText
          v-model="message"
          :placeholder="t('git.dialogs.stashSave.messagePlaceholder')"
          class="w-full"
          @keydown.enter="confirmStash"
        />
      </div>
    </div>
    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="t('git.dialogs.stashSave.confirm')"
        confirm-icon="pi pi-inbox"
        :busy="saving"
        @cancel="visible = false"
        @confirm="confirmStash"
      />
    </template>
  </Dialog>
</template>
