<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import Button from "primevue/button";
import { useAiUsage } from "../composables/useAiUsage";
import AiUsageMeter from "./AiUsageMeter.vue";
import AiUsageAddAccountDialog from "./AiUsageAddAccountDialog.vue";
import AiUsageSettingsDialog from "./AiUsageSettingsDialog.vue";
import AiUsageDetectDialog from "./AiUsageDetectDialog.vue";
import AiUsageTerminalDialog from "./AiUsageTerminalDialog.vue";
import {
  AI_ACCOUNT_STATUS_META,
  AI_ACCOUNT_TYPE_META,
  AI_PROVIDER_LABEL,
  AI_USAGE_SOURCE_LABEL,
} from "@/models/ai-usage";
import type { AiAccount, AiProvider } from "@/models/ai-usage";

const { t } = useI18n();
const ctrl = useAiUsage();

const isDialogOpen = ref(false);
const showSettings = ref(false);
const showDetect = ref(false);

const showTerminal = ref(false);
const terminalConfigDir = ref("");
const terminalIsLogin = ref(false);

/** Mở dialog dò login local (dò trước rồi hiện). */
async function openDetect() {
  const ok = await ctrl.detectLocal();
  if (ok) showDetect.value = true;
}

/** Mở dialog terminal để login tại config dir do dialog thêm account yêu cầu. */
function onLoginTerminal(configDir: string) {
  terminalConfigDir.value = configDir;
  terminalIsLogin.value = true;
  showTerminal.value = true;
}

function openTerminalDialog(account: AiAccount) {
  terminalConfigDir.value = account.config_dir || "";
  terminalIsLogin.value = false;
  showTerminal.value = true;
}

onMounted(() => {
  void ctrl.start();
});

/** Nhóm account theo provider để hiển thị. */
const groups = computed(() => {
  const map = new Map<AiProvider, AiAccount[]>();
  for (const account of ctrl.accounts.value) {
    const list = map.get(account.provider) ?? [];
    list.push(account);
    map.set(account.provider, list);
  }
  return Array.from(map.entries());
});

function onPriorityChange(account: AiAccount, event: Event) {
  const value = Math.max(1, Number((event.target as HTMLInputElement).value) || 1);
  if (value !== account.priority) {
    void ctrl.updateAccount({ id: account.id, priority: value });
  }
}
</script>

<template>
  <div class="flex flex-1 flex-col gap-4 overflow-auto">
    <div class="rounded-lg border border-divider bg-panel p-6 shadow-sm">
      <div class="flex flex-wrap items-center gap-3">
        <i class="pi pi-chart-bar text-2xl text-muted" />
        <div class="min-w-0">
          <h2 class="page-title">{{ t("aiUsage.title") }}</h2>
          <p class="text-sm text-muted">
            {{ t("aiUsage.subtitle") }}
          </p>
        </div>
        <div class="ml-auto flex shrink-0 items-center gap-2">
          <Button
            icon="pi pi-search"
            :label="t('aiUsage.actions.detectLocal')"
            severity="secondary"
            size="small"
            :loading="ctrl.isDetecting.value"
            :title="t('aiUsage.actions.detectLocalTooltip')"
            @click="openDetect"
          />
          <Button icon="pi pi-cog" :label="t('aiUsage.actions.settings')" severity="secondary" size="small" @click="showSettings = true" />
          <Button icon="pi pi-plus" :label="t('aiUsage.actions.addAccount')" size="small" @click="isDialogOpen = true" />
        </div>
      </div>
    </div>

    <!-- Account groups by provider -->
    <template v-if="ctrl.accounts.value.length">
      <div v-for="[groupProvider, list] in groups" :key="groupProvider" class="space-y-3">
        <h3 class="section-eyebrow px-1">
          {{ AI_PROVIDER_LABEL[groupProvider] }}
        </h3>
        <div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
          <div
            v-for="account in list"
            :key="account.id"
            class="flex flex-col rounded-lg border bg-panel p-5 shadow-sm"
            :class="account.is_active ? 'border-brand ring-1 ring-brand/40' : 'border-divider'"
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
              <Button
                icon="pi pi-trash"
                severity="danger"
                text
                rounded
                size="small"
                :title="t('aiUsage.actions.deleteAccount')"
                @click="ctrl.deleteAccount(account.id)"
              />
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
                <i class="pi pi-sync" />{{ t("aiUsage.card.updated", { time: account.last_checked_at }) }}
              </p>
            </div>

            <!-- Stats -->
            <div class="mt-3 grid grid-cols-2 gap-3 border-t border-divider pt-3 text-xs text-muted">
              <div class="flex items-center gap-1.5" :title="t('aiUsage.card.resetsAt', { time: account.reset_at || t('aiUsage.card.unknown') })">
                <i class="pi pi-refresh" />
                <span class="truncate">{{ t("aiUsage.card.reset") }} {{ account.reset_at || "—" }}</span>
              </div>
              <label class="flex items-center justify-end gap-1.5" :title="t('aiUsage.card.priorityTooltip')">
                <i class="pi pi-sort-amount-down" />
                <span>{{ t("aiUsage.card.priority") }}</span>
                <input
                  type="number"
                  min="1"
                  class="w-14 rounded border border-divider bg-canvas px-1.5 py-0.5 text-right text-ink"
                  :value="account.priority"
                  @change="onPriorityChange(account, $event)"
                />
              </label>
            </div>

            <!-- Actions -->
            <div class="mt-3 flex flex-wrap items-center gap-2">
              <Button
                icon="pi pi-refresh"
                :label="t('aiUsage.actions.refresh')"
                size="small"
                severity="secondary"
                :loading="ctrl.refreshingId.value === account.id"
                @click="ctrl.refreshAccount(account.id)"
              />
              <Button
                icon="pi pi-check-circle"
                :label="t('aiUsage.actions.setActive')"
                size="small"
                :severity="account.is_active ? 'secondary' : undefined"
                :disabled="account.is_active"
                @click="ctrl.setActive(account.id)"
              />
              <Button
                v-if="account.account_type !== 'subscription'"
                icon="pi pi-copy"
                :label="t('aiUsage.actions.copyToken')"
                size="small"
                severity="secondary"
                outlined
                @click="ctrl.copyToken(account.id)"
              />
              <Button
                v-if="account.account_type === 'subscription'"
                icon="pi pi-terminal"
                :label="t('aiUsage.actions.openTerminal')"
                size="small"
                severity="secondary"
                outlined
                @click="openTerminalDialog(account)"
              />
              <Button
                icon="pi pi-exclamation-triangle"
                :label="t('aiUsage.actions.markExhausted')"
                size="small"
                severity="warn"
                text
                :title="t('aiUsage.actions.markExhaustedTooltip')"
                @click="ctrl.reportExhausted(account.id)"
              />
            </div>
          </div>
        </div>
      </div>
    </template>

    <div v-else class="flex flex-1 items-center justify-center rounded-lg border border-dashed border-divider bg-panel/50 p-12">
      <p class="text-sm text-muted">
        {{ ctrl.isLoading.value ? t("aiUsage.empty.loading") : t("aiUsage.empty.none") }}
      </p>
    </div>

    <AiUsageAddAccountDialog v-model:visible="isDialogOpen" :ctrl="ctrl" @login-terminal="onLoginTerminal" />
    <AiUsageSettingsDialog v-model:visible="showSettings" :ctrl="ctrl" />
    <AiUsageDetectDialog v-model:visible="showDetect" :ctrl="ctrl" />
    <AiUsageTerminalDialog v-model:visible="showTerminal" :ctrl="ctrl" :config-dir="terminalConfigDir" :is-login="terminalIsLogin" />
  </div>
</template>
