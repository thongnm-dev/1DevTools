# Workspaces — Layout UI cụ thể

Stack UI: PrimeVue + Tailwind, cùng convention hiện có (`page-title`,
`section-title`, `bg-panel`/`bg-canvas`/`border-divider`, composable-per-domain
kiểu `useGit.ts`). `IconPickerDialog.vue` và `DialogFooter.vue` đã có sẵn
trong `src/shared/components/`, tái dùng thẳng — không cần copy từ project
khác.

## 1. Workspaces shell (màn hình gốc)

```
┌─────────────────────────────────────────────────────────────────┐
│ [+ New]  ●Proj A  ●Proj B  ●Proj C            (tab bar, kéo thả) │
├───────────────┬───────────────────────────────────────────────────┤
│ Sidebar        │  Sub-nav: Git | Terminal | Runner | Agent |      │
│ (project tree/ │           Workflow | Skill | Prompt              │
│  quick switch) │  ─────────────────────────────────────────────  │
│                │                                                   │
│                │        <panel của sub-nav đang chọn>              │
│                │                                                   │
└───────────────┴───────────────────────────────────────────────────┘
```

- Mỗi tab ở tab bar = 1 `Workspace` (đã mở), giữ mounted (dùng `v-show`, xem
  `roadmap.md` mục "chạy nền thật sự") — sub-nav bên trong tái dùng nguyên
  `GitTabSwitcher.vue`-pattern (đổi từ 2 tab Changes/History thành N tab).
- Sub-nav **Git/Terminal/Runner** = render lại đúng component hiện có
  (`GitDesktopPage` nội dung, `TerminalPage` nội dung, `GitRunnerPanel`),
  chỉ đổi input `repoPath`/`workspaceId` theo tab đang active.
- Sub-nav **Agent** = picker chọn `ai_account` áp cho workspace (mặc định
  dùng khi mở terminal agent trong workspace này).
- Sub-nav **Workflow/Skill/Prompt** = 3 trang mới, mô tả ở mục 2–3 dưới đây.
  Đây là **thư viện global** (không đổi theo workspace), nên có thể mở như
  overlay/dialog full-screen thay vì sub-nav riêng nếu muốn — nhưng để nhất
  quán với sub-nav pattern hiện tại, vẫn đặt làm tab ngang hàng, chỉ khác là
  nội dung không phụ thuộc `activeRepo`.

## 2. Workflow page — clone từ `AiWorkflowPage.vue`

Giữ nguyên toàn bộ layout/interaction của bản tham khảo (sidebar resize
280px, canvas free-drag node + SVG bezier arrow, step dialog, delete
confirm, duplicate, auto-layout) — chỉ đổi phần data binding:

| Bản tham khảo (`internal-extension`) | Bản clone (1DevTools) |
|---|---|
| `useAiWorkflow()` gọi Tauri command `ai_workflow_*` (Postgres, `username`-scoped) | `useWorkflow()` gọi Tauri command `workflow_*` (JSON store, không username) |
| `stepModelId` chọn từ `aiModelList()` (bảng `ai_model`) | Đổi thành `stepAiAccountId`, chọn từ `ai_account_store` đã có (field `provider`/`name`) |
| `stepSkillName` free-text (folder `.claude/skills/<name>`) | Đổi thành `Select` chọn `skillId` từ `skill_store` (chỉ hiện khi `stepType === 'skill'`) |
| Step type: skill / implement / review / release / custom | Step type: **skill / prompt / runner / terminal / custom** (xem `data-model.md` mục 3) — thêm field tương ứng: `promptId` (khi `prompt`), `runnerCommand` (khi `runner`/`terminal`, `InputText` literal command) |
| `aiWorkflowStepReorder` (API riêng) | Không cần — `updateWorkflow` ghi lại nguyên `steps: WorkflowStep[]` |
| Không có nút "Run" | **Thêm nút "Run step" / "Run workflow"** ở panel chi tiết step + header — resolve step thành `autoCommand`, gọi `useTerminal().addTab()` đúng cách `GitRunnerPanel.runCommand()` đang làm, mở lần lượt từng tab cho mỗi step (không block chờ exit — giữ đơn giản như runner hiện tại) |
| Không gắn với project cụ thể | Khi bấm "Run", nếu đang ở trong 1 Workspace tab thì dùng `startDir` = path của workspace đó; nếu mở Workflow như trang global thì hiện dialog chọn workspace đích trước khi chạy |

Step type badge màu (giữ style `bg-x-100 text-x-700` như bản gốc, đổi map):

```
skill:    bg-sky-100 text-sky-700     · pi pi-book
prompt:   bg-fuchsia-100 text-fuchsia-700 · pi pi-comment
runner:   bg-violet-100 text-violet-700   · pi pi-play
terminal: bg-amber-100 text-amber-700     · pi pi-desktop
custom:   bg-canvas text-muted            · pi pi-cog
```

File cần tạo (đặt trong `src/features/workspaces/` — domain mới, không nhồi
vào `ai-agent` vì Workflow ở đây phục vụ Workspaces chứ không riêng AI usage):

```
src/features/workspaces/components/WorkflowPage.vue      (clone AiWorkflowPage.vue)
src/features/workspaces/composables/useWorkflow.ts        (clone useAiWorkflow.ts, bỏ username)
src/tauri/commands/workflow.ts                             (clone ai-workflow.ts, đổi field theo data-model.md)
src/_/types/workflow.ts                                     (STEP_TYPE_META mới)
```

## 3. Skill page & Prompt page — CRUD đơn giản (không cần canvas)

Hai trang này **không** clone AiWorkflowPage — nội dung chỉ là list + form,
nên dùng layout đơn giản hơn nhiều, giống `GovernanceMenusPage.vue`/
`AiUsagePage.vue`:

```
┌───────────────────────────────────────────────┐
│ [Search...............]           [+ New]     │
├───────────────────────────────────────────────┤
│ ▸ Card/List item: icon, name, category badge,  │
│   description rút gọn, tags, [Edit] [Delete]   │
│ ▸ ...                                          │
└───────────────────────────────────────────────┘
```

- **SkillPage.vue**: list card 2–3 cột, filter theo `category`. Dialog
  add/edit có `Textarea` lớn cho `instructions` (markdown, dùng chung style
  input với `MarkdownPreviewDialog.vue` đã có sẵn trong repo để preview).
- **PromptPage.vue**: list dạng bảng gọn (title, category, tags, updatedAt).
  Dialog add/edit có `Textarea` cho `body`, hiển thị realtime danh sách biến
  `{{var}}` phát hiện được ngay dưới textarea (regex phía frontend, không
  cần gọi backend).
- Cả hai đều có nút "Insert into terminal" / "Copy" ở mỗi item — dùng ngay
  không cần qua Workflow (khớp yêu cầu gốc "chọn skill", "chọn prompt" như
  hành động độc lập, không bắt buộc phải đi qua Workflow).

## 4. Đăng ký route/menu

Thêm vào `src/app/router/routes.ts` (theo đúng pattern các route khác):

```ts
{ key: "workspaces", path: "/workspaces", title: "Workspaces", subtitle: "..." }
```

Và khai báo tương ứng trong governance menu store để phân quyền theo role
(xem `roadmap.md` mục 1). Workflow/Skill/Prompt là sub-tab bên trong
`WorkspacesPage.vue`, không cần route riêng — tránh phá vỡ mental model "1
màn hình cho mọi công cụ" mà yêu cầu gốc đang hướng tới.
