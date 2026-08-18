<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { useAiUsage } from "@/features/ai-agent/composables/useAiUsage";
import AiAgentProfile from "@/features/ai-agent/components/AiAgentProfile.vue";
import { useWorkspaceTerminal } from "../composables/useWorkspaceTerminal";
import type { AiAccount, AiProvider } from "@/models/ai-usage";
import type { Workspace } from "@/models/workspace";

const AGENT_OPTIONS = [
  {
    key: "claude",
    title: "Claude",
    command: "claude",
    icon: "pi pi-sparkles",
    provider: "claude" as AiProvider | null,
    presets: ["--dangerously-skip-permissions", "--resume", ""],
  },
  {
    key: "codex",
    title: "Codex",
    command: "codex",
    icon: "pi pi-microchip-ai",
    provider: "codex" as AiProvider | null,
    presets: ["--full-auto", "--dangerously-bypass-approvals-and-sandbox", ""],
  },
  {
    key: "copilot",
    title: "Copilot",
    command: "copilot",
    icon: "pi pi-github",
    provider: null as AiProvider | null,
    presets: ["--allow-all-tools", ""],
  },
] as const;

const props = defineProps<{ workspace: Workspace }>();
const emit = defineEmits<{ "open-terminal": [] }>();

const { t } = useI18n();
const ctrl = useAiUsage();
const wsTerm = useWorkspaceTerminal();

const selectedKey = ref<(typeof AGENT_OPTIONS)[number]["key"]>(AGENT_OPTIONS[0].key);

const currentAgent = computed(
  () => AGENT_OPTIONS.find((a) => a.key === selectedKey.value) ?? AGENT_OPTIONS[0],
);

// Chỉ hiển thị account đúng provider của agent đang chọn — chọn agent nào thì
// load/active account (profile) của agent đó, thay vì luôn hiện toàn bộ danh sách.
const accountsForAgent = computed(() => {
  const provider = currentAgent.value.provider;
  if (!provider) return [];
  return ctrl.accounts.value.filter((a) => a.provider === provider);
});

function selectAgent(agent: (typeof AGENT_OPTIONS)[number]) {
  selectedKey.value = agent.key;
}

// Panel này trước đây không gọi start() nên danh sách account luôn rỗng —
// cần load account (profile) mỗi khi panel mount để có cái mà active theo agent.
onMounted(() => {
  void ctrl.start();
});

/** Mở terminal cho một account cụ thể: kích hoạt profile rồi mở tab terminal của workspace
 * với command (agent + preset) mà box của account đó đã chọn sẵn. */
function openTerminalForAccount(account: AiAccount, command?: string) {
  if (!account.is_active) void ctrl.setActive(account.id);
  if (!command) return;
  const path = props.workspace.project_path;
  if (!path) return;
  wsTerm.addTab(props.workspace.id, `${currentAgent.value.title} · ${props.workspace.name}`, path, command);
  emit("open-terminal");
}
</script>

<template>
  <div class="flex h-full flex-col overflow-hidden bg-sidebar text-sidebar-text">
    <div class="flex shrink-0 items-center border-b border-sidebar-border px-3 py-2">
      <span class="flex-1 truncate text-[11px] font-semibold uppercase tracking-wide">{{ t("workspaces.sidebar.agents") }}</span>
    </div>

    <div class="min-h-0 flex-1 overflow-y-auto">
      <!-- Launch in terminal -->
      <div class="px-3 py-3">
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

        <!-- Accounts (profiles) available for the currently selected agent -->
        <p class="mb-1 text-[11px] font-semibold uppercase tracking-wide text-muted">
          {{ t("workspaces.sidebar.agents") }} · {{ currentAgent.title }}
        </p>
        <div class="mb-3">
          <div v-if="ctrl.isLoading.value" class="flex items-center gap-2 px-1 py-2 text-xs text-sidebar-text">
            <i class="pi pi-spinner pi-spin" /> {{ t("common.loading") }}
          </div>
          <p v-else-if="!currentAgent.provider" class="px-1 py-2 text-xs text-sidebar-text">
            {{ t("workspaces.sidebar.agentsNotSupported", { agent: currentAgent.title }) }}
          </p>
          <p v-else-if="!accountsForAgent.length" class="px-1 py-2 text-xs text-sidebar-text">
            {{ t("workspaces.sidebar.agentsEmptyForAgent", { agent: currentAgent.title }) }}
          </p>
          <div v-else class="grid gap-4 sm:grid-cols-2 xl:grid-cols-4">
            <AiAgentProfile
              v-for="account in accountsForAgent"
              :key="account.id"
              :account="account"
              :ctrl="ctrl"
              :agent="currentAgent"
              @open-terminal="openTerminalForAccount"
            />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
