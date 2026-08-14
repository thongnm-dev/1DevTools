//! Model cho liên kết Workspace <-> Task (bảng cục bộ `workspace_tasks.json`).
//!
//! Workspace là registry cục bộ theo máy (`workspace_store.rs`), còn Task nằm ở
//! Postgres dùng chung — nên liên kết này cũng phải là dữ liệu cục bộ (không thể
//! FK `workspace_id` vào Postgres vì id workspace chỉ duy nhất trong phạm vi 1 máy).

use serde::{Deserialize, Serialize};

/// Một liên kết "task này được thêm vào workspace này".
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkspaceTask {
    pub id: i64,
    pub workspace_id: i64,
    pub task_id: i32,
}
