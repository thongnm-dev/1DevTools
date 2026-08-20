import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";

import { friendlyError } from "@/tauri/commands/_base";
import { skillCreate, skillDelete, skillList, skillUpdate } from "@/tauri/commands/skill";
import { masterDataList } from "@/tauri/commands/master-data";
import type { Skill, SkillRequest, SkillType } from "@/models/skill";
import type { MasterData } from "@/models/master-data";
import { useToast } from "@/shared/composables/useToast";

/** `keygroup` của master_data chứa danh mục skill. */
const SKILL_TYPE_KEYGROUP = "SKILL_TYPE";

interface SkillFilters {
  keyword: string;
  category: "All" | SkillType;
}

const defaultFilters = (): SkillFilters => ({ keyword: "", category: "All" });

export function useSkill() {
  const { t } = useI18n();
  const toast = useToast();

  const skills = ref<Skill[]>([]);
  const isLoading = ref(false);
  const error = ref("");
  const filters = ref<SkillFilters>(defaultFilters());
  const sortBy = ref<"name" | "category" | "created_at">("created_at");
  const sortDir = ref<"asc" | "desc">("desc");

  /** Danh mục skill — lấy từ `master_data` (keygroup `SKILL_TYPE`), sắp theo `display_order`. */
  const categories = ref<MasterData[]>([]);

  const filteredSkills = computed(() => {
    const f = filters.value;
    let list = [...skills.value];

    if (f.category !== "All") {
      list = list.filter((s) => s.category === f.category);
    }

    const q = f.keyword.trim().toLowerCase();
    if (q) {
      list = list.filter(
        (s) => s.name.toLowerCase().includes(q) || s.description.toLowerCase().includes(q) || s.tags.some((tag) => tag.toLowerCase().includes(q)),
      );
    }

    const dir = sortDir.value === "asc" ? 1 : -1;
    list.sort((a, b) => {
      if (sortBy.value === "name") return dir * a.name.localeCompare(b.name);
      if (sortBy.value === "category") return dir * a.category.localeCompare(b.category);
      return dir * (new Date(a.created_at).getTime() - new Date(b.created_at).getTime());
    });
    return list;
  });

  async function loadSkills() {
    isLoading.value = true;
    error.value = "";
    try {
      skills.value = await skillList();
    } catch (e) {
      error.value = friendlyError(e);
    } finally {
      isLoading.value = false;
    }
  }

  async function loadCategories() {
    try {
      const all = await masterDataList();
      categories.value = all
        .filter((m) => m.keygroup === SKILL_TYPE_KEYGROUP)
        .sort((a, b) => a.display_order - b.display_order);
    } catch {
      categories.value = [];
    }
  }

  void loadSkills();
  void loadCategories();

  function resetFilters() {
    filters.value = defaultFilters();
  }

  function search() {
    // Lọc phía client theo `filteredSkills` — điểm neo cho tìm kiếm server sau này.
  }

  async function createSkill(request: SkillRequest): Promise<Skill | null> {
    try {
      const skill = await skillCreate(request);
      skills.value.unshift(skill);
      toast.success(t("skill.toast.created"));
      return skill;
    } catch (e) {
      toast.error(friendlyError(e));
      return null;
    }
  }

  async function updateSkill(id: number, request: SkillRequest) {
    try {
      const updated = await skillUpdate(id, request);
      const idx = skills.value.findIndex((s) => s.id === id);
      if (idx !== -1) skills.value[idx] = updated;
      toast.success(t("skill.toast.updated"));
    } catch (e) {
      toast.error(friendlyError(e));
    }
  }

  async function deleteSkill(id: number) {
    try {
      await skillDelete(id);
      skills.value = skills.value.filter((s) => s.id !== id);
      toast.success(t("skill.toast.deleted"));
    } catch (e) {
      toast.error(friendlyError(e));
    }
  }

  return {
    skills,
    isLoading,
    error,
    filters,
    sortBy,
    sortDir,
    filteredSkills,
    categories,
    resetFilters,
    search,
    createSkill,
    updateSkill,
    deleteSkill,
  };
}
