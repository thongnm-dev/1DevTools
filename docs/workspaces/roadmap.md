# Workspaces — Lộ trình đề xuất

Màn hình Workspaces cho phép làm việc cùng lúc trên nhiều project trong một cửa
sổ: chọn project, chọn Git repo, mở terminal, chạy runner command, chọn
agent/provider, chọn workflow, chọn skill, chọn prompt.

Quyết định kiến trúc đã chốt: **các workspace chạy nền thật sự** (giống VS
Code) — mở nhiều project cùng lúc, terminal/git-watch/process vẫn sống khi
workspace không ở tab active, không phải load lại khi chuyển tab.

## 1. Hiện trạng tái dùng được

| Nhu cầu | Đã có trong codebase |
|---|---|
| Chọn project | `src-tauri/src/database/git_repo_store.rs` (`GitRepo { id, name, path, last_opened }`) |
| Chạy runner command | `src-tauri/src/database/dev_runner_store.rs` (per-repo, key theo hash path) + `GitRunnerPanel.vue` |
| Chọn agent/provider | `src-tauri/src/database/ai_account_store.rs` + `ai_profile_store.rs` |
| Mở terminal | `src/features/terminal` (`useTerminal.ts`, xterm) + `terminal_commands.rs` |
| Menu theo role | `governance` (menu_store, menu_permission_store) |

## 2. Kết quả kiểm tra khả năng chạy nền song song

| Thành phần | Hiện trạng | Cần sửa? |
|---|---|---|
| Terminal (`useTerminal.ts`) | `tabs`/`entries` khai báo ở **module scope** → đã là singleton toàn app, mỗi tab sống độc lập với route. Backend `terminal_service` key theo `sessionId` riêng từng phiên. | ✅ Không cần sửa |
| Dev Runner (chạy command) | `runCommand()` trong `GitRunnerPanel.vue` chỉ gọi `term.addTab({ autoCommand })` — "chạy command" = mở tab terminal mới, không có process riêng ở backend. | ✅ Không cần sửa, ăn theo terminal |
| Git watch (auto-refresh) | `git_watch_service.rs`: state là `Mutex<Option<Active>>` — chỉ theo dõi được **1 repo tại 1 thời điểm**, `git_watch_stop()` không nhận path, event `git-repo-changed` không kèm path. | ❌ Đổi sang `Mutex<HashMap<path, Active>>`, event kèm `path` |
| Git panel state (`useGit.ts`) | `activeRepo`, staged/unstaged/branches... là **module-scope singleton** — chỉ 1 repo active cùng lúc. | ❌ Refactor thành `Map<workspaceId, GitState>`, theo đúng khuôn mẫu `tabs`/`entries` đã có trong `useTerminal.ts` |

## 3. Domain mới cần thêm

- **Workflow**: chuỗi step tự động hoá (chạy skill / prompt / runner command / mở terminal) theo thứ tự — xem `data-model.md`.
- **Skill**: thư viện chỉ dẫn tái sử dụng cho agent (CRUD đơn giản).
- **Prompt**: thư viện prompt/snippet tái sử dụng, có placeholder biến (CRUD đơn giản).
- **Testing**: không tạo entity riêng — chỉ thêm field `kind: run | test | build` vào `DevCommand` hiện có.
- **Automation**: hợp nhất vào Workflow — trigger từ git-watch event hoặc chạy tay, không phải entity riêng.

## 4. Lộ trình theo phase

1. **Phase 1 — Workspace registry** ✅ **Đã làm**: `workspace_store.rs` + `workspace_commands.rs` (JSON, giống `git_repo_store`, dedupe theo `project_path`) + `WorkspacesPage.vue` (tab bar MRU, dialog tạo/sửa workspace — chọn từ `GitRepo` đã add hoặc browse folder mới, icon picker). Nội dung mỗi tab hiện là panel overview (icon/tên/path + 4 quick action: Terminal/Git/VS Code/Explorer) — CHƯA embed Git/Terminal/Runner thật, việc đó thuộc Phase 2.
2. **Phase 2 — Chạy nền thật sự** ✅ **Đã làm**: `git_watch_service.rs` đổi từ `Mutex<Option<Active>>` sang `Mutex<HashMap<path, Active>>` (nhiều path theo dõi đồng thời, `git_watch_stop` nhận `path`). `useGit.ts` KHÔNG phải module-scope singleton như ghi nhận cũ (mỗi lần gọi `useGit()` tạo state độc lập) — chỉ cần thêm: (a) tự track `watchedPath` theo từng instance để chuyển repo trong 1 instance vẫn dừng đúng watcher cũ, (b) option `openRepo(repo, { persist: false })` để nhúng không ghi đè `localStorage["git.activeRepoId"]` của màn Git Desktop độc lập. Thêm `WorkspaceGitPanel.vue` (component mới, KHÔNG đụng `GitDesktopPage.vue`) — nhúng thật branch info + staged/unstaged + diff + commit vào từng workspace tab, tất cả tab giữ mounted qua `v-show` (đã verify bằng invoke-count: chuyển tab qua lại không gọi lại `git_status`/`git_repo_info`). Terminal/Runner vẫn qua quick action (đã multi-instance sẵn từ Phase 1, chưa cần nhúng UI riêng).
3. **Phase 3 — Agent/Skill/Prompt**: picker agent/provider (tái dùng `ai_account_store`); thêm `skill_store.rs` + `prompt_store.rs` (CRUD).
4. **Phase 4 — Workflow/Automation**: `workflow_store.rs` (chuỗi step + canvas layout); trigger tự động từ git-watch event; thêm field `kind` vào `DevCommand` cho Testing.

Chi tiết data model: xem [`data-model.md`](./data-model.md).
Chi tiết layout UI: xem [`ui-design.md`](./ui-design.md).
