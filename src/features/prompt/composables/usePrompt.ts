import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";

import { friendlyError } from "@/tauri/commands/_base";
import { promptCreate, promptDelete, promptList, promptMarkUsed, promptUpdate } from "@/tauri/commands/prompt";
import type { Prompt, PromptRequest } from "@/models/prompt";
import { useToast } from "@/shared/composables/useToast";

export function usePrompt() {
  const { t } = useI18n();
  const toast = useToast();

  const prompts = ref<Prompt[]>([]);
  const isLoading = ref(false);
  const error = ref("");
  const searchQuery = ref("");

  const filteredPrompts = computed(() => {
    const q = searchQuery.value.trim().toLowerCase();
    if (!q) return prompts.value;
    return prompts.value.filter(
      (p) => p.title.toLowerCase().includes(q) || p.category.toLowerCase().includes(q) || p.tags.some((tag) => tag.toLowerCase().includes(q)),
    );
  });

  async function loadPrompts() {
    isLoading.value = true;
    error.value = "";
    try {
      prompts.value = await promptList();
    } catch (e) {
      error.value = friendlyError(e);
    } finally {
      isLoading.value = false;
    }
  }

  void loadPrompts();

  async function createPrompt(request: PromptRequest): Promise<Prompt | null> {
    try {
      const prompt = await promptCreate(request);
      prompts.value.unshift(prompt);
      toast.success(t("prompt.toast.created"));
      return prompt;
    } catch (e) {
      toast.error(friendlyError(e));
      return null;
    }
  }

  async function updatePrompt(id: number, request: PromptRequest) {
    try {
      const updated = await promptUpdate(id, request);
      const idx = prompts.value.findIndex((p) => p.id === id);
      if (idx !== -1) prompts.value[idx] = updated;
      toast.success(t("prompt.toast.updated"));
    } catch (e) {
      toast.error(friendlyError(e));
    }
  }

  async function deletePrompt(id: number) {
    try {
      await promptDelete(id);
      prompts.value = prompts.value.filter((p) => p.id !== id);
      toast.success(t("prompt.toast.deleted"));
    } catch (e) {
      toast.error(friendlyError(e));
    }
  }

  async function copyPrompt(prompt: Prompt) {
    try {
      await navigator.clipboard.writeText(prompt.body);
      const updated = await promptMarkUsed(prompt.id);
      const idx = prompts.value.findIndex((p) => p.id === prompt.id);
      if (idx !== -1) prompts.value[idx] = updated;
      toast.success(t("prompt.toast.copied"));
    } catch (e) {
      toast.error(friendlyError(e));
    }
  }

  return {
    prompts,
    isLoading,
    error,
    searchQuery,
    filteredPrompts,
    createPrompt,
    updatePrompt,
    deletePrompt,
    copyPrompt,
  };
}
