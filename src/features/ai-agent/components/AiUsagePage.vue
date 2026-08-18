<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import Button from "primevue/button";
import { useAiUsage } from "../composables/useAiUsage";
import AiAgentProfile from "./AiAgentProfile.vue";
import AiUsageAddAccountDialog from "./AiUsageAddAccountDialog.vue";
import AiUsageSettingsDialog from "./AiUsageSettingsDialog.vue";
import AiUsageDetectDialog from "./AiUsageDetectDialog.vue";
import AiUsageTerminalDialog from "./AiUsageTerminalDialog.vue";
import { AI_PROVIDER_LABEL } from "@/models/ai-usage";
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
        <div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-4">
          <AiAgentProfile
            v-for="account in list"
            :key="account.id"
            :account="account"
            :ctrl="ctrl"
            @open-terminal="openTerminalDialog"
          />
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
