# Workspaces — Data model: Workflow / Skill / Prompt

Nguyên tắc: theo đúng convention hiện có trong repo — mỗi domain một file JSON
cục bộ trong `data_dir()`/`data_subdir()` (xem `git_repo_store.rs`,
`ai_account_store.rs`), không dùng SQL/Postgres, không có khái niệm
`username` (app single-user cục bộ). Không tách bảng step riêng như bản tham
khảo (Postgres, `internal-extension/.../useAiWorkflow.ts`) — vì cả object
Workflow (kèm steps + layout) nhỏ, lưu 1 file JSON là đủ và đơn giản hơn nhiều
so với việc gọi API tạo/sửa/xoá/reorder step riêng lẻ.

## 1. Skill

Thư viện chỉ dẫn tái sử dụng để "nạp" cho agent (Claude/Codex/Copilot) khi mở
một terminal session.

```rust
// src-tauri/src/models/skill.rs
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum SkillCategory {
    #[serde(rename = "implement")] Implement,
    #[serde(rename = "review")]     Review,
    #[serde(rename = "test")]       Test,
    #[serde(rename = "release")]    Release,
    #[serde(rename = "docs")]       Docs,
    #[serde(rename = "custom")]     Custom,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Skill {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub icon: String,              // "pi pi-book"
    pub category: SkillCategory,
    /// Nội dung chỉ dẫn (markdown) — tương đương SKILL.md.
    pub instructions: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}
```

Store: `skill_store.rs` — file `skills.json`, cấu trúc `{ skills: Vec<Skill>, next_id: i64 }` giống `GitRepoData`.

**Cách "chạy" 1 Skill trong workspace** (dùng ở bước Runner/Workflow, không
phải bảng riêng): resolve thành 1 `autoCommand` cho terminal, ví dụ ghi
`instructions` ra `<project>/.claude/skills/<slug>/SKILL.md` (đúng convention
Claude Code skill hiện có) rồi mở tab terminal mới với `autoCommand: "claude"`
tại `startDir` của workspace. Cụ thể do implementation phase quyết định —
model ở trên đã đủ để build CRUD trước.

## 2. Prompt

Thư viện prompt/snippet tái sử dụng, hỗ trợ placeholder dạng `{{variable}}`.

```rust
// src-tauri/src/models/prompt.rs
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Prompt {
    pub id: i64,
    pub title: String,
    /// Nội dung prompt, có thể chứa placeholder {{var}}.
    pub body: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub category: String,          // free-text, không cần enum cứng
    #[serde(default)]
    pub usage_count: i32,
    pub created_at: String,
    pub updated_at: String,
}
```

`variables: Vec<String>` **không** lưu ở backend — frontend parse từ `body`
bằng regex `\{\{(\w+)\}\}` mỗi lần hiển thị form điền giá trị trước khi dùng
(tránh đồng bộ 2 nguồn sự thật).

Store: `prompt_store.rs` — file `prompts.json`, cùng cấu trúc list + `next_id`.

## 3. Workflow

Chuỗi step tự động hoá, gắn với canvas layout (kéo-thả vị trí node như bản
tham khảo). Step type đổi từ 5 loại "nhãn giai đoạn" trừu tượng (skill /
implement / review / release / custom) của bản tham khảo sang loại **gắn
được với hành động thật trong app** — vì Workflow ở Workspaces phải chạy
được (mở terminal, chạy runner...), không chỉ là tag hiển thị.

```rust
// src-tauri/src/models/workflow.rs
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum WorkflowStepType {
    #[serde(rename = "skill")]    Skill,     // gắn 1 Skill → mở terminal + nạp chỉ dẫn
    #[serde(rename = "prompt")]   Prompt,    // gắn 1 Prompt → điền placeholder rồi gõ vào agent
    #[serde(rename = "runner")]   Runner,    // chạy 1 dev command đã khai báo ở Runner
    #[serde(rename = "terminal")] Terminal,  // mở terminal thường, autoCommand tự do
    #[serde(rename = "custom")]   Custom,    // chỉ là note/checklist, không tự chạy gì
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub id: String,                 // uuid v4 (nhẹ hơn cần bộ đếm riêng cho step lồng trong workflow)
    pub name: String,
    pub step_type: WorkflowStepType,
    pub icon: String,
    pub description: String,
    #[serde(default)]
    pub is_latest_step: bool,       // đánh dấu bước cuối — không cho mở thêm terminal sau bước này

    // Binding theo step_type — luôn có mặt trong JSON nhưng chỉ field khớp
    // step_type mới có giá trị, còn lại None.
    #[serde(default)] pub skill_id: Option<i64>,
    #[serde(default)] pub prompt_id: Option<i64>,
    #[serde(default)] pub runner_command: Option<String>,   // literal command khi step_type = runner/terminal
    #[serde(default)] pub ai_account_id: Option<i64>,        // pin agent/provider cho step (tái dùng ai_account_store)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodePos { pub x: f64, pub y: f64 }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Workflow {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub steps: Vec<WorkflowStep>,            // đã đúng thứ tự chạy — không cần step_order riêng
    #[serde(default)]
    pub layout: HashMap<String, NodePos>,    // vị trí canvas, key = step.id
    pub created_at: String,
    pub updated_at: String,
}
```

Store: `workflow_store.rs` — file `workflows.json`, `{ workflows: Vec<Workflow>, next_id: i64 }`.

**Workflow là thư viện global, tái sử dụng cho mọi workspace** — không gắn
`workspace_id` cố định. Khi "chạy" một workflow, người dùng chọn workspace
đích tại thời điểm chạy (giống cách chọn runner command hiện tại), không lưu
sẵn trong Workflow. Điều này khớp với yêu cầu gốc "chọn workflow" như một
picker trong màn hình Workspaces.

**So với bản tham khảo** (`internal-extension`):
- Không có `username`/`created_by` (app single-user cục bộ).
- Không tách bảng `ai_workflow_step` riêng + API reorder — steps là 1 array
  lồng trong Workflow, sửa thứ tự = sửa array rồi save nguyên object.
- Không có bảng `ai_model` riêng — field `ai_account_id` trỏ trực tiếp vào
  `ai_account_store` đã có sẵn trong app (account đã gắn `provider`).
- `skillName` (free-text) → `skill_id`/`prompt_id` (foreign key) vì Skill và
  Prompt giờ là library CRUD riêng, không phải folder name tự nhập.

## 4. Tauri commands cần thêm

Theo đúng pattern mỏng hiện có (`git_repo_store` ↔ `git_commands.rs`):

```
skill_list, skill_create, skill_update, skill_delete
prompt_list, prompt_create, prompt_update, prompt_delete
workflow_list, workflow_create, workflow_update, workflow_delete, workflow_duplicate
workflow_save_layout(id, layout)      // chỉ ghi field layout, không đụng steps
```

Không cần command riêng cho step (add/update/delete/reorder step) như bản
tham khảo — vì `workflow_update` nhận toàn bộ `steps: Vec<WorkflowStep>` mới
và ghi đè, đơn giản hơn nhiều với quy mô dữ liệu nhỏ (vài chục step/workflow).
