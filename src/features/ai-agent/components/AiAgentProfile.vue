<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import Button from "primevue/button";
import Select from "primevue/select";
import InputText from "primevue/inputtext";
import AiUsageMeter from "./AiUsageMeter.vue";
import {
  AI_ACCOUNT_STATUS_META,
  AI_ACCOUNT_TYPE_META,
  AI_USAGE_SOURCE_LABEL,
} from "@/models/ai-usage";
import type { AiAccount } from "@/models/ai-usage";
import type { AiUsageApi } from "../composables/useAiUsage";

const CUSTOM_PRESET = "__custom__";

/** CLI agent (claude/codex/…) dùng để launch terminal — chỉ truyền ở màn Workspace,
 * cho phép mỗi account tự chọn preset tham số riêng trước khi mở terminal. */
type LaunchAgent = { key: string; command: string; presets: readonly string[] };

const props = defineProps<{ account: AiAccount; ctrl: AiUsageApi; agent?: LaunchAgent }>();
const emit = defineEmits<{ "open-terminal": [account: AiAccount, command?: string] }>();

const { t } = useI18n();

const selectedPreset = ref(props.agent?.presets[0] ?? "");
const customArgs = ref("");

// Đổi agent (chuyển tab Claude/Codex/Copilot ở panel cha) → preset của account này reset về mặc định.
watch(
  () => props.agent?.key,
  () => {
    selectedPreset.value = props.agent?.presets[0] ?? "";
    customArgs.value = "";
  },
);

const presetOptions = computed(() => {
  if (!props.agent) return [];
  return [
    ...props.agent.presets.map((p) => ({ label: p || t("git.dialogs.agentTerminal.noArgs"), value: p })),
    { label: t("git.dialogs.agentTerminal.customPreset"), value: CUSTOM_PRESET },
  ];
});

const isCustomPreset = computed(() => selectedPreset.value === CUSTOM_PRESET);

const finalArgs = computed(() => (isCustomPreset.value ? customArgs.value.trim() : selectedPreset.value));

function activate() {
  if (!props.account.is_active) void props.ctrl.setActive(props.account.id);
}

/** Backend trả `YYYY-MM-DD HH:MM:SS` — chỉ cần đổi dấu `-` sang `/` để ra `yyyy/MM/dd HH:mm:ss`. */
function formatDateTime(raw: string): string {
  return raw.replace(/-/g, "/");
}

function launchTerminal() {
  if (!props.agent) {
    emit("open-terminal", props.account);
    return;
  }
  const args = finalArgs.value;
  const command = args ? `${props.agent.command} ${args}` : props.agent.command;
  emit("open-terminal", props.account, command);
}
</script>

<template>
  <div
    class="flex flex-col rounded-lg border bg-panel p-5 shadow-sm transition-colors"
    :class="account.is_active ? 'border-brand ring-1 ring-brand/40' : 'cursor-pointer border-divider hover:border-brand/60'"
    :title="!account.is_active ? t('aiUsage.actions.setActive') : undefined"
    @click="activate"
  >
    <div class="flex items-start gap-3">
      <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-md bg-brand/10 text-brand">
        <i class="pi pi-sparkles" />
      </div>
      <div class="min-w-0 flex-1">
        <div class="flex flex-wrap items-center gap-2">
          <h3 class="section-title truncate" :title="account.name">{{ account.name }}</h3>
          <span v-if="account.is_active" class="shrink-0 rounded-full bg-brand px-2 py-0.5 text-[11px] font-bold text-on-brand">
            {{ t("aiUsage.status.active") }}
          </span>
          <span
            v-if="account.account_type !== 'subscription'"
            :class="['shrink-0', AI_ACCOUNT_TYPE_META[account.account_type].badgeClass]"
          >
            {{ AI_ACCOUNT_TYPE_META[account.account_type].label }}
          </span>
          <span
            v-if="account.subscription_type"
            class="shrink-0 badge-neutral"
          >
            {{ account.subscription_type }}
          </span>
        </div>
        <template v-if="account.account_type === 'subscription'">
          <p v-if="account.email" class="mt-0.5 truncate text-xs text-muted" :title="account.email">
            <i class="pi pi-envelope mr-1" />{{ account.email }}
          </p>
          <p class="mt-0.5 truncate font-mono text-xs text-muted" :title="account.config_dir">
            <i class="pi pi-folder mr-1" />{{ account.config_dir || "—" }}
          </p>
        </template>
        <p v-else class="mt-0.5 font-mono text-xs text-muted">{{ account.api_key_masked }}</p>
      </div>
      <div class="flex shrink-0 items-center gap-1">
        <Button
          icon="pi pi-refresh"
          severity="secondary"
          text
          rounded
          size="small"
          :loading="ctrl.refreshingId.value === account.id"
          :title="t('common.refresh')"
          @click.stop="ctrl.refreshAccount(account.id)"
        />
        <Button
          icon="pi pi-trash"
          severity="danger"
          text
          rounded
          size="small"
          :title="t('aiUsage.actions.deleteAccount')"
          @click.stop="ctrl.deleteAccount(account.id)"
        />
      </div>
    </div>

    <!-- Status + usage source -->
    <div class="mt-3 flex flex-wrap items-center gap-2 text-[11px]">
      <span :class="AI_ACCOUNT_STATUS_META[account.status].badgeClass">
        {{ AI_ACCOUNT_STATUS_META[account.status].label }}
      </span>
      <span class="text-muted">{{ t("aiUsage.card.source") }} {{ AI_USAGE_SOURCE_LABEL[account.usage_source] }}</span>
    </div>

    <!-- Usage used (API / Codex — số liệu tổng hợp) -->
    <div v-if="account.account_type !== 'subscription'" class="mt-3">
      <AiUsageMeter :label="t('aiUsage.meter.usageUsed')" :remaining-percent="account.usage_percent" size="md" />
    </div>

    <!-- Subscription: session (5h) + weekly (7 ngày) từ OAuth usage endpoint -->
    <div v-else class="mt-3 space-y-3">
      <AiUsageMeter
        :label="t('aiUsage.meter.currentSession')"
        :remaining-percent="account.session_percent"
        :reset-at="account.session_reset_at"
        size="md"
      />
      <AiUsageMeter
        :label="t('aiUsage.meter.weeklyLimit')"
        :remaining-percent="account.weekly_percent"
        :reset-at="account.weekly_reset_at"
        size="md"
      />

      <p
        v-if="!account.session_reset_at && !account.weekly_reset_at"
        class="text-xs text-muted"
      >
        {{ t("aiUsage.card.noData") }}
      </p>
      <p v-if="account.last_checked_at" class="flex items-center gap-1 text-xs text-muted">
        <i class="pi pi-sync" />{{ t("aiUsage.card.updated", { time: formatDateTime(account.last_checked_at) }) }}
      </p>
    </div>

    <!-- Preset tham số CLI (chỉ khi có agent — dùng ở màn Workspace) -->
    <div v-if="agent" class="mt-3 flex flex-col gap-2" @click.stop>
      <Select
        v-model="selectedPreset"
        :options="presetOptions"
        option-label="label"
        option-value="value"
        class="w-full font-mono text-muted"
      />
      <InputText
        v-if="isCustomPreset"
        v-model="customArgs"
        :placeholder="t('git.dialogs.agentTerminal.argsPlaceholder')"
        class="w-full font-mono text-muted"
        @keydown.enter="launchTerminal"
      />
    </div>

    <!-- Actions -->
    <div class="mt-3 flex flex-wrap items-center gap-2 border-t border-divider pt-3">
      <Button
        v-if="account.account_type !== 'subscription'"
        icon="pi pi-copy"
        :label="t('aiUsage.actions.copyToken')"
        size="small"
        severity="secondary"
        outlined
        @click.stop="ctrl.copyToken(account.id)"
      />
      <Button
        v-if="agent || account.account_type === 'subscription'"
        icon="pi pi-terminal"
        :label="t('aiUsage.actions.openTerminal')"
        size="small"
        severity="secondary"
        outlined
        @click.stop="launchTerminal"
      />
    </div>
  </div>
</template>
