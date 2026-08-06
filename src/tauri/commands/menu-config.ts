import { isMockMode, safeInvoke } from "./_base";
import { mockMenuConfigs } from "./_mock-data";
import type { MenuConfig } from "@/models/menu-config";

export function listMenuConfigs() {
  if (isMockMode()) {
    return Promise.resolve(mockMenuConfigs);
  }
  return safeInvoke<MenuConfig[]>("list_menu_configs");
}
