import { ref, computed } from "vue";
import { useI18n } from "vue-i18n";

import { canUseTauriRuntime, friendlyError } from "@/tauri/commands/_base";
import {
  agentProviderList,
  agentProviderCreate,
  agentProviderUpdate,
  agentProviderDelete,
  agentProviderSetEnabled,
} from "@/tauri/commands/agent-provider";
import type {
  AgentProvider,
  AgentProviderRequest,
  AgentProviderType,
} from "@/models/agent-provider";
import { useToast } from "@/shared/composables/useToast";

interface ProviderFilters {
  keyword: string;
  providerType: "All" | AgentProviderType;
  status: "All" | "enabled" | "disabled";
}

const defaultFilters = (): ProviderFilters => ({
  keyword: "",
  providerType: "All",
  status: "All",
});

const emptyDraft = (): AgentProviderRequest => ({
  name: "",
  code: "",
  provider_type: "claude",
  description: "",
  icon: "pi pi-android",
  command: "",
  website: "",
  models: [],
  enabled: true,
});

export function useAgentProvider() {
  const { t } = useI18n();
  const toast = useToast();

  const providers = ref<AgentProvider[]>([]);
  const loading = ref(false);
  const error = ref("");

  const filters = ref<ProviderFilters>(defaultFilters());

  // --- Dialog draft state ---
  const draft = ref<AgentProviderRequest>(emptyDraft());
  const editingId = ref<number | null>(null);
  const isCreating = computed(() => editingId.value === null);

  const filteredProviders = computed(() => {
    let list = [...providers.value];
    const f = filters.value;

    if (f.providerType !== "All") {
      list = list.filter((p) => p.provider_type === f.providerType);
    }
    if (f.status !== "All") {
      const enabled = f.status === "enabled";
      list = list.filter((p) => p.enabled === enabled);
    }
    if (f.keyword.trim()) {
      const q = f.keyword.trim().toLowerCase();
      list = list.filter(
        (p) =>
          p.name.toLowerCase().includes(q) ||
          p.code.toLowerCase().includes(q) ||
          p.command.toLowerCase().includes(q) ||
          p.description.toLowerCase().includes(q),
      );
    }
    return list;
  });

  async function loadProviders() {
    if (!canUseTauriRuntime()) return;
    loading.value = true;
    error.value = "";
    try {
      providers.value = await agentProviderList();
    } catch (e) {
      error.value = friendlyError(e);
    } finally {
      loading.value = false;
    }
  }

  function resetFilters() {
    filters.value = defaultFilters();
  }

  function search() {
    // Lọc phía client theo `filteredProviders` — điểm neo cho tìm kiếm server sau này.
  }

  function startCreate() {
    editingId.value = null;
    draft.value = emptyDraft();
  }

  function selectProvider(id: number) {
    const p = providers.value.find((x) => x.id === id);
    if (!p) return;
    editingId.value = id;
    draft.value = {
      name: p.name,
      code: p.code,
      provider_type: p.provider_type,
      description: p.description,
      icon: p.icon,
      command: p.command,
      website: p.website,
      models: [...p.models],
      enabled: p.enabled,
    };
  }

  function updateDraft<K extends keyof AgentProviderRequest>(
    field: K,
    value: AgentProviderRequest[K],
  ) {
    draft.value = { ...draft.value, [field]: value };
  }

  async function saveDraft(): Promise<boolean> {
    if (!draft.value.name.trim()) return false;
    error.value = "";
    const payload: AgentProviderRequest = {
      ...draft.value,
      name: draft.value.name.trim(),
      code: draft.value.code.trim(),
      command: draft.value.command.trim(),
      website: draft.value.website.trim(),
      models: draft.value.models.map((m) => m.trim()).filter(Boolean),
    };
    try {
      if (isCreating.value) {
        await agentProviderCreate(payload);
        toast.success(t("agentProvider.toast.created"));
      } else {
        await agentProviderUpdate(editingId.value as number, payload);
        toast.success(t("agentProvider.toast.updated"));
      }
      await loadProviders();
      return true;
    } catch (e) {
      error.value = friendlyError(e);
      toast.error(error.value);
      return false;
    }
  }

  async function toggleEnabled(provider: AgentProvider) {
    try {
      const updated = await agentProviderSetEnabled(provider.id, !provider.enabled);
      const idx = providers.value.findIndex((p) => p.id === provider.id);
      if (idx !== -1) providers.value[idx] = updated;
    } catch (e) {
      toast.error(friendlyError(e));
    }
  }

  async function removeProvider(id: number): Promise<boolean> {
    try {
      await agentProviderDelete(id);
      providers.value = providers.value.filter((p) => p.id !== id);
      toast.success(t("agentProvider.toast.deleted"));
      return true;
    } catch (e) {
      toast.error(friendlyError(e));
      return false;
    }
  }

  function init() {
    void loadProviders();
  }

  return {
    providers,
    filteredProviders,
    loading,
    error,
    filters,
    draft,
    editingId,
    isCreating,
    loadProviders,
    resetFilters,
    search,
    startCreate,
    selectProvider,
    updateDraft,
    saveDraft,
    toggleEnabled,
    removeProvider,
    init,
  };
}

export type AgentProviderApi = ReturnType<typeof useAgentProvider>;
