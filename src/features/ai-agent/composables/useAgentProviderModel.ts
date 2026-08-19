import { ref, computed } from "vue";
import { useI18n } from "vue-i18n";

import { canUseTauriRuntime, friendlyError } from "@/tauri/commands/_base";
import {
  agentProviderModelList,
  agentProviderModelCreate,
  agentProviderModelUpdate,
  agentProviderModelDelete,
  agentProviderModelSetEnabled,
} from "@/tauri/commands/agent-provider-model";
import { agentProviderList } from "@/tauri/commands/agent-provider";
import type {
  AgentProviderModel,
  AgentProviderModelRequest,
} from "@/models/agent-provider-model";
import type { AgentProvider } from "@/models/agent-provider";
import { useToast } from "@/shared/composables/useToast";

interface ModelFilters {
  keyword: string;
  providerId: number | "All";
  status: "All" | "enabled" | "disabled";
}

const defaultFilters = (): ModelFilters => ({
  keyword: "",
  providerId: "All",
  status: "All",
});

const emptyDraft = (providerId: number): AgentProviderModelRequest => ({
  provider_id: providerId,
  name: "",
  code: "",
  version: "",
  description: "",
  enabled: true,
});

export function useAgentProviderModel() {
  const { t } = useI18n();
  const toast = useToast();

  const models = ref<AgentProviderModel[]>([]);
  const providers = ref<AgentProvider[]>([]);
  const loading = ref(false);
  const error = ref("");

  const filters = ref<ModelFilters>(defaultFilters());

  // --- Dialog draft state ---
  const draft = ref<AgentProviderModelRequest>(emptyDraft(0));
  const editingId = ref<number | null>(null);
  const isCreating = computed(() => editingId.value === null);

  const filteredModels = computed(() => {
    let list = [...models.value];
    const f = filters.value;

    if (f.providerId !== "All") {
      list = list.filter((m) => m.provider_id === f.providerId);
    }
    if (f.status !== "All") {
      const enabled = f.status === "enabled";
      list = list.filter((m) => m.enabled === enabled);
    }
    if (f.keyword.trim()) {
      const q = f.keyword.trim().toLowerCase();
      list = list.filter(
        (m) =>
          m.name.toLowerCase().includes(q) ||
          m.code.toLowerCase().includes(q) ||
          m.version.toLowerCase().includes(q) ||
          m.provider_name.toLowerCase().includes(q),
      );
    }
    return list;
  });

  async function loadProviders() {
    try {
      providers.value = await agentProviderList();
    } catch (e) {
      error.value = friendlyError(e);
    }
  }

  async function loadModels() {
    loading.value = true;
    error.value = "";
    try {
      models.value = await agentProviderModelList();
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
    // Lọc phía client theo `filteredModels` — điểm neo cho tìm kiếm server sau này.
  }

  function startCreate() {
    editingId.value = null;
    // Mặc định chọn provider đầu tiên nếu có.
    draft.value = emptyDraft(providers.value[0]?.id ?? 0);
  }

  function selectModel(id: number) {
    const m = models.value.find((x) => x.id === id);
    if (!m) return;
    editingId.value = id;
    draft.value = {
      provider_id: m.provider_id,
      name: m.name,
      code: m.code,
      version: m.version,
      description: m.description,
      enabled: m.enabled,
    };
  }

  function updateDraft<K extends keyof AgentProviderModelRequest>(
    field: K,
    value: AgentProviderModelRequest[K],
  ) {
    draft.value = { ...draft.value, [field]: value };
  }

  async function saveDraft(): Promise<boolean> {
    if (!draft.value.name.trim()) return false;
    if (!draft.value.provider_id) {
      error.value = t("agentProviderModel.errors.providerRequired");
      toast.error(error.value);
      return false;
    }
    error.value = "";
    const payload: AgentProviderModelRequest = {
      ...draft.value,
      name: draft.value.name.trim(),
      code: draft.value.code.trim(),
      version: draft.value.version.trim(),
    };
    try {
      if (isCreating.value) {
        await agentProviderModelCreate(payload);
        toast.success(t("agentProviderModel.toast.created"));
      } else {
        await agentProviderModelUpdate(editingId.value as number, payload);
        toast.success(t("agentProviderModel.toast.updated"));
      }
      await loadModels();
      return true;
    } catch (e) {
      error.value = friendlyError(e);
      toast.error(error.value);
      return false;
    }
  }

  async function toggleEnabled(model: AgentProviderModel) {
    try {
      const updated = await agentProviderModelSetEnabled(model.id, !model.enabled);
      const idx = models.value.findIndex((m) => m.id === model.id);
      if (idx !== -1) models.value[idx] = updated;
    } catch (e) {
      toast.error(friendlyError(e));
    }
  }

  async function removeModel(id: number): Promise<boolean> {
    try {
      await agentProviderModelDelete(id);
      models.value = models.value.filter((m) => m.id !== id);
      toast.success(t("agentProviderModel.toast.deleted"));
      return true;
    } catch (e) {
      toast.error(friendlyError(e));
      return false;
    }
  }

  function init() {
    if (!canUseTauriRuntime()) return;
    void loadProviders();
    void loadModels();
  }

  return {
    models,
    providers,
    filteredModels,
    loading,
    error,
    filters,
    draft,
    editingId,
    isCreating,
    loadModels,
    loadProviders,
    resetFilters,
    search,
    startCreate,
    selectModel,
    updateDraft,
    saveDraft,
    toggleEnabled,
    removeModel,
    init,
  };
}

export type AgentProviderModelApi = ReturnType<typeof useAgentProviderModel>;
