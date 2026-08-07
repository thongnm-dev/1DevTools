//! Tauri command handlers cho màn hình Docker Desktop.
//!
//! Gồm 2 nhóm:
//! - Quản lý danh sách "project" build đã lưu (lưu cục bộ JSON): list/add/update/remove/touch.
//! - Thao tác Docker: liệt kê container/image, start/stop/restart/remove, build,
//!   compose up/down, prune. Mọi thao tác gọi `docker` CLI nên chạy qua `spawn_blocking`
//!   để không chặn UI.

use tauri::ipc::Channel;

use crate::app::error::{log_err, AppError, AppErrorPayload};
use crate::database::docker_project_store;
use crate::models::docker::{DockerContainer, DockerImage, DockerProject};
use crate::services::docker_service;

// === Trạng thái Docker ===

#[tauri::command]
pub async fn docker_available() -> bool {
    tauri::async_runtime::spawn_blocking(docker_service::is_available)
        .await
        .unwrap_or(false)
}

/// Tự khởi động Docker Desktop/daemon (spawn, không chờ tới lúc sẵn sàng).
#[tauri::command]
pub async fn docker_start_desktop() -> Result<(), AppErrorPayload> {
    tauri::async_runtime::spawn_blocking(docker_service::start_desktop)
        .await
        .map_err(|e| log_err(AppError::new(e.to_string())))?
        .map_err(log_err)
}

// === Container / Image ===

#[tauri::command]
pub async fn docker_list_containers(all: bool) -> Result<Vec<DockerContainer>, AppErrorPayload> {
    tauri::async_runtime::spawn_blocking(move || docker_service::list_containers(all))
        .await
        .map_err(|e| log_err(AppError::new(e.to_string())))?
        .map_err(log_err)
}

#[tauri::command]
pub async fn docker_list_images() -> Result<Vec<DockerImage>, AppErrorPayload> {
    tauri::async_runtime::spawn_blocking(docker_service::list_images)
        .await
        .map_err(|e| log_err(AppError::new(e.to_string())))?
        .map_err(log_err)
}

#[tauri::command]
pub async fn docker_start_container(id: String) -> Result<String, AppErrorPayload> {
    tauri::async_runtime::spawn_blocking(move || docker_service::start_container(&id))
        .await
        .map_err(|e| log_err(AppError::new(e.to_string())))?
        .map_err(log_err)
}

#[tauri::command]
pub async fn docker_stop_container(id: String) -> Result<String, AppErrorPayload> {
    tauri::async_runtime::spawn_blocking(move || docker_service::stop_container(&id))
        .await
        .map_err(|e| log_err(AppError::new(e.to_string())))?
        .map_err(log_err)
}

#[tauri::command]
pub async fn docker_restart_container(id: String) -> Result<String, AppErrorPayload> {
    tauri::async_runtime::spawn_blocking(move || docker_service::restart_container(&id))
        .await
        .map_err(|e| log_err(AppError::new(e.to_string())))?
        .map_err(log_err)
}

#[tauri::command]
pub async fn docker_remove_container(id: String, force: bool) -> Result<String, AppErrorPayload> {
    tauri::async_runtime::spawn_blocking(move || docker_service::remove_container(&id, force))
        .await
        .map_err(|e| log_err(AppError::new(e.to_string())))?
        .map_err(log_err)
}

#[tauri::command]
pub async fn docker_remove_image(id: String, force: bool) -> Result<String, AppErrorPayload> {
    tauri::async_runtime::spawn_blocking(move || docker_service::remove_image(&id, force))
        .await
        .map_err(|e| log_err(AppError::new(e.to_string())))?
        .map_err(log_err)
}

#[tauri::command]
pub async fn docker_prune_containers() -> Result<String, AppErrorPayload> {
    tauri::async_runtime::spawn_blocking(docker_service::prune_containers)
        .await
        .map_err(|e| log_err(AppError::new(e.to_string())))?
        .map_err(log_err)
}

#[tauri::command]
pub async fn docker_prune_images(dangling_only: bool) -> Result<String, AppErrorPayload> {
    tauri::async_runtime::spawn_blocking(move || docker_service::prune_images(dangling_only))
        .await
        .map_err(|e| log_err(AppError::new(e.to_string())))?
        .map_err(log_err)
}

#[tauri::command]
pub async fn docker_prune_system() -> Result<String, AppErrorPayload> {
    tauri::async_runtime::spawn_blocking(docker_service::prune_system)
        .await
        .map_err(|e| log_err(AppError::new(e.to_string())))?
        .map_err(log_err)
}

// === Build / Compose (stream output qua Channel) ===

#[tauri::command]
pub async fn docker_build(
    context_path: String,
    dockerfile: String,
    tag: String,
    no_cache: bool,
    on_progress: Channel<String>,
) -> Result<String, AppErrorPayload> {
    tauri::async_runtime::spawn_blocking(move || {
        docker_service::build_with_progress(&context_path, &dockerfile, &tag, no_cache, |line| {
            let _ = on_progress.send(line);
        })
    })
    .await
    .map_err(|e| log_err(AppError::new(e.to_string())))?
    .map_err(log_err)
}

#[tauri::command]
pub async fn docker_compose_up(
    compose_file: String,
    clean: bool,
    on_progress: Channel<String>,
) -> Result<String, AppErrorPayload> {
    tauri::async_runtime::spawn_blocking(move || {
        docker_service::compose_up_with_progress(&compose_file, clean, |line| {
            let _ = on_progress.send(line);
        })
    })
    .await
    .map_err(|e| log_err(AppError::new(e.to_string())))?
    .map_err(log_err)
}

#[tauri::command]
pub async fn docker_compose_down(compose_file: String) -> Result<String, AppErrorPayload> {
    tauri::async_runtime::spawn_blocking(move || docker_service::compose_down(&compose_file))
        .await
        .map_err(|e| log_err(AppError::new(e.to_string())))?
        .map_err(log_err)
}

// === Danh sách project build đã lưu (JSON cục bộ) ===

#[tauri::command]
pub fn docker_list_projects() -> Result<Vec<DockerProject>, AppErrorPayload> {
    let data = docker_project_store::load().map_err(log_err)?;
    let mut projects = data.projects;
    projects.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(projects)
}

#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub fn docker_add_project(
    name: String,
    kind: String,
    context_path: String,
    dockerfile_path: String,
    image_tag: String,
    compose_file: String,
) -> Result<DockerProject, AppErrorPayload> {
    let mut data = docker_project_store::load().map_err(log_err)?;
    if data.next_id < 1 {
        data.next_id = 1;
    }
    let project = DockerProject {
        id: data.next_id,
        name,
        kind,
        context_path,
        dockerfile_path,
        image_tag,
        compose_file,
        last_built: String::new(),
    };
    data.next_id += 1;
    data.projects.push(project.clone());
    docker_project_store::save(&data).map_err(log_err)?;
    Ok(project)
}

#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub fn docker_update_project(
    id: i64,
    name: String,
    kind: String,
    context_path: String,
    dockerfile_path: String,
    image_tag: String,
    compose_file: String,
) -> Result<DockerProject, AppErrorPayload> {
    let mut data = docker_project_store::load().map_err(log_err)?;
    let project = data
        .projects
        .iter_mut()
        .find(|p| p.id == id)
        .ok_or_else(|| log_err(AppError::new(format!("Không tìm thấy project #{id}"))))?;
    project.name = name;
    project.kind = kind;
    project.context_path = context_path;
    project.dockerfile_path = dockerfile_path;
    project.image_tag = image_tag;
    project.compose_file = compose_file;
    let updated = project.clone();
    docker_project_store::save(&data).map_err(log_err)?;
    Ok(updated)
}

#[tauri::command]
pub fn docker_remove_project(id: i64) -> Result<(), AppErrorPayload> {
    let mut data = docker_project_store::load().map_err(log_err)?;
    data.projects.retain(|p| p.id != id);
    docker_project_store::save(&data).map_err(log_err)?;
    Ok(())
}

/// Đánh dấu thời điểm build gần nhất của một project (gọi sau khi build/compose up thành công).
#[tauri::command]
pub fn docker_touch_project(id: i64) -> Result<(), AppErrorPayload> {
    let mut data = docker_project_store::load().map_err(log_err)?;
    if let Some(project) = data.projects.iter_mut().find(|p| p.id == id) {
        project.last_built = chrono::Local::now().to_rfc3339();
        docker_project_store::save(&data).map_err(log_err)?;
    }
    Ok(())
}
