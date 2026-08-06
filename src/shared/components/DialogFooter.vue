<script setup lang="ts">
import { useI18n } from "vue-i18n";
import Button from "primevue/button";

withDefaults(
  defineProps<{
    cancelLabel?: string;
    cancelIcon?: string;
    confirmLabel?: string;
    confirmIcon?: string;
    confirmSeverity?: "secondary" | "danger" | "success" | "warn" | "info" | "contrast";
    confirmDisabled?: boolean;
    cancelDisabled?: boolean;
    hideConfirm?: boolean;
    busy?: boolean;
  }>(),
  {
    cancelLabel: undefined,
    cancelIcon: undefined,
    confirmLabel: undefined,
    confirmIcon: undefined,
    confirmSeverity: undefined,
    confirmDisabled: false,
    cancelDisabled: false,
    hideConfirm: false,
    busy: false,
  },
);

defineEmits<{
  cancel: [];
  confirm: [];
}>();

const { t } = useI18n();
</script>

<template>
  <div class="flex items-center justify-end gap-2">
    <slot name="extra" />
    <Button
      :label="cancelLabel ?? t('common.cancel')"
      :icon="cancelIcon"
      severity="secondary"
      outlined
      :disabled="cancelDisabled || busy"
      @click="$emit('cancel')"
    />
    <Button
      v-if="!hideConfirm"
      :label="confirmLabel ?? t('common.save')"
      :icon="confirmIcon"
      :severity="confirmSeverity"
      :loading="busy"
      :disabled="confirmDisabled"
      @click="$emit('confirm')"
    />
  </div>
</template>
