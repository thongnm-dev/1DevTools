import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";
import { open } from "@tauri-apps/plugin-dialog";

import { friendlyError } from "@/tauri/commands/_base";
import { workspaceCreate, workspaceList, workspaceRemove, workspaceTouch, workspaceUpdate } from "@/tauri/commands/workspace";
import { gitAddRepo, gitListRepos } from "@/tauri/commands/git";
import type { GitRepo } from "@/models/git";
import type { Workspace } from "@/models/workspace";
import { DEFAULT_WORKSPACE_ICON } from "@/models/workspace";
import { useToast } from "@/shared/composables/useToast";

const ACTIVE_WORKSPACE_KEY = "workspaces.activeId";

export function useWorkspace() {
  const { t } = useI18n();
  const toast = useToast();

  const workspaces = ref<Workspace[]>([]);
  const gitRepos = ref<GitRepo[]>([]);
  const activeId = ref<number | null>(null);
  const isLoading = ref(false);
  const error = ref("");

  const activeWorkspace = computed(() => workspaces.value.find((w) => w.id === activeId.value) ?? null);

  async function loadWorkspaces() {
    isLoading.value = true;
    error.value = "";
    try {
      workspaces.value = await workspaceList();
      const savedId = Number(localStorage.getItem(ACTIVE_WORKSPACE_KEY) ?? "");
      const target = workspaces.value.find((w) => w.id === savedId) ?? workspaces.value[0] ?? null;
      activeId.value = target?.id ?? null;
    } catch (e) {
      error.value = friendlyError(e);
    } finally {
      isLoading.value = false;
    }
  }

  async function loadGitRepos() {
    try {
      gitRepos.value = await gitListRepos();
    } catch (e) {
      error.value = friendlyError(e);
    }
  }

  void loadWorkspaces();
  void loadGitRepos();

  function selectWorkspace(id: number) {
    activeId.value = id;
    localStorage.setItem(ACTIVE_WORKSPACE_KEY, String(id));
    workspaceTouch(id).catch(() => {});
  }

  async function createFromRepo(repo: GitRepo, name: string, icon?: string): Promise<Workspace | null> {
    error.value = "";
    try {
      const ws = await workspaceCreate({ name, project_path: repo.path, icon: icon ?? DEFAULT_WORKSPACE_ICON });
      if (!workspaces.value.some((w) => w.id === ws.id)) {
        workspaces.value.unshift(ws);
      } else {
        const idx = workspaces.value.findIndex((w) => w.id === ws.id);
        workspaces.value[idx] = ws;
      }
      selectWorkspace(ws.id);
      toast.success(t("workspaces.toast.created"));
      return ws;
    } catch (e) {
      error.value = friendlyError(e);
      toast.error(friendlyError(e));
      return null;
    }
  }

  /** Mở dialog chọn thư mục mới và thêm vào Git repo registry (chưa tạo workspace). */
  async function pickFolder(): Promise<GitRepo | null> {
    try {
      const picked = await open({ directory: true, title: t("workspaces.dialog.selectFolderTitle") });
      if (!picked || typeof picked !== "string") return null;
      const repo = await gitAddRepo(picked);
      if (!gitRepos.value.some((r) => r.id === repo.id)) {
        gitRepos.value = [repo, ...gitRepos.value];
      }
      return repo;
    } catch (e) {
      error.value = friendlyError(e);
      toast.error(friendlyError(e));
      return null;
    }
  }

  async function updateWorkspace(id: number, patch: { name?: string; icon?: string }) {
    const ws = workspaces.value.find((w) => w.id === id);
    if (!ws) return;
    error.value = "";
    try {
      const updated = await workspaceUpdate(id, {
        name: patch.name ?? ws.name,
        icon: patch.icon ?? ws.icon,
      });
      const idx = workspaces.value.findIndex((w) => w.id === id);
      if (idx !== -1) workspaces.value[idx] = updated;
    } catch (e) {
      error.value = friendlyError(e);
      toast.error(friendlyError(e));
    }
  }

  async function removeWorkspace(id: number) {
    error.value = "";
    try {
      await workspaceRemove(id);
      workspaces.value = workspaces.value.filter((w) => w.id !== id);
      if (activeId.value === id) {
        const next = workspaces.value[0]?.id ?? null;
        if (next !== null) {
          selectWorkspace(next);
        } else {
          activeId.value = null;
          localStorage.removeItem(ACTIVE_WORKSPACE_KEY);
        }
      }
      toast.success(t("workspaces.toast.closed"));
    } catch (e) {
      error.value = friendlyError(e);
      toast.error(friendlyError(e));
    }
  }

  return {
    workspaces,
    gitRepos,
    activeId,
    activeWorkspace,
    isLoading,
    error,
    selectWorkspace,
    createFromRepo,
    pickFolder,
    updateWorkspace,
    removeWorkspace,
  };
}

export type WorkspaceApi = ReturnType<typeof useWorkspace>;
