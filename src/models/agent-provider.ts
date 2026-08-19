/** Types cho quản lý AI Agent Provider — khớp với DTO ở `src-tauri/src/models/agent_provider.rs`. */

export type AgentProviderType =
  | "claude"
  | "codex"
  | "gemini"
  | "copilot"
  | "cursor"
  | "custom";

export interface AgentProvider {
  id: number;
  /** Tên hiển thị, VD: "Claude Code". */
  name: string;
  /** Mã định danh duy nhất (slug), VD: "claude-code". */
  code: string;
  provider_type: AgentProviderType;
  description: string;
  icon: string;
  /** Lệnh CLI để khởi chạy agent, VD: "claude". */
  command: string;
  /** Trang chủ / tài liệu. */
  website: string;
  /** Danh sách model được hỗ trợ. */
  models: string[];
  /** Có cho phép sử dụng trong hệ thống hay không. */
  enabled: boolean;
  created_at: string;
  updated_at: string;
}

/** Request đăng ký / cập nhật provider. */
export interface AgentProviderRequest {
  name: string;
  code: string;
  provider_type: AgentProviderType;
  description: string;
  icon: string;
  command: string;
  website: string;
  models: string[];
  enabled: boolean;
}

export const DEFAULT_AGENT_PROVIDER_ICON = "pi pi-android";

export interface AgentProviderTypeMeta {
  icon: string;
  badgeClass: string;
}

export const AGENT_PROVIDER_TYPES: AgentProviderType[] = [
  "claude",
  "codex",
  "gemini",
  "copilot",
  "cursor",
  "custom",
];

export const AGENT_PROVIDER_TYPE_META: Record<AgentProviderType, AgentProviderTypeMeta> = {
  claude:  { icon: "pi pi-android",  badgeClass: "bg-orange-100 text-orange-700" },
  codex:   { icon: "pi pi-code",     badgeClass: "bg-slate-100 text-slate-700" },
  gemini:  { icon: "pi pi-sparkles", badgeClass: "bg-blue-100 text-blue-700" },
  copilot: { icon: "pi pi-github",   badgeClass: "bg-violet-100 text-violet-700" },
  cursor:  { icon: "pi pi-pencil",   badgeClass: "bg-teal-100 text-teal-700" },
  custom:  { icon: "pi pi-cog",      badgeClass: "bg-canvas text-muted" },
};
