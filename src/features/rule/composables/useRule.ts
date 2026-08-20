import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";

import { friendlyError } from "@/tauri/commands/_base";
import { ruleCreate, ruleDelete, ruleList, ruleUpdate } from "@/tauri/commands/rule";
import type { Rule, RuleRequest } from "@/models/rule";
import { useToast } from "@/shared/composables/useToast";

interface RuleFilters {
  keyword: string;
}

const defaultFilters = (): RuleFilters => ({ keyword: "" });

export function useRule() {
  const { t } = useI18n();
  const toast = useToast();

  const rules = ref<Rule[]>([]);
  const isLoading = ref(false);
  const error = ref("");
  const filters = ref<RuleFilters>(defaultFilters());
  const sortBy = ref<"name" | "created_at">("created_at");
  const sortDir = ref<"asc" | "desc">("desc");

  const filteredRules = computed(() => {
    const f = filters.value;
    let list = [...rules.value];

    const q = f.keyword.trim().toLowerCase();
    if (q) {
      list = list.filter(
        (r) => r.name.toLowerCase().includes(q) || r.description.toLowerCase().includes(q) || r.tags.some((tag) => tag.toLowerCase().includes(q)),
      );
    }

    const dir = sortDir.value === "asc" ? 1 : -1;
    list.sort((a, b) => {
      if (sortBy.value === "name") return dir * a.name.localeCompare(b.name);
      return dir * (new Date(a.created_at).getTime() - new Date(b.created_at).getTime());
    });
    return list;
  });

  async function loadRules() {
    isLoading.value = true;
    error.value = "";
    try {
      rules.value = await ruleList();
    } catch (e) {
      error.value = friendlyError(e);
    } finally {
      isLoading.value = false;
    }
  }

  void loadRules();

  function resetFilters() {
    filters.value = defaultFilters();
  }

  function search() {
    // Lọc phía client theo `filteredRules` — điểm neo cho tìm kiếm server sau này.
  }

  async function createRule(request: RuleRequest): Promise<Rule | null> {
    try {
      const rule = await ruleCreate(request);
      rules.value.unshift(rule);
      toast.success(t("rule.toast.created"));
      return rule;
    } catch (e) {
      toast.error(friendlyError(e));
      return null;
    }
  }

  async function updateRule(id: number, request: RuleRequest) {
    try {
      const updated = await ruleUpdate(id, request);
      const idx = rules.value.findIndex((r) => r.id === id);
      if (idx !== -1) rules.value[idx] = updated;
      toast.success(t("rule.toast.updated"));
    } catch (e) {
      toast.error(friendlyError(e));
    }
  }

  async function deleteRule(id: number) {
    try {
      await ruleDelete(id);
      rules.value = rules.value.filter((r) => r.id !== id);
      toast.success(t("rule.toast.deleted"));
    } catch (e) {
      toast.error(friendlyError(e));
    }
  }

  return {
    rules,
    isLoading,
    error,
    filters,
    sortBy,
    sortDir,
    filteredRules,
    resetFilters,
    search,
    createRule,
    updateRule,
    deleteRule,
  };
}
