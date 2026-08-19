<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { useAiUsage } from "@/features/ai-agent/composables/useAiUsage";
import AiAgentProfile from "@/features/ai-agent/components/AiAgentProfile.vue";
import { useWorkspaceTerminal } from "../composables/useWorkspaceTerminal";
import { agentProviderList } from "@/tauri/commands/agent-provider";
import type { AiAccount, AiProvider } from "@/models/ai-usage";
import type { AgentProvider } from "@/models/agent-provider";
import type { Workspace } from "@/models/workspace";

/** Agent để launch terminal — dựng từ 1 bản ghi `agent_providers` (DB). */
type AgentOption = {
  key: string;
  title: string;
  command: string;
  icon: string;
  /** Provider tương ứng của account (chỉ claude/codex có account); null = không map account. */
  provider: AiProvider | null;
  presets: string[];
  /** Biến môi trường trỏ config dir (VD "CLAUDE_CONFIG_DIR"); rỗng = không set. */
  configEnv: string;
};

const props = defineProps<{ workspace: Workspace }>();
const emit = defineEmits<{ "open-terminal": [] }>();

const { t } = useI18n();
const ctrl = useAiUsage();
const wsTerm = useWorkspaceTerminal();

const agents = ref<AgentOption[]>([]);
const selectedKey = ref<string>("");

/** Chỉ claude/codex có hệ thống account (AI Usage); các provider khác → null. */
function toAccountProvider(providerType: string): AiProvider | null {
  return providerType === "claude" || providerType === "codex" ? providerType : null;
}

function toAgentOption(p: AgentProvider): AgentOption {
  return {
    key: p.code || p.provider_type || String(p.id),
    title: p.name,
    command: p.command,
    icon: p.icon || "pi pi-android",
    provider: toAccountProvider(p.provider_type),
    presets: p.presets,
    configEnv: p.config_env,
  };
}

const currentAgent = computed<AgentOption | null>(
  () => agents.value.find((a) => a.key === selectedKey.value) ?? agents.value[0] ?? null,
);

// Chỉ hiển thị account đúng provider của agent đang chọn — chọn agent nào thì
// load/active account (profile) của agent đó, thay vì luôn hiện toàn bộ danh sách.
const accountsForAgent = computed(() => {
  const provider = currentAgent.value?.provider;
  if (!provider) return [];
  return ctrl.accounts.value.filter((a) => a.provider === provider);
});

function selectAgent(agent: AgentOption) {
  selectedKey.value = agent.key;
}

async function loadAgents() {
  try {
    const list = await agentProviderList();
    agents.value = list
      .filter((p) => p.enabled)
      .sort((a, b) => a.id - b.id)
      .map(toAgentOption);
    if (!agents.value.some((a) => a.key === selectedKey.value)) {
      selectedKey.value = agents.value[0]?.key ?? "";
    }
  } catch {
    agents.value = [];
  }
}

// Panel này trước đây không gọi start() nên danh sách account luôn rỗng —
// cần load account (profile) + danh sách agent (DB) mỗi khi panel mount.
onMounted(() => {
  void ctrl.start();
  void loadAgents();
});

/** Xây env vars cho phiên terminal dựa trên provider + account config. */
function buildEnvForAccount(account: AiAccount): Record<string, string> | undefined {
  const agent = currentAgent.value;
  if (!agent?.provider || !agent.configEnv || !account.config_dir) return undefined;
  return { [agent.configEnv]: account.config_dir };
}

/** Mở terminal cho một account cụ thể: kích hoạt profile rồi mở tab terminal của workspace
 * với command (agent + preset) mà box của account đó đã chọn sẵn. */
function openTerminalForAccount(account: AiAccount, command?: string) {
  if (!account.is_active) void ctrl.setActive(account.id);
  if (!command) return;
  const path = props.workspace.project_path;
  if (!path) return;
  const env = buildEnvForAccount(account);
  const title = `${currentAgent.value?.title ?? ""} · ${props.workspace.name}`;
  wsTerm.addTab(props.workspace.id, title, path, command, env);
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
            v-for="agent in agents"
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
          {{ t("workspaces.sidebar.agents") }} · {{ currentAgent?.title ?? "" }}
        </p>
        <div class="mb-3">
          <div v-if="ctrl.isLoading.value" class="flex items-center gap-2 px-1 py-2 text-xs text-sidebar-text">
            <i class="pi pi-spinner pi-spin" /> {{ t("common.loading") }}
          </div>
          <p v-else-if="!currentAgent?.provider" class="px-1 py-2 text-xs text-sidebar-text">
            {{ t("workspaces.sidebar.agentsNotSupported", { agent: currentAgent?.title ?? "" }) }}
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
              :agent="currentAgent ?? undefined"
              @open-terminal="openTerminalForAccount"
            />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
