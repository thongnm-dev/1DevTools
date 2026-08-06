import type { LoginResponse } from "@/models/auth";
import type { MenuConfig } from "@/models/menu-config";
import type { EffectiveMenuPermission } from "@/models/menu-permission";

/** Nhóm menu top-level (không thuộc menu_group nào), khớp UNGROUPED trong menu store. */
const UNGROUPED = "—";

export const mockLoginResponse: LoginResponse = {
  user_id: 1,
  username: "demo",
  full_name: "Nguyễn Minh Thông",
  email: "thongnm@allexceed.co.jp",
  roles: ["admin"],
};

export const mockMenuConfigs: MenuConfig[] = [
  { key: "overview", title: "Overview", path: "/overview", icon: "pi-home", group: UNGROUPED, visible: true, order: 1 },
  { key: "tool-snippets", title: "Snippets", path: "/tools/snippets", icon: "pi-code", group: "Tools", visible: true, order: 10 },
  { key: "tool-converter", title: "Converter", path: "/tools/converter", icon: "pi-sync", group: "Tools", visible: true, order: 11 },
  { key: "cloud-storage", title: "Storage", path: "/cloud/storage", icon: "pi-cloud-upload", group: "Cloud", visible: true, order: 20 },
  { key: "cloud-deploy", title: "Deployments", path: "/cloud/deploy", icon: "pi-server", group: "Cloud", visible: true, order: 21 },
  { key: "ai-chat", title: "Chat Assistant", path: "/ai/chat", icon: "pi-comments", group: "AI Agent", visible: true, order: 30 },
  { key: "gov-users", title: "Users", path: "/governance/users", icon: "pi-users", group: "Governance", visible: true, order: 40 },
  { key: "gov-roles", title: "Roles", path: "/governance/roles", icon: "pi-shield", group: "Governance", visible: true, order: 41 },
  { key: "settings", title: "Settings", path: "/settings", icon: "pi-cog", group: UNGROUPED, visible: true, order: 100 },
];

export const mockEffectivePermissions: EffectiveMenuPermission[] = mockMenuConfigs.map((m) => ({
  menu_key: m.key,
  is_allowed: true,
  role_allowed: true,
  source: "role",
}));
