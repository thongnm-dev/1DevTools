import { safeInvoke } from "./_base";
import type {
  AgentProviderModel,
  AgentProviderModelRequest,
} from "@/models/agent-provider-model";

export function agentProviderModelList() {
  return safeInvoke<AgentProviderModel[]>("agent_provider_model_list");
}

/** Chỉ các model đang bật — dùng cho danh mục chọn model của workflow step. */
export function agentProviderModelListEnabled() {
  return safeInvoke<AgentProviderModel[]>("agent_provider_model_list_enabled");
}

export function agentProviderModelCreate(request: AgentProviderModelRequest) {
  return safeInvoke<AgentProviderModel>("agent_provider_model_create", { request });
}

export function agentProviderModelUpdate(id: number, request: AgentProviderModelRequest) {
  return safeInvoke<AgentProviderModel>("agent_provider_model_update", { id, request });
}

export function agentProviderModelDelete(id: number) {
  return safeInvoke<void>("agent_provider_model_delete", { id });
}

export function agentProviderModelSetEnabled(id: number, enabled: boolean) {
  return safeInvoke<AgentProviderModel>("agent_provider_model_set_enabled", { id, enabled });
}
