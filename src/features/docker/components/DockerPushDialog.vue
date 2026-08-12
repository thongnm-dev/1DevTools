<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import Dialog from "primevue/dialog";
import InputText from "primevue/inputtext";
import Button from "primevue/button";
import DialogFooter from "@/shared/components/DialogFooter.vue";

export interface PushPayload {
  sourceImage: string;
  targetImage: string;
}

const props = defineProps<{
  sourceImage: string;
}>();

const visible = defineModel<boolean>("visible", { default: false });
const emit = defineEmits<{ push: [payload: PushPayload] }>();

const { t } = useI18n();

const targetImage = ref("");

watch(visible, (v) => {
  if (!v) return;
  targetImage.value = props.sourceImage;
});

const canPush = computed(() => targetImage.value.trim().length > 0);

function doPush() {
  if (!canPush.value) return;
  emit("push", {
    sourceImage: props.sourceImage,
    targetImage: targetImage.value.trim(),
  });
  visible.value = false;
}
</script>

<template>
  <Dialog v-model:visible="visible" modal :header="t('docker.pushDialog.title')" :style="{ width: '460px' }">
    <div class="flex flex-col gap-3">
      <div>
        <label class="mb-1 block text-xs font-bold text-muted">{{ t("docker.pushDialog.sourceLabel") }}</label>
        <InputText :model-value="sourceImage" readonly class="w-full" />
      </div>
      <div>
        <label class="mb-1 block text-xs font-bold text-muted">{{ t("docker.pushDialog.targetLabel") }}</label>
        <InputText v-model="targetImage" class="w-full" placeholder="username/myapp:latest" />
        <p class="mt-1 text-[10px] text-muted">{{ t("docker.pushDialog.targetHint") }}</p>
      </div>
    </div>
    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="t('docker.pushDialog.confirm')"
        confirm-icon="pi pi-cloud-upload"
        :confirm-disabled="!canPush"
        @cancel="visible = false"
        @confirm="doPush"
      />
    </template>
  </Dialog>
</template>
