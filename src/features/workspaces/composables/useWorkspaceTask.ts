import { ref } from "vue";

export interface WorkspaceTask {
  id: string;
  name: string;
  done: boolean;
}

function storageKey(workspaceId: number) {
  return `workspace_${workspaceId}_tasks`;
}

function loadFromStorage(workspaceId: number): WorkspaceTask[] {
  try {
    return JSON.parse(localStorage.getItem(storageKey(workspaceId)) ?? "[]");
  } catch {
    return [];
  }
}

export function useWorkspaceTask(workspaceId: number) {
  const tasks = ref<WorkspaceTask[]>(loadFromStorage(workspaceId));

  function persist() {
    localStorage.setItem(storageKey(workspaceId), JSON.stringify(tasks.value));
  }

  function addTask(name: string) {
    const trimmed = name.trim();
    if (!trimmed) return;
    tasks.value.push({ id: crypto.randomUUID(), name: trimmed, done: false });
    persist();
  }

  function toggleDone(id: string) {
    const task = tasks.value.find((t) => t.id === id);
    if (task) { task.done = !task.done; persist(); }
  }

  function removeTask(id: string) {
    tasks.value = tasks.value.filter((t) => t.id !== id);
    persist();
  }

  function renameTask(id: string, name: string) {
    const trimmed = name.trim();
    if (!trimmed) return;
    const task = tasks.value.find((t) => t.id === id);
    if (task) { task.name = trimmed; persist(); }
  }

  return { tasks, addTask, toggleDone, removeTask, renameTask };
}
