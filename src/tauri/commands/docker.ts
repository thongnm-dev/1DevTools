import { Channel } from "@tauri-apps/api/core";
import { safeInvoke } from "./_base";
import type { DockerContainer, DockerImage, DockerProject, DockerProjectKind } from "@/models/docker";

/** Tạo Channel nhận từng dòng output; gắn callback nếu có. */
function lineChannel(onLine?: (line: string) => void) {
  const channel = new Channel<string>();
  if (onLine) channel.onmessage = onLine;
  return channel;
}

// === Trạng thái Docker ===

export function dockerAvailable() {
  return safeInvoke<boolean>("docker_available");
}

/** Tự khởi động Docker Desktop/daemon (bắn và quên — không chờ tới lúc sẵn sàng). */
export function dockerStartDesktop() {
  return safeInvoke<void>("docker_start_desktop");
}

// === Container / Image ===

export function dockerListContainers(all: boolean) {
  return safeInvoke<DockerContainer[]>("docker_list_containers", { all });
}

export function dockerListImages() {
  return safeInvoke<DockerImage[]>("docker_list_images");
}

export function dockerStartContainer(id: string) {
  return safeInvoke<string>("docker_start_container", { id });
}

export function dockerStopContainer(id: string) {
  return safeInvoke<string>("docker_stop_container", { id });
}

export function dockerRestartContainer(id: string) {
  return safeInvoke<string>("docker_restart_container", { id });
}

export function dockerRemoveContainer(id: string, force: boolean) {
  return safeInvoke<string>("docker_remove_container", { id, force });
}

export function dockerRemoveImage(id: string, force: boolean) {
  return safeInvoke<string>("docker_remove_image", { id, force });
}

export function dockerPruneContainers() {
  return safeInvoke<string>("docker_prune_containers");
}

export function dockerPruneImages(danglingOnly: boolean) {
  return safeInvoke<string>("docker_prune_images", { danglingOnly });
}

export function dockerPruneSystem() {
  return safeInvoke<string>("docker_prune_system");
}

// === Build / Compose (stream output theo từng dòng) ===

export function dockerBuild(
  contextPath: string,
  dockerfile: string,
  tag: string,
  noCache: boolean,
  onLine?: (line: string) => void,
) {
  return safeInvoke<string>("docker_build", {
    contextPath,
    dockerfile,
    tag,
    noCache,
    onProgress: lineChannel(onLine),
  });
}

export function dockerComposeUp(composeFile: string, clean: boolean, onLine?: (line: string) => void) {
  return safeInvoke<string>("docker_compose_up", {
    composeFile,
    clean,
    onProgress: lineChannel(onLine),
  });
}

export function dockerComposeDown(composeFile: string) {
  return safeInvoke<string>("docker_compose_down", { composeFile });
}

// === Danh sách project build đã lưu ===

export function dockerListProjects() {
  return safeInvoke<DockerProject[]>("docker_list_projects");
}

export function dockerAddProject(
  name: string,
  kind: DockerProjectKind,
  contextPath: string,
  dockerfilePath: string,
  imageTag: string,
  composeFile: string,
) {
  return safeInvoke<DockerProject>("docker_add_project", {
    name,
    kind,
    contextPath,
    dockerfilePath,
    imageTag,
    composeFile,
  });
}

export function dockerUpdateProject(
  id: number,
  name: string,
  kind: DockerProjectKind,
  contextPath: string,
  dockerfilePath: string,
  imageTag: string,
  composeFile: string,
) {
  return safeInvoke<DockerProject>("docker_update_project", {
    id,
    name,
    kind,
    contextPath,
    dockerfilePath,
    imageTag,
    composeFile,
  });
}

export function dockerRemoveProject(id: number) {
  return safeInvoke<void>("docker_remove_project", { id });
}

export function dockerTouchProject(id: number) {
  return safeInvoke<void>("docker_touch_project", { id });
}
