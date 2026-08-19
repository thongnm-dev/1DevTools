/** Types cho quản lý AI Agent Provider Model — khớp với DTO ở
 * `src-tauri/src/models/agent_provider_model.rs`. */

export interface AgentProviderModel {
  id: number;
  provider_id: number;
  /** Tên provider (join sẵn để hiển thị). */
  provider_name: string;
  name: string;
  code: string;
  version: string;
  description: string;
  /** Có cho phép sử dụng model trong hệ thống hay không. */
  enabled: boolean;
  created_at: string;
  updated_at: string;
}

/** Request đăng ký / cập nhật model. */
export interface AgentProviderModelRequest {
  provider_id: number;
  name: string;
  code: string;
  version: string;
  description: string;
  enabled: boolean;
}

/** Nhãn hiển thị cho 1 model — dùng thẳng tên (đã đầy đủ, vd "Claude Opus 4.8"). */
export function agentProviderModelLabel(model: AgentProviderModel): string {
  return model.name;
}

/** Cờ `--model` truyền cho CLI — ưu tiên mã model (`code`), fallback về tên. */
export function agentProviderModelFlag(model: AgentProviderModel): string {
  return model.code || model.name;
}
