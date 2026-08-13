<script setup lang="ts">
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { open } from "@tauri-apps/plugin-dialog";
import Button from "primevue/button";
import Dialog from "primevue/dialog";
import InputGroup from "primevue/inputgroup";
import InputText from "primevue/inputtext";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { AiUsageApi } from "../composables/useAiUsage";

const props = defineProps<{ ctrl: AiUsageApi; configDir: string; isLogin: boolean }>();
const visible = defineModel<boolean>("visible", { default: false });

const { t } = useI18n();
const workDir = ref("");

watch(visible, (v) => {
  if (v) workDir.value = "";
});

async function browseWorkDir() {
  const selected = await open({ directory: true, title: t("aiUsage.dialog.selectWorkDirTitle") });
  if (typeof selected === "string") workDir.value = selected;
}

async function confirmOpenTerminal() {
  const fn = props.isLogin ? props.ctrl.openLogin : props.ctrl.openTerminal;
  const ok = await fn(props.configDir, workDir.value);
  if (ok) visible.value = false;
}
</script>

<template>
  <Dialog
    :visible="visible"
    class="w-full max-w-md rounded-lg bg-panel shadow-xl"
    :closable="true"
    modal
    @update:visible="visible = $event"
  >
    <template #header>
      <h3 class="section-title">{{ t("aiUsage.terminalDialog.header") }}</h3>
    </template>

    <div class="space-y-4">
      <label class="block">
        <span class="text-xs font-bold text-muted">{{ t("aiUsage.terminalDialog.workingDir") }} <span class="text-red-500">*</span></span>
        <InputGroup class="h-8">
          <InputText readonly :placeholder="t('aiUsage.terminalDialog.workingDirPlaceholder')" :model-value="workDir" />
          <Button icon="pi pi-folder-open" severity="secondary" outlined :title="t('aiUsage.addDialog.selectFolder')" @click="browseWorkDir" />
          <Button v-if="workDir" icon="pi pi-times" severity="danger" text :title="t('aiUsage.addDialog.clearPath')" @click="workDir = ''" />
        </InputGroup>
        <span class="text-xs text-muted">{{ t("aiUsage.terminalDialog.workingDirHint") }}</span>
      </label>
    </div>

    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="t('aiUsage.terminalDialog.continue')"
        :confirm-disabled="!workDir.trim()"
        @cancel="visible = false"
        @confirm="confirmOpenTerminal"
      />
    </template>
  </Dialog>
</template>
