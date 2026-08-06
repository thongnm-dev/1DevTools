import { safeInvoke } from "./_base";
import type { MenuConfig } from "@/models/menu-config";

export function listMenuConfigs() {
  return safeInvoke<MenuConfig[]>("list_menu_configs");
}
