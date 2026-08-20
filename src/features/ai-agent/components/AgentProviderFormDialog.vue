<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import Dialog from "primevue/dialog";
import InputText from "primevue/inputtext";
import Textarea from "primevue/textarea";
import Select from "primevue/select";
import ToggleChip from "@/shared/components/ToggleChip.vue";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { AgentProviderApi } from "../composables/useAgentProvider";
import type { AgentProviderType } from "@/models/agent-provider";
import { AGENT_PROVIDER_TYPES, AGENT_PROVIDER_TYPE_META } from "@/models/agent-provider";

const props = defineProps<{ ctrl: AgentProviderApi }>();
const visible = defineModel<boolean>("visible", { default: false });

const { t } = useI18n();

const typeOptions = computed(() =>
  AGENT_PROVIDER_TYPES.map((value) => ({ value, label: t(`agentProvider.type.${value}`) })),
);

// models[] <-> comma separated text
const modelsText = computed({
  get: () => props.ctrl.draft.value.models.join(", "),
  set: (val: string) => props.ctrl.updateDraft("models", val.split(",").map((s) => s.trim()).filter(Boolean)),
});

// presets[] <-> one-per-line text. Giữ dòng trống (preset "chạy không cờ") nhưng
// bỏ khoảng trắng thừa; chỉ loại các dòng chỉ chứa whitespace ở cuối.
const presetsText = computed({
  get: () => props.ctrl.draft.value.presets.join("\n"),
  set: (val: string) =>
    props.ctrl.updateDraft("presets", val.split("\n").map((s) => s.trim())),
});

function onTypeChange(type: AgentProviderType) {
  props.ctrl.updateDraft("provider_type", type);
  // Gợi ý icon theo loại nếu icon còn để mặc định.
  const meta = AGENT_PROVIDER_TYPE_META[type];
  if (meta) props.ctrl.updateDraft("icon", meta.icon);
}

async function saveAndClose() {
  if (await props.ctrl.saveDraft()) {
    visible.value = false;
  }
}
</script>

<template>
  <Dialog
    :visible="visible"
    class="w-full max-w-xl rounded-lg bg-panel shadow-xl"
    :closable="true"
    modal
    @update:visible="visible = $event"
  >
    <template #header>
      <div>
        <h3 class="section-title">
          {{ ctrl.isCreating.value ? t("agentProvider.dialog.registerTitle") : t("agentProvider.dialog.editTitle") }}
        </h3>
        <p class="mt-0.5 text-xs text-muted">{{ t("agentProvider.dialog.subtitle") }}</p>
      </div>
    </template>

    <div class="space-y-4">
      <div class="grid gap-4 md:grid-cols-2">
        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("agentProvider.form.name") }} <span class="text-red-500">*</span></span>
          <InputText
            class="mt-1 w-full"
            :model-value="ctrl.draft.value.name"
            :placeholder="t('agentProvider.form.namePlaceholder')"
            autofocus
            @update:model-value="ctrl.updateDraft('name', ($event as string) ?? '')"
          />
        </label>
        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("agentProvider.form.code") }}</span>
          <InputText
            class="mt-1 w-full"
            :model-value="ctrl.draft.value.code"
            :placeholder="t('agentProvider.form.codePlaceholder')"
            @update:model-value="ctrl.updateDraft('code', ($event as string) ?? '')"
          />
        </label>
      </div>

      <div class="grid gap-4 md:grid-cols-2">
        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("agentProvider.form.type") }}</span>
          <Select
            class="mt-1 w-full"
            :options="typeOptions"
            option-label="label"
            option-value="value"
            :model-value="ctrl.draft.value.provider_type"
            @update:model-value="onTypeChange($event)"
          />
        </label>
        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("agentProvider.form.icon") }}</span>
          <div class="mt-1 flex items-center gap-2 rounded-md border border-divider bg-panel px-3">
            <i :class="[ctrl.draft.value.icon || 'pi pi-android', 'text-brand']" />
            <InputText
              :model-value="ctrl.draft.value.icon"
              class="embedded-input w-full border-0 !bg-transparent !p-0 !py-2 !text-sm"
              placeholder="pi pi-android"
              @update:model-value="ctrl.updateDraft('icon', ($event as string) ?? '')"
            />
          </div>
        </label>
      </div>

      <div class="grid gap-4 md:grid-cols-2">
        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("agentProvider.form.command") }}</span>
          <InputText
            class="mt-1 w-full font-mono"
            :model-value="ctrl.draft.value.command"
            :placeholder="t('agentProvider.form.commandPlaceholder')"
            @update:model-value="ctrl.updateDraft('command', ($event as string) ?? '')"
          />
        </label>
        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("agentProvider.form.website") }}</span>
          <InputText
            class="mt-1 w-full"
            :model-value="ctrl.draft.value.website"
            placeholder="https://..."
            @update:model-value="ctrl.updateDraft('website', ($event as string) ?? '')"
          />
        </label>
      </div>

      <label class="block">
        <span class="text-xs font-bold text-muted">{{ t("agentProvider.form.models") }}</span>
        <InputText v-model="modelsText" class="mt-1 w-full" :placeholder="t('agentProvider.form.modelsPlaceholder')" />
        <span class="mt-1 block text-[11px] text-muted">{{ t("agentProvider.form.modelsHint") }}</span>
      </label>

      <div class="grid gap-4 md:grid-cols-2">
        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("agentProvider.form.modelFlag") }}</span>
          <InputText
            class="mt-1 w-full font-mono"
            :model-value="ctrl.draft.value.model_flag"
            :placeholder="t('agentProvider.form.modelFlagPlaceholder')"
            @update:model-value="ctrl.updateDraft('model_flag', ($event as string) ?? '')"
          />
          <span class="mt-1 block text-[11px] text-muted">{{ t("agentProvider.form.modelFlagHint") }}</span>
        </label>
        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("agentProvider.form.configEnv") }}</span>
          <InputText
            class="mt-1 w-full font-mono"
            :model-value="ctrl.draft.value.config_env"
            :placeholder="t('agentProvider.form.configEnvPlaceholder')"
            @update:model-value="ctrl.updateDraft('config_env', ($event as string) ?? '')"
          />
          <span class="mt-1 block text-[11px] text-muted">{{ t("agentProvider.form.configEnvHint") }}</span>
        </label>
      </div>

      <label class="block">
        <span class="text-xs font-bold text-muted">{{ t("agentProvider.form.presets") }}</span>
        <Textarea v-model="presetsText" :rows="3" class="mt-1 w-full font-mono !text-xs" :placeholder="t('agentProvider.form.presetsPlaceholder')" />
        <span class="mt-1 block text-[11px] text-muted">{{ t("agentProvider.form.presetsHint") }}</span>
      </label>

      <label class="block">
        <span class="text-xs font-bold text-muted">{{ t("agentProvider.form.description") }}</span>
        <Textarea
          :model-value="ctrl.draft.value.description"
          :rows="3"
          class="mt-1 w-full !text-xs"
          :placeholder="t('agentProvider.form.descriptionPlaceholder')"
          @update:model-value="ctrl.updateDraft('description', ($event as string) ?? '')"
        />
      </label>

      <div>
        <span class="text-xs font-bold text-muted">{{ t("agentProvider.form.status") }}</span>
        <div class="mt-1 grid grid-cols-2 rounded-md border border-divider bg-canvas p-1">
          <ToggleChip
            variant="segment"
            :active="ctrl.draft.value.enabled"
            icon="pi-check-circle"
            :label="t('agentProvider.status.enabled')"
            @click="ctrl.updateDraft('enabled', true)"
          />
          <ToggleChip
            variant="segment"
            :active="!ctrl.draft.value.enabled"
            icon="pi-minus-circle"
            :label="t('agentProvider.status.disabled')"
            @click="ctrl.updateDraft('enabled', false)"
          />
        </div>
      </div>
    </div>

    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="ctrl.isCreating.value ? t('common.register') : t('common.save')"
        :confirm-icon="ctrl.isCreating.value ? 'pi pi-plus' : 'pi pi-save'"
        :confirm-disabled="!ctrl.draft.value.name.trim()"
        @cancel="visible = false"
        @confirm="saveAndClose"
      />
    </template>
  </Dialog>
</template>
