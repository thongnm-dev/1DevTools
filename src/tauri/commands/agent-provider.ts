import { safeInvoke } from "./_base";
import type { AgentProvider, AgentProviderRequest } from "@/models/agent-provider";

export function agentProviderList() {
  return safeInvoke<AgentProvider[]>("agent_provider_list");
}

export function agentProviderCreate(request: AgentProviderRequest) {
  return safeInvoke<AgentProvider>("agent_provider_create", { request });
}

export function agentProviderUpdate(id: number, request: AgentProviderRequest) {
  return safeInvoke<AgentProvider>("agent_provider_update", { id, request });
}

export function agentProviderDelete(id: number) {
  return safeInvoke<void>("agent_provider_delete", { id });
}

export function agentProviderSetEnabled(id: number, enabled: boolean) {
  return safeInvoke<AgentProvider>("agent_provider_set_enabled", { id, enabled });
}
