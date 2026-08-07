/** Types cho module Docker Desktop — khớp với DTO ở `src-tauri/src/models/docker.rs`. */

/** Một container (đang chạy hoặc đã dừng). */
export interface DockerContainer {
  id: string;
  name: string;
  image: string;
  command: string;
  status: string;
  state: string;
  ports: string;
  created: string;
  cpu: string;
  memory: string;
  compose_project: string;
  compose_working_dir: string;
}

/** Một image. */
export interface DockerImage {
  id: string;
  repository: string;
  tag: string;
  created: string;
  size: string;
}

/** Loại project build đã lưu. */
export type DockerProjectKind = "dockerfile" | "compose";

/** Một project build đã lưu (lưu cục bộ). */
export interface DockerProject {
  id: number;
  name: string;
  kind: DockerProjectKind;
  context_path: string;
  dockerfile_path: string;
  image_tag: string;
  compose_file: string;
  last_built: string;
}
