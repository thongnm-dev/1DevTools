<script setup lang="ts">
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import Dialog from "primevue/dialog";
import InputNumber from "primevue/inputnumber";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { AiUsageApi } from "../composables/useAiUsage";

const props = defineProps<{ ctrl: AiUsageApi }>();
const visible = defineModel<boolean>("visible", { default: false });

const { t } = useI18n();

const thresholdInput = ref<number | null>(10);
const intervalInput = ref<number | null>(60);

watch(visible, (v) => {
  if (!v) return;
  thresholdInput.value = props.ctrl.settings.value.switch_threshold_percent;
  intervalInput.value = props.ctrl.settings.value.poll_interval_secs;
});

async function saveSettings() {
  const ok = await props.ctrl.saveSettings({
    switch_threshold_percent: thresholdInput.value ?? 0,
    poll_interval_secs: Math.max(60, intervalInput.value ?? 300),
  });
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
      <h3 class="section-title">{{ t("aiUsage.settings.header") }}</h3>
    </template>

    <div class="space-y-4">
      <label class="block">
        <span class="text-xs font-bold text-muted">{{ t("aiUsage.settings.threshold") }}</span>
        <InputNumber v-model="thresholdInput" class="mt-1 w-full" :min="0" :max="100" :use-grouping="false" />
        <span class="text-xs text-muted">{{ t("aiUsage.settings.thresholdHint") }}</span>
      </label>
      <label class="block">
        <span class="text-xs font-bold text-muted">{{ t("aiUsage.settings.interval") }}</span>
        <InputNumber v-model="intervalInput" class="mt-1 w-full" :min="60" :use-grouping="false" />
        <span class="text-xs text-muted">{{ t("aiUsage.settings.intervalHint") }}</span>
      </label>
    </div>

    <template #footer>
      <DialogFooter cancel-icon="pi pi-times" cancel-severity="danger" confirm-icon="pi pi-save" @cancel="visible = false" @confirm="saveSettings" />
    </template>
  </Dialog>
</template>
