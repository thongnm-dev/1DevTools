//! Business logic cho liên kết Workspace <-> Task (JSON cục bộ).

use crate::app::result::AppResult;
use crate::database::workspace_task_store;
use crate::models::workspace_task::WorkspaceTask;

pub fn list_for_workspace(workspace_id: i64) -> AppResult<Vec<WorkspaceTask>> {
    let data = workspace_task_store::load()?;
    Ok(data.links.into_iter().filter(|l| l.workspace_id == workspace_id).collect())
}

/// Thêm liên kết (workspace_id, task_id) — idempotent, trả lại liên kết đã có
/// nếu task này đã được thêm vào workspace này thay vì tạo trùng.
pub fn add(workspace_id: i64, task_id: i32) -> AppResult<WorkspaceTask> {
    let mut data = workspace_task_store::load()?;
    if let Some(existing) = data
        .links
        .iter()
        .find(|l| l.workspace_id == workspace_id && l.task_id == task_id)
    {
        return Ok(existing.clone());
    }
    data.next_id += 1;
    let link = WorkspaceTask { id: data.next_id, workspace_id, task_id };
    data.links.push(link.clone());
    workspace_task_store::save(&data)?;
    Ok(link)
}

pub fn remove(workspace_id: i64, task_id: i32) -> AppResult<()> {
    let mut data = workspace_task_store::load()?;
    data.links.retain(|l| !(l.workspace_id == workspace_id && l.task_id == task_id));
    workspace_task_store::save(&data)?;
    Ok(())
}
