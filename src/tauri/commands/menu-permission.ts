import { safeInvoke } from "./_base";
import type { EffectiveMenuPermission } from "@/models/menu-permission";

export function listEffectiveMenuPermissions(userId: number) {
  return safeInvoke<EffectiveMenuPermission[]>("list_effective_menu_permissions", { userId });
}
