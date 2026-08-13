<script setup lang="ts">
import { useI18n } from "vue-i18n";
import Dialog from "primevue/dialog";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { AiUsageApi } from "../composables/useAiUsage";

defineProps<{ ctrl: AiUsageApi }>();
const visible = defineModel<boolean>("visible", { default: false });

const { t } = useI18n();
</script>

<template>
  <Dialog
    :visible="visible"
    class="w-full max-w-lg rounded-lg bg-panel shadow-xl"
    :closable="true"
    modal
    @update:visible="visible = $event"
  >
    <template #header>
      <h3 class="section-title">{{ t("aiUsage.detectDialog.header") }}</h3>
    </template>

    <div class="space-y-3">
      <p class="text-xs text-muted">
        {{ t("aiUsage.detectDialog.intro1") }} <code class="rounded bg-canvas px-1">.claude.json</code> {{ t("aiUsage.detectDialog.intro2") }}
      </p>

      <div
        v-for="login in ctrl.detected.value"
        :key="login.config_dir + login.email"
        class="rounded-lg border border-divider bg-canvas/50 p-3"
      >
        <div class="flex items-center gap-2">
          <i class="pi pi-user text-muted" />
          <span class="truncate font-semibold text-ink" :title="login.email">{{ login.email || t("aiUsage.detectDialog.noEmail") }}</span>
          <span v-if="login.subscription_type" class="shrink-0 badge-info">
            {{ login.subscription_type }}
          </span>
          <span class="ml-auto shrink-0" :class="login.already_added ? 'badge-success' : 'badge-warning'">
            {{ login.already_added ? t("aiUsage.detectDialog.alreadyAdded") : t("aiUsage.detectDialog.new") }}
          </span>
        </div>
        <div class="mt-1.5 grid gap-0.5 text-xs text-muted">
          <span v-if="login.display_name">{{ login.display_name }}</span>
          <span class="truncate font-mono" :title="login.config_dir">
            <i class="pi pi-folder mr-1" />{{ login.config_dir }}
          </span>
          <span v-if="login.token_expires_at">
            <i class="pi pi-clock mr-1" />{{ t("aiUsage.detectDialog.tokenExpires", { time: login.token_expires_at }) }}
          </span>
        </div>
      </div>

      <p v-if="!ctrl.detected.value.length" class="rounded-lg border border-dashed border-divider p-6 text-center text-sm text-muted">
        {{ t("aiUsage.detectDialog.empty") }}
      </p>
    </div>

    <template #footer>
      <DialogFooter
        :cancel-label="t('aiUsage.detectDialog.close')"
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        confirm-icon="pi pi-download"
        :confirm-label="ctrl.isDetecting.value ? t('aiUsage.detectDialog.adding') : t('aiUsage.detectDialog.addNew')"
        :confirm-disabled="ctrl.isDetecting.value || !ctrl.detected.value.some((l) => !l.already_added)"
        @cancel="visible = false"
        @confirm="ctrl.importDetected()"
      />
    </template>
  </Dialog>
</template>
