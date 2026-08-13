export type MenuKey =
  | "overview"
  | "git"
  | "terminal"
  | "docker"
  | "gov-users"
  | "gov-roles"
  | "gov-menus"
  | "gov-permissions"
  | "app-config"
  | "ai-usage"
  | "workflow"
  | "workspaces";

export type AppRouteKey = MenuKey | "login" | "forgotPassword";

export type MessageMode = "info" | "error";

export type SummaryMetric = {
  label: string;
  value: string;
};
