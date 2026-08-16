import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";

import { friendlyError } from "@/tauri/commands/_base";
import { skillCreate, skillDelete, skillList, skillUpdate } from "@/tauri/commands/skill";
import type { Skill, SkillRequest } from "@/models/skill";
import { useToast } from "@/shared/composables/useToast";

export function useSkill() {
  const { t } = useI18n();
  const toast = useToast();

  const skills = ref<Skill[]>([]);
  const isLoading = ref(false);
  const error = ref("");
  const searchQuery = ref("");
  const sortBy = ref<"name" | "category" | "created_at">("created_at");
  const sortDir = ref<"asc" | "desc">("desc");

  const filteredSkills = computed(() => {
    const q = searchQuery.value.trim().toLowerCase();
    const list = q
      ? skills.value.filter(
          (s) => s.name.toLowerCase().includes(q) || s.description.toLowerCase().includes(q) || s.tags.some((tag) => tag.toLowerCase().includes(q)),
        )
      : [...skills.value];

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

  void loadSkills();

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
    searchQuery,
    sortBy,
    sortDir,
    filteredSkills,
    createSkill,
    updateSkill,
    deleteSkill,
  };
}
