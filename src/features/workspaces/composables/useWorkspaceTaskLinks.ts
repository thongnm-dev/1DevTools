import { computed, ref } from "vue";
import { friendlyError } from "@/tauri/commands/_base";
import { taskList } from "@/tauri/commands/task";
import { workspaceTaskAdd, workspaceTaskList, workspaceTaskRemove } from "@/tauri/commands/workspace-task";
import { useToast } from "@/shared/composables/useToast";
import type { Task } from "@/models/task";
import type { Workspace } from "@/models/workspace";

/**
 * Danh sách task đã được thêm vào 1 workspace cụ thể (liên kết cục bộ, xem
 * `workspace_task_commands.rs`), join với dữ liệu task thật từ registry chung
 * (`taskList`).
 */
export function useWorkspaceTaskLinks(workspace: Workspace) {
  const toast = useToast();

  const allTasks = ref<Task[]>([]);
  const linkedTaskIds = ref<number[]>([]);
  const loading = ref(false);
  const error = ref("");

  const linkedTasks = computed(() => allTasks.value.filter((t) => linkedTaskIds.value.includes(t.id)));

  async function refresh() {
    loading.value = true;
    error.value = "";
    try {
      const [tasks, links] = await Promise.all([taskList(), workspaceTaskList(workspace.id)]);
      allTasks.value = tasks;
      linkedTaskIds.value = links.map((l) => l.task_id);
    } catch (e) {
      error.value = friendlyError(e);
    } finally {
      loading.value = false;
    }
  }

  void refresh();

  async function addTask(task: Task) {
    try {
      await workspaceTaskAdd(workspace.id, task.id);
      await refresh();
    } catch (e) {
      toast.error(friendlyError(e));
    }
  }

  async function removeTask(taskId: number) {
    try {
      await workspaceTaskRemove(workspace.id, taskId);
      await refresh();
    } catch (e) {
      toast.error(friendlyError(e));
    }
  }

  return {
    linkedTasks,
    loading,
    error,
    addTask,
    removeTask,
  };
}

export type WorkspaceTaskLinksApi = ReturnType<typeof useWorkspaceTaskLinks>;
