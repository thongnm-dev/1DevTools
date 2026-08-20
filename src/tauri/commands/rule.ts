import { safeInvoke } from "./_base";
import type { Rule, RuleRequest } from "@/models/rule";

export function ruleList() {
  return safeInvoke<Rule[]>("rule_list");
}

export function ruleCreate(request: RuleRequest) {
  return safeInvoke<Rule>("rule_create", { request });
}

export function ruleUpdate(id: number, request: RuleRequest) {
  return safeInvoke<Rule>("rule_update", { id, request });
}

export function ruleDelete(id: number) {
  return safeInvoke<void>("rule_delete", { id });
}

/** Xuất rule ra file markdown, trả về đường dẫn file để preview. */
export function ruleExport(id: number) {
  return safeInvoke<string>("rule_export", { id });
}
