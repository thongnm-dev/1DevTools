import { safeInvoke } from "./_base";
import type { UserSummary } from "@/models/user";

export function listUsers() {
  return safeInvoke<UserSummary[]>("list_users");
}
