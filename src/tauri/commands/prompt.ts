import { safeInvoke } from "./_base";
import type { Prompt, PromptRequest } from "@/models/prompt";

export function promptList() {
  return safeInvoke<Prompt[]>("prompt_list");
}

export function promptCreate(request: PromptRequest) {
  return safeInvoke<Prompt>("prompt_create", { request });
}

export function promptUpdate(id: number, request: PromptRequest) {
  return safeInvoke<Prompt>("prompt_update", { id, request });
}

export function promptDelete(id: number) {
  return safeInvoke<void>("prompt_delete", { id });
}

export function promptMarkUsed(id: number) {
  return safeInvoke<Prompt>("prompt_mark_used", { id });
}
