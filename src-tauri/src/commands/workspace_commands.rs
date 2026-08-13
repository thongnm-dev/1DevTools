//! Tauri IPC commands cho registry Workspace (CRUD, đồng thời là tab bar).

use crate::app::error::{log_err, AppErrorPayload};
use crate::database::workspace_store;
use crate::models::workspace::{CreateWorkspaceRequest, UpdateWorkspaceRequest, Workspace};

/// Danh sách toàn bộ workspace, mở gần nhất lên đầu (MRU — dùng làm thứ tự tab).
#[tauri::command]
pub fn workspace_list() -> Result<Vec<Workspace>, AppErrorPayload> {
    let data = workspace_store::load().map_err(log_err)?;
    let mut workspaces = data.workspaces;
    workspaces.sort_by(|a, b| b.last_opened_at.cmp(&a.last_opened_at));
    Ok(workspaces)
}

/// Tạo workspace mới trỏ tới `project_path`. Nếu đã có workspace trỏ tới đúng
/// path này thì trả lại workspace đó (touch lại `last_opened_at`) thay vì tạo trùng tab.
#[tauri::command]
pub fn workspace_create(request: CreateWorkspaceRequest) -> Result<Workspace, AppErrorPayload> {
    let mut data = workspace_store::load().map_err(log_err)?;
    let now = chrono::Local::now().to_rfc3339();

    if let Some(existing) = data.workspaces.iter_mut().find(|w| w.project_path == request.project_path) {
        existing.last_opened_at = now;
        let result = existing.clone();
        workspace_store::save(&data).map_err(log_err)?;
        return Ok(result);
    }

    data.next_id += 1;
    let workspace = Workspace {
        id: data.next_id,
        name: request.name,
        project_path: request.project_path,
        icon: request.icon,
        last_opened_at: now,
        auto_workflow_id: None,
    };
    data.workspaces.push(workspace.clone());
    workspace_store::save(&data).map_err(log_err)?;
    Ok(workspace)
}

/// Đổi tên/icon/auto-trigger workspace.
#[tauri::command]
pub fn workspace_update(id: i64, request: UpdateWorkspaceRequest) -> Result<Workspace, AppErrorPayload> {
    let mut data = workspace_store::load().map_err(log_err)?;
    let workspace = data
        .workspaces
        .iter_mut()
        .find(|w| w.id == id)
        .ok_or_else(|| log_err(crate::app::error::AppError::new(format!("Workspace #{id} không tồn tại"))))?;
    workspace.name = request.name;
    workspace.icon = request.icon;
    workspace.auto_workflow_id = request.auto_workflow_id;
    let result = workspace.clone();
    workspace_store::save(&data).map_err(log_err)?;
    Ok(result)
}

/// Đóng tab / xoá workspace khỏi registry (không đụng tới GitRepo mà nó trỏ tới).
#[tauri::command]
pub fn workspace_remove(id: i64) -> Result<(), AppErrorPayload> {
    let mut data = workspace_store::load().map_err(log_err)?;
    data.workspaces.retain(|w| w.id != id);
    workspace_store::save(&data).map_err(log_err)?;
    Ok(())
}

/// Cập nhật thời điểm mở gần nhất (dùng khi chuyển tab, để giữ thứ tự MRU).
#[tauri::command]
pub fn workspace_touch(id: i64) -> Result<(), AppErrorPayload> {
    let mut data = workspace_store::load().map_err(log_err)?;
    if let Some(workspace) = data.workspaces.iter_mut().find(|w| w.id == id) {
        workspace.last_opened_at = chrono::Local::now().to_rfc3339();
        workspace_store::save(&data).map_err(log_err)?;
    }
    Ok(())
}
