<script setup lang="ts">
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";
import Select from "primevue/select";
import InputText from "primevue/inputtext";
import { useAiUsage } from "@/features/ai-agent/composables/useAiUsage";
import { useTerminal } from "@/features/terminal/composables/useTerminal";
import { AI_PROVIDER_LABEL } from "@/models/ai-usage";
import type { Workspace } from "@/models/workspace";

const CUSTOM_PRESET = "__custom__";

const AGENT_OPTIONS = [
  {
    key: "claude",
    title: "Claude",
    command: "claude",
    icon: "pi pi-sparkles",
    presets: ["--dangerously-skip-permissions", "--resume", ""],
  },
  {
    key: "codex",
    title: "Codex",
    command: "codex",
    icon: "pi pi-microchip-ai",
    presets: ["--full-auto", "--dangerously-bypass-approvals-and-sandbox", ""],
  },
  {
    key: "copilot",
    title: "Copilot",
    command: "copilot",
    icon: "pi pi-github",
    presets: ["--allow-all-tools", ""],
  },
] as const;

const props = defineProps<{ workspace: Workspace }>();

const { t } = useI18n();
const router = useRouter();
const ctrl = useAiUsage();
const term = useTerminal();

const selectedKey = ref<(typeof AGENT_OPTIONS)[number]["key"]>(AGENT_OPTIONS[0].key);
const selectedPreset = ref<string>(AGENT_OPTIONS[0].presets[0]);
const customArgs = ref("");

const currentAgent = computed(
  () => AGENT_OPTIONS.find((a) => a.key === selectedKey.value) ?? AGENT_OPTIONS[0],
);

const presetOptions = computed(() => [
  ...currentAgent.value.presets.map((p) => ({
    label: p || t("git.dialogs.agentTerminal.noArgs"),
    value: p,
  })),
  { label: t("git.dialogs.agentTerminal.customPreset"), value: CUSTOM_PRESET },
]);

const isCustomPreset = computed(() => selectedPreset.value === CUSTOM_PRESET);

const finalArgs = computed(() =>
  isCustomPreset.value ? customArgs.value.trim() : selectedPreset.value,
);

function selectAgent(agent: (typeof AGENT_OPTIONS)[number]) {
  selectedKey.value = agent.key;
  selectedPreset.value = agent.presets[0];
  customArgs.value = "";
}

function doLaunch() {
  const path = props.workspace.project_path;
  if (!path) return;
  const agent = currentAgent.value;
  const args = finalArgs.value;
  const command = args ? `${agent.command} ${args}` : agent.command;
  term.addTab({ title: `${agent.title} · ${props.workspace.name}`, startDir: path, autoCommand: command });
  void router.push("/terminal");
}
</script>

<template>
  <div class="flex h-full flex-col overflow-hidden bg-sidebar text-sidebar-text">
    <div class="flex shrink-0 items-center border-b border-sidebar-border px-3 py-2">
      <span class="flex-1 truncate text-[11px] font-semibold uppercase tracking-wide">{{ t("workspaces.sidebar.agents") }}</span>
    </div>

    <div class="min-h-0 flex-1 overflow-y-auto">
      <!-- AI accounts list -->
      <div class="p-2">
        <div v-if="ctrl.isLoading.value" class="flex items-center gap-2 px-1 py-2 text-xs text-sidebar-text">
          <i class="pi pi-spinner pi-spin" /> {{ t("common.loading") }}
        </div>
        <p v-else-if="!ctrl.accounts.value.length" class="px-1 py-2 text-xs text-sidebar-text">
          {{ t("workspaces.sidebar.agentsEmpty") }}
        </p>
        <button
          v-for="account in ctrl.accounts.value"
          :key="account.id"
          type="button"
          class="mb-1 flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-left text-xs transition-colors hover:bg-sidebar-hover"
          :class="account.is_active ? 'bg-sidebar-active text-sidebar-text-active' : 'text-sidebar-text'"
          :disabled="account.is_active"
          :title="AI_PROVIDER_LABEL[account.provider]"
          @click="ctrl.setActive(account.id)"
        >
          <i class="pi pi-sparkles shrink-0" />
          <span class="min-w-0 flex-1 truncate">{{ account.name }}</span>
          <i v-if="account.is_active" class="pi pi-check-circle shrink-0 text-brand" />
        </button>
      </div>

      <!-- Launch in terminal -->
      <div class="border-t border-sidebar-border px-3 py-3">
        <p class="mb-2 text-[11px] font-semibold uppercase tracking-wide text-muted">{{ t("git.dialogs.agentTerminal.title") }}</p>

        <div class="mb-2 grid grid-cols-3 gap-1.5">
          <button
            v-for="agent in AGENT_OPTIONS"
            :key="agent.key"
            type="button"
            class="flex flex-col items-center gap-1 rounded-md border py-2 text-[11px] font-medium transition-colors"
            :class="selectedKey === agent.key
              ? 'border-brand bg-brand/10 text-brand'
              : 'border-sidebar-border text-sidebar-text hover:bg-sidebar-hover'"
            @click="selectAgent(agent)"
          >
            <i :class="agent.icon" class="text-sm" />
            {{ agent.title }}
          </button>
        </div>

        <Select
          v-model="selectedPreset"
          :options="presetOptions"
          option-label="label"
          option-value="value"
          class="mb-2 w-full font-mono text-xs"
        />

        <InputText
          v-if="isCustomPreset"
          v-model="customArgs"
          :placeholder="t('git.dialogs.agentTerminal.argsPlaceholder')"
          class="mb-2 w-full font-mono text-xs"
          @keydown.enter="doLaunch"
        />

        <button
          type="button"
          class="flex w-full items-center justify-center gap-1.5 rounded-md bg-brand px-3 py-1.5 text-xs font-medium text-white transition-opacity hover:opacity-90 disabled:opacity-40"
          :disabled="!workspace.project_path"
          @click="doLaunch"
        >
          <i class="pi pi-play text-[10px]" />
          {{ t("git.dialogs.agentTerminal.confirm") }}
        </button>
      </div>
    </div>
  </div>
</template>
