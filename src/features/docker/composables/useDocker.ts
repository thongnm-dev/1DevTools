import { ref } from "vue";
import { useI18n } from "vue-i18n";

import { canUseTauriRuntime, friendlyError } from "@/tauri/commands/_base";
import {
  dockerAddProject,
  dockerAvailable,
  dockerBuild,
  dockerComposeDown,
  dockerComposeUp,
  dockerListContainers,
  dockerListImages,
  dockerListProjects,
  dockerPruneContainers,
  dockerPruneImages,
  dockerPruneSystem,
  dockerRemoveContainer,
  dockerRemoveImage,
  dockerRemoveProject,
  dockerRestartContainer,
  dockerStartContainer,
  dockerStartDesktop,
  dockerStopContainer,
  dockerTouchProject,
  dockerUpdateProject,
} from "@/tauri/commands/docker";
import type { DockerContainer, DockerImage, DockerProject, DockerProjectKind } from "@/models/docker";
import { useToast } from "@/shared/composables/useToast";

export function useDocker() {
  const { t } = useI18n();
  const toast = useToast();

  const runtimeAvailable = canUseTauriRuntime();
  const dockerReady = ref(true);

  const containers = ref<DockerContainer[]>([]);
  const images = ref<DockerImage[]>([]);
  const projects = ref<DockerProject[]>([]);

  const loadingContainers = ref(false);
  const loadingImages = ref(false);
  const loadingProjects = ref(false);

  /** true: `docker ps -a` (kèm container đã dừng); false: chỉ container đang chạy. */
  const showStopped = ref(true);

  /** Id đang có thao tác chạy (start/stop/remove/...) — dùng hiện spinner trên đúng hàng. */
  const busyIds = ref<Set<string>>(new Set());

  function isBusy(id: string) {
    return busyIds.value.has(id);
  }

  function setBusy(id: string, busy: boolean) {
    const next = new Set(busyIds.value);
    if (busy) next.add(id);
    else next.delete(id);
    busyIds.value = next;
  }

  function reportError(prefix: string, e: unknown) {
    toast.error(`${prefix}: ${friendlyError(e)}`);
  }

  // === Nạp danh sách ===

  async function loadContainers() {
    if (!runtimeAvailable) return;
    // Docker chưa chạy — banner "Docker is not running" đã báo rồi, tránh spam
    // thêm toast lỗi kết nối kỹ thuật (named pipe/socket) mỗi lần refresh.
    if (!dockerReady.value) {
      containers.value = [];
      return;
    }
    loadingContainers.value = true;
    try {
      containers.value = await dockerListContainers(showStopped.value);
    } catch (e) {
      reportError(t("docker.toast.containersLoadFailed"), e);
    } finally {
      loadingContainers.value = false;
    }
  }

  async function loadImages() {
    if (!runtimeAvailable) return;
    if (!dockerReady.value) {
      images.value = [];
      return;
    }
    loadingImages.value = true;
    try {
      images.value = await dockerListImages();
    } catch (e) {
      reportError(t("docker.toast.imagesLoadFailed"), e);
    } finally {
      loadingImages.value = false;
    }
  }

  async function loadProjects() {
    if (!runtimeAvailable) return;
    loadingProjects.value = true;
    try {
      projects.value = await dockerListProjects();
    } catch (e) {
      reportError(t("docker.toast.projectsLoadFailed"), e);
    } finally {
      loadingProjects.value = false;
    }
  }

  async function refreshAll() {
    if (!runtimeAvailable) return;
    try {
      dockerReady.value = await dockerAvailable();
    } catch {
      dockerReady.value = false;
    }
    await Promise.all([loadContainers(), loadImages(), loadProjects()]);
  }

  async function toggleShowStopped() {
    showStopped.value = !showStopped.value;
    await loadContainers();
  }

  async function init() {
    if (!runtimeAvailable) return;
    await refreshAll();
  }

  // === Tự khởi động Docker khi banner báo "Docker is not running" ===

  const startingDesktop = ref(false);
  const POLL_INTERVAL_MS = 3000;
  const POLL_MAX_ATTEMPTS = 40; // ~2 phút
  let pollTimer: ReturnType<typeof setTimeout> | null = null;

  function stopPollingDockerReady() {
    if (pollTimer) clearTimeout(pollTimer);
    pollTimer = null;
    startingDesktop.value = false;
  }

  function pollDockerReady(attempt: number) {
    pollTimer = setTimeout(async () => {
      const ready = await dockerAvailable().catch(() => false);
      if (ready) {
        dockerReady.value = true;
        startingDesktop.value = false;
        pollTimer = null;
        toast.success(t("docker.toast.dockerNowReady"));
        await refreshAll();
        return;
      }
      if (attempt >= POLL_MAX_ATTEMPTS) {
        startingDesktop.value = false;
        pollTimer = null;
        return;
      }
      pollDockerReady(attempt + 1);
    }, POLL_INTERVAL_MS);
  }

  /** Tự mở Docker Desktop/daemon rồi tự poll `dockerAvailable()` cho tới khi sẵn sàng. */
  async function startDockerDesktop() {
    try {
      await dockerStartDesktop();
      toast.info(t("docker.toast.startingDocker"));
      stopPollingDockerReady();
      startingDesktop.value = true;
      pollDockerReady(1);
    } catch (e) {
      reportError(t("docker.toast.startDockerFailed"), e);
    }
  }

  // === Container ===

  async function startContainer(id: string) {
    setBusy(id, true);
    try {
      await dockerStartContainer(id);
      toast.success(t("docker.toast.containerStarted"));
      await loadContainers();
    } catch (e) {
      reportError(t("docker.toast.containerStartFailed"), e);
    } finally {
      setBusy(id, false);
    }
  }

  async function stopContainer(id: string) {
    setBusy(id, true);
    try {
      await dockerStopContainer(id);
      toast.success(t("docker.toast.containerStopped"));
      await loadContainers();
    } catch (e) {
      reportError(t("docker.toast.containerStopFailed"), e);
    } finally {
      setBusy(id, false);
    }
  }

  async function restartContainer(id: string) {
    setBusy(id, true);
    try {
      await dockerRestartContainer(id);
      toast.success(t("docker.toast.containerRestarted"));
      await loadContainers();
    } catch (e) {
      reportError(t("docker.toast.containerRestartFailed"), e);
    } finally {
      setBusy(id, false);
    }
  }

  async function removeContainer(id: string, force: boolean) {
    setBusy(id, true);
    try {
      await dockerRemoveContainer(id, force);
      toast.success(t("docker.toast.containerRemoved"));
      await loadContainers();
    } catch (e) {
      reportError(t("docker.toast.containerRemoveFailed"), e);
    } finally {
      setBusy(id, false);
    }
  }

  // === Image ===

  async function removeImage(id: string, force: boolean) {
    setBusy(id, true);
    try {
      await dockerRemoveImage(id, force);
      toast.success(t("docker.toast.imageRemoved"));
      await loadImages();
    } catch (e) {
      reportError(t("docker.toast.imageRemoveFailed"), e);
    } finally {
      setBusy(id, false);
    }
  }

  // === Dọn dẹp (cleanup) ===

  async function pruneContainers() {
    try {
      await dockerPruneContainers();
      toast.success(t("docker.toast.pruneContainersDone"));
      await loadContainers();
    } catch (e) {
      reportError(t("docker.toast.pruneFailed"), e);
    }
  }

  async function pruneImages(danglingOnly: boolean) {
    try {
      await dockerPruneImages(danglingOnly);
      toast.success(t("docker.toast.pruneImagesDone"));
      await loadImages();
    } catch (e) {
      reportError(t("docker.toast.pruneFailed"), e);
    }
  }

  async function pruneSystem() {
    try {
      await dockerPruneSystem();
      toast.success(t("docker.toast.pruneSystemDone"));
      await refreshAll();
    } catch (e) {
      reportError(t("docker.toast.pruneFailed"), e);
    }
  }

  // === Build / rebuild ===

  /**
   * Build ad-hoc (không gắn với project đã lưu) hoặc build lại một project đã lưu.
   * `noCache` = "Clean and build": bỏ qua build cache và luôn kéo lại base image
   * mới nhất, thay vì rebuild tăng dần (mặc định, vẫn tái dùng cache cũ).
   */
  async function build(
    contextPath: string,
    dockerfile: string,
    tag: string,
    onLine: (line: string) => void,
    projectId?: number,
    noCache = false,
  ) {
    await dockerBuild(contextPath, dockerfile, tag, noCache, onLine);
    if (projectId) await dockerTouchProject(projectId).catch(() => undefined);
    await loadImages();
    if (projectId) await loadProjects();
  }

  /**
   * `docker compose up -d --build` (rebuild tăng dần) cho một project compose,
   * hoặc `docker compose build --no-cache --pull` rồi `up -d` nếu `clean` = true
   * ("Clean and build").
   */
  async function composeUp(composeFile: string, onLine: (line: string) => void, projectId?: number, clean = false) {
    await dockerComposeUp(composeFile, clean, onLine);
    if (projectId) await dockerTouchProject(projectId).catch(() => undefined);
    await Promise.all([loadContainers(), loadImages()]);
    if (projectId) await loadProjects();
  }

  async function composeDown(project: DockerProject) {
    setBusy(`project-${project.id}`, true);
    try {
      await dockerComposeDown(project.compose_file);
      toast.success(t("docker.toast.composeDownDone", { name: project.name }));
      await loadContainers();
    } catch (e) {
      reportError(t("docker.toast.composeDownFailed"), e);
    } finally {
      setBusy(`project-${project.id}`, false);
    }
  }

  // === Project build đã lưu (CRUD) ===

  async function addProject(
    name: string,
    kind: DockerProjectKind,
    contextPath: string,
    dockerfilePath: string,
    imageTag: string,
    composeFile: string,
  ) {
    try {
      const project = await dockerAddProject(name, kind, contextPath, dockerfilePath, imageTag, composeFile);
      projects.value = [...projects.value, project].sort((a, b) => a.name.localeCompare(b.name));
      toast.success(t("docker.toast.projectSaved"));
      return project;
    } catch (e) {
      reportError(t("docker.toast.projectSaveFailed"), e);
      return null;
    }
  }

  async function updateProject(
    id: number,
    name: string,
    kind: DockerProjectKind,
    contextPath: string,
    dockerfilePath: string,
    imageTag: string,
    composeFile: string,
  ) {
    try {
      const project = await dockerUpdateProject(id, name, kind, contextPath, dockerfilePath, imageTag, composeFile);
      projects.value = projects.value
        .map((p) => (p.id === id ? project : p))
        .sort((a, b) => a.name.localeCompare(b.name));
      toast.success(t("docker.toast.projectSaved"));
      return project;
    } catch (e) {
      reportError(t("docker.toast.projectSaveFailed"), e);
      return null;
    }
  }

  async function removeProject(id: number) {
    try {
      await dockerRemoveProject(id);
      projects.value = projects.value.filter((p) => p.id !== id);
      toast.success(t("docker.toast.projectRemoved"));
    } catch (e) {
      reportError(t("docker.toast.projectRemoveFailed"), e);
    }
  }

  return {
    runtimeAvailable,
    dockerReady,
    startingDesktop,
    startDockerDesktop,
    stopPollingDockerReady,
    containers,
    images,
    projects,
    loadingContainers,
    loadingImages,
    loadingProjects,
    showStopped,
    isBusy,
    init,
    refreshAll,
    loadContainers,
    loadImages,
    loadProjects,
    toggleShowStopped,
    startContainer,
    stopContainer,
    restartContainer,
    removeContainer,
    removeImage,
    pruneContainers,
    pruneImages,
    pruneSystem,
    build,
    composeUp,
    composeDown,
    addProject,
    updateProject,
    removeProject,
  };
}

export type DockerApi = ReturnType<typeof useDocker>;
