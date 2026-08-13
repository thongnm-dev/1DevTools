import { safeInvoke } from "./_base";
import type { Skill, SkillRequest } from "@/models/skill";

export function skillList() {
  return safeInvoke<Skill[]>("skill_list");
}

export function skillCreate(request: SkillRequest) {
  return safeInvoke<Skill>("skill_create", { request });
}

export function skillUpdate(id: number, request: SkillRequest) {
  return safeInvoke<Skill>("skill_update", { id, request });
}

export function skillDelete(id: number) {
  return safeInvoke<void>("skill_delete", { id });
}
