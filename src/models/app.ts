export type MenuKey = "overview" | "git" | "terminal";

export type AppRouteKey = MenuKey | "login" | "forgotPassword";

export type MessageMode = "info" | "error";

export type SummaryMetric = {
  label: string;
  value: string;
};
