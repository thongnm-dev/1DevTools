import { isMockMode, safeInvoke } from "./_base";
import { mockEffectivePermissions } from "./_mock-data";
import type { EffectiveMenuPermission } from "@/models/menu-permission";

export function listEffectiveMenuPermissions(userId: number) {
  if (isMockMode()) {
    return Promise.resolve(mockEffectivePermissions);
  }
  return safeInvoke<EffectiveMenuPermission[]>("list_effective_menu_permissions", { userId });
}
