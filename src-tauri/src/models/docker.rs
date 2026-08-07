//! Model/DTO cho module Docker Desktop — quản lý container/image/project build local.
//!
//! Toàn bộ thao tác được thực hiện bằng cách gọi `docker` CLI của hệ điều hành
//! (cùng cách tiếp cận với Git Desktop ở `models::git`) để tận dụng Docker context/
//! credential sẵn có của người dùng, không cần thêm dependency HTTP client nặng.

use serde::{Deserialize, Serialize};

/// Một container (đang chạy hoặc đã dừng), lấy từ `docker ps -a --format {{json .}}`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DockerContainer {
    pub id: String,
    pub name: String,
    pub image: String,
    pub command: String,
    /// Chuỗi trạng thái hiển thị của Docker (vd. "Up 3 hours", "Exited (0) 2 days ago").
    pub status: String,
    /// Trạng thái rút gọn: running/exited/paused/created/restarting/dead...
    pub state: String,
    pub ports: String,
    pub created: String,
    /// % CPU hiện tại (từ `docker stats`, "—" nếu không lấy được — vd. container đã dừng).
    #[serde(default)]
    pub cpu: String,
    /// Bộ nhớ đang dùng / giới hạn (từ `docker stats`, "—" nếu không lấy được).
    #[serde(default)]
    pub memory: String,
    /// Tên project Docker Compose (label `com.docker.compose.project`), rỗng nếu
    /// container không được tạo bởi `docker compose` (chạy trực tiếp bằng `docker run`).
    #[serde(default)]
    pub compose_project: String,
    /// Thư mục chứa file compose lúc `up` (label `com.docker.compose.project.working_dir`),
    /// dùng để nhóm các container cùng một lần khởi tạo (vd. cùng thư mục `deploy/`).
    #[serde(default)]
    pub compose_working_dir: String,
}

/// Một image, lấy từ `docker images --format {{json .}}`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DockerImage {
    pub id: String,
    pub repository: String,
    pub tag: String,
    pub created: String,
    pub size: String,
}

/// Một project build đã lưu (lưu cục bộ trong JSON) — để build/rebuild lại mà không
/// phải nhớ/gõ lại đường dẫn context, Dockerfile hay tên file compose mỗi lần.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DockerProject {
    pub id: i64,
    pub name: String,
    /// "dockerfile" (build một image từ Dockerfile) hoặc "compose" (docker compose up --build).
    pub kind: String,
    /// Build context (kind = dockerfile) hoặc thư mục chứa file compose (kind = compose).
    pub context_path: String,
    /// Đường dẫn Dockerfile, tương đối hoặc tuyệt đối. Rỗng = "Dockerfile" mặc định trong context.
    pub dockerfile_path: String,
    /// Tag đặt cho image khi build (kind = dockerfile). Rỗng = không gắn tag (`docker build` không `-t`).
    pub image_tag: String,
    /// Đường dẫn file compose (kind = compose), vd. "docker-compose.yml".
    pub compose_file: String,
    /// Thời điểm build gần nhất (ISO string), rỗng nếu chưa build lần nào.
    #[serde(default)]
    pub last_built: String,
}
