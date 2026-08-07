<script setup lang="ts">
import { nextTick, watch, ref } from "vue";
import { useI18n } from "vue-i18n";
import Dialog from "primevue/dialog";
import Button from "primevue/button";

const props = defineProps<{
  title: string;
  lines: string[];
  status: "running" | "success" | "error";
}>();

const visible = defineModel<boolean>("visible", { default: false });

const { t } = useI18n();
const logEl = ref<HTMLElement | null>(null);

watch(
  () => props.lines.length,
  async () => {
    await nextTick();
    if (logEl.value) logEl.value.scrollTop = logEl.value.scrollHeight;
  },
);
</script>

<template>
  <Dialog v-model:visible="visible" modal :header="title" :style="{ width: '760px' }" :dismissable-mask="false">
    <div class="flex flex-col gap-2">
      <div class="flex items-center gap-2 text-xs">
        <i v-if="status === 'running'" class="pi pi-spinner pi-spin text-brand" />
        <i v-else-if="status === 'success'" class="pi pi-check-circle text-emerald-500" />
        <i v-else class="pi pi-times-circle text-red-500" />
        <span class="text-secondary">
          {{
            status === "running"
              ? t("docker.buildLog.running")
              : status === "success"
                ? t("docker.buildLog.success")
                : t("docker.buildLog.failed")
          }}
        </span>
      </div>
      <pre
        ref="logEl"
        class="h-[440px] w-full overflow-auto whitespace-pre-wrap break-all rounded-md p-3 font-mono text-[11px] leading-relaxed text-slate-200"
        style="background: #0b0f19"
      >{{ lines.join("\n") }}</pre>
    </div>
    <template #footer>
      <Button icon="pi pi-times" :label="t('common.close')" severity="danger" outlined @click="visible = false" />
    </template>
  </Dialog>
</template>
