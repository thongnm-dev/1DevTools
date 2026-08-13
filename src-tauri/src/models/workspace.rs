//! Model cho registry Workspace — màn hình cho phép mở nhiều project cùng lúc
//! trong 1 cửa sổ. Mỗi workspace trỏ tới 1 project đã được thêm ở Git Desktop
//! (`GitRepo.path`), không lưu lại dữ liệu repo (tránh trùng nguồn sự thật).

use serde::{Deserialize, Serialize};

/// Icon mặc định khi workspace chưa chọn icon riêng.
fn default_workspace_icon() -> String {
    "pi pi-folder".to_string()
}

/// Một workspace đã mở — hiển thị dưới dạng tab trên màn hình Workspaces.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Workspace {
    pub id: i64,
    pub name: String,
    /// Đường dẫn tuyệt đối tới project — trỏ tới `GitRepo.path` đã thêm ở Git Desktop.
    pub project_path: String,
    #[serde(default = "default_workspace_icon")]
    pub icon: String,
    /// Thời điểm mở gần nhất (ISO string) — dùng để sắp xếp tab theo MRU.
    #[serde(default)]
    pub last_opened_at: String,
}

/// Request tạo workspace mới (hoặc mở lại nếu đã có workspace trỏ tới path này).
#[derive(Debug, Deserialize)]
pub struct CreateWorkspaceRequest {
    pub name: String,
    pub project_path: String,
    #[serde(default = "default_workspace_icon")]
    pub icon: String,
}

/// Request đổi tên/icon workspace.
#[derive(Debug, Deserialize)]
pub struct UpdateWorkspaceRequest {
    pub name: String,
    pub icon: String,
}
