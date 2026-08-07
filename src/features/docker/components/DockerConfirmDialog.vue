<script setup lang="ts">
import Dialog from "primevue/dialog";
import DialogFooter from "@/shared/components/DialogFooter.vue";

withDefaults(
  defineProps<{
    title: string;
    confirmLabel: string;
    busy?: boolean;
  }>(),
  { busy: false },
);

const visible = defineModel<boolean>("visible", { default: false });

const emit = defineEmits<{
  confirm: [];
  cancel: [];
}>();

function onCancel() {
  visible.value = false;
  emit("cancel");
}
</script>

<template>
  <Dialog v-model:visible="visible" modal :header="title" :style="{ width: '440px' }">
    <div class="text-sm text-secondary">
      <slot />
    </div>
    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="confirmLabel"
        confirm-icon="pi pi-trash"
        confirm-severity="danger"
        :busy="busy"
        @cancel="onCancel"
        @confirm="$emit('confirm')"
      />
    </template>
  </Dialog>
</template>
