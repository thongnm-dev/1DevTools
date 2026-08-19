import { ref, computed } from "vue";
import { useI18n } from "vue-i18n";

import { canUseTauriRuntime, friendlyError } from "@/tauri/commands/_base";
import {
  masterDataList,
  masterDataCreate,
  masterDataUpdate,
  masterDataDelete,
} from "@/tauri/commands/master-data";
import type { MasterData, MasterDataRequest } from "@/models/master-data";
import { DEFAULT_MASTER_DATA_ICON } from "@/models/master-data";
import { useToast } from "@/shared/composables/useToast";

interface MasterDataFilters {
  keyword: string;
  keygroup: "All" | string;
}

const defaultFilters = (): MasterDataFilters => ({
  keyword: "",
  keygroup: "All",
});

const emptyDraft = (): MasterDataRequest => ({
  name: "",
  icon: DEFAULT_MASTER_DATA_ICON,
  keygroup: "",
  display_order: 0,
  description: "",
});

export function useMasterData() {
  const { t } = useI18n();
  const toast = useToast();

  const items = ref<MasterData[]>([]);
  const loading = ref(false);
  const error = ref("");

  const filters = ref<MasterDataFilters>(defaultFilters());

  // --- Dialog draft state ---
  const draft = ref<MasterDataRequest>(emptyDraft());
  const editingId = ref<number | null>(null);
  const isCreating = computed(() => editingId.value === null);

  /** Danh sách keygroup có sẵn (distinct) để đổ vào Select điều kiện tìm kiếm. */
  const keygroupOptions = computed(() => {
    const set = new Set<string>();
    for (const m of items.value) {
      if (m.keygroup.trim()) set.add(m.keygroup.trim());
    }
    return [...set].sort((a, b) => a.localeCompare(b));
  });

  const filteredItems = computed(() => {
    let list = [...items.value];
    const f = filters.value;

    if (f.keygroup !== "All") {
      list = list.filter((m) => m.keygroup === f.keygroup);
    }
    if (f.keyword.trim()) {
      const q = f.keyword.trim().toLowerCase();
      list = list.filter(
        (m) =>
          m.name.toLowerCase().includes(q) ||
          m.keygroup.toLowerCase().includes(q) ||
          m.description.toLowerCase().includes(q),
      );
    }
    return list;
  });

  async function loadItems() {
    if (!canUseTauriRuntime()) return;
    loading.value = true;
    error.value = "";
    try {
      items.value = await masterDataList();
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
    // Lọc phía client theo `filteredItems` — điểm neo cho tìm kiếm server sau này.
  }

  function startCreate() {
    editingId.value = null;
    draft.value = emptyDraft();
  }

  function selectItem(id: number) {
    const m = items.value.find((x) => x.id === id);
    if (!m) return;
    editingId.value = id;
    draft.value = {
      name: m.name,
      icon: m.icon,
      keygroup: m.keygroup,
      display_order: m.display_order,
      description: m.description,
    };
  }

  function updateDraft<K extends keyof MasterDataRequest>(
    field: K,
    value: MasterDataRequest[K],
  ) {
    draft.value = { ...draft.value, [field]: value };
  }

  async function saveDraft(): Promise<boolean> {
    if (!draft.value.name.trim()) return false;
    error.value = "";
    const payload: MasterDataRequest = {
      ...draft.value,
      name: draft.value.name.trim(),
      icon: draft.value.icon.trim(),
      keygroup: draft.value.keygroup.trim(),
      description: draft.value.description.trim(),
    };
    try {
      if (isCreating.value) {
        await masterDataCreate(payload);
        toast.success(t("masterData.toast.created"));
      } else {
        await masterDataUpdate(editingId.value as number, payload);
        toast.success(t("masterData.toast.updated"));
      }
      await loadItems();
      return true;
    } catch (e) {
      error.value = friendlyError(e);
      toast.error(error.value);
      return false;
    }
  }

  async function removeItem(id: number): Promise<boolean> {
    try {
      await masterDataDelete(id);
      items.value = items.value.filter((m) => m.id !== id);
      toast.success(t("masterData.toast.deleted"));
      return true;
    } catch (e) {
      toast.error(friendlyError(e));
      return false;
    }
  }

  function init() {
    void loadItems();
  }

  return {
    items,
    filteredItems,
    keygroupOptions,
    loading,
    error,
    filters,
    draft,
    editingId,
    isCreating,
    loadItems,
    resetFilters,
    search,
    startCreate,
    selectItem,
    updateDraft,
    saveDraft,
    removeItem,
    init,
  };
}

export type MasterDataApi = ReturnType<typeof useMasterData>;
