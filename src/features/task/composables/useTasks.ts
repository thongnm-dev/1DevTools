import { computed, ref } from "vue";
import { canUseTauriRuntime, friendlyError } from "@/tauri/commands/_base";
import { taskCreate, taskList, taskUpdate } from "@/tauri/commands/task";
import { useAuthStore } from "@/app/stores/auth";
import type { CreateTaskRequest, Task, UpdateTaskRequest } from "@/models/task";

interface TaskFilters {
  keyword: string;
  isComplete: boolean | null;
}

const defaultFilters = (): TaskFilters => ({ keyword: "", isComplete: null });

export function useTasks() {
  function username(): string {
    return useAuthStore().user?.username ?? "";
  }

  const tasks = ref<Task[]>([]);
  const loading = ref(false);
  const error = ref("");
  const filters = ref<TaskFilters>(defaultFilters());

  const draft = ref<{
    id: number;
    task_cd: string;
    task_name: string;
    category_id: string;
    is_complete: boolean;
  } | null>(null);
  const isCreating = ref(false);

  const filteredTasks = computed(() => [...tasks.value].sort((a, b) => b.id - a.id));

  function resetFilters() {
    filters.value = defaultFilters();
    void fetchTasks();
  }

  async function fetchTasks() {
    if (!canUseTauriRuntime()) return;
    loading.value = true;
    error.value = "";
    try {
      tasks.value = await taskList(filters.value.keyword, filters.value.isComplete);
    } catch (e) {
      error.value = friendlyError(e);
    } finally {
      loading.value = false;
    }
  }

  function search() {
    void fetchTasks();
  }

  function startCreate() {
    isCreating.value = true;
    draft.value = { id: 0, task_cd: "", task_name: "", category_id: "other", is_complete: false };
  }

  function selectTask(id: number) {
    isCreating.value = false;
    const task = tasks.value.find((t) => t.id === id);
    if (task) {
      draft.value = {
        id: task.id,
        task_cd: task.task_cd,
        task_name: task.task_name,
        category_id: task.category_id,
        is_complete: task.is_complete,
      };
    }
  }

  async function saveDraft(): Promise<boolean> {
    if (!draft.value) return false;
    if (!draft.value.task_cd.trim()) {
      error.value = "Task code is required.";
      return false;
    }
    error.value = "";
    try {
      if (isCreating.value) {
        const request: CreateTaskRequest = {
          task_cd: draft.value.task_cd,
          task_name: draft.value.task_name,
          category_id: draft.value.category_id,
        };
        await taskCreate(username(), request);
      } else {
        const request: UpdateTaskRequest = {
          task_cd: draft.value.task_cd,
          task_name: draft.value.task_name,
          category_id: draft.value.category_id,
          is_complete: draft.value.is_complete,
        };
        await taskUpdate(draft.value.id, username(), request);
      }
      await fetchTasks();
      return true;
    } catch (e) {
      error.value = friendlyError(e);
      return false;
    }
  }

  return {
    tasks,
    filteredTasks,
    loading,
    error,
    filters,
    draft,
    isCreating,
    fetchTasks,
    search,
    resetFilters,
    startCreate,
    selectTask,
    saveDraft,
  };
}

export type TasksApi = ReturnType<typeof useTasks>;
