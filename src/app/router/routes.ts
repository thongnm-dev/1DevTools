import type { RouteRecordRaw } from "vue-router";
import type { AppRouteKey, MenuKey } from "@/models/app";

export type AppRoute = {
  key: AppRouteKey;
  path: string;
  requiresAuth?: boolean;
  title: string;
  subtitle: string;
  breadcrumbs?: string[];
};

export const appRoutes: AppRoute[] = [
  {
    key: "overview",
    path: "/overview",
    title: "Overview",
    subtitle: "Project and phase summary for the selected work data.",
  },
  {
    key: "login",
    path: "/login",
    title: "Login",
    subtitle: "Authenticate to continue to protected screens.",
  },
  {
    key: "forgotPassword",
    path: "/forgot-password",
    title: "Forgot Password",
    subtitle: "Request a password reset link via email.",
  },
];

export const defaultRoute = appRoutes[0];
export const loginRoute = appRoutes.find((r) => r.key === "login")!;

export function routeByKey(key: MenuKey): AppRoute {
  return appRoutes.find((r) => r.key === key) ?? defaultRoute;
}

export function routeByPath(path: string): AppRoute {
  const pathname = path.split("?")[0];
  return appRoutes.find((r) => r.path === pathname) ?? defaultRoute;
}

export const vueRoutes: RouteRecordRaw[] = [
  { path: "/", redirect: "/overview" },
  {
    path: "/overview",
    component: () => import("@/features/overview/components/OverviewPage.vue"),
    meta: { key: "overview" as MenuKey },
  },
  {
    path: "/login",
    component: () => import("@/features/auth/components/LoginPage.vue"),
    meta: { key: "login" as AppRouteKey },
  },
  {
    path: "/forgot-password",
    component: () => import("@/features/auth/components/ForgotPasswordPage.vue"),
    meta: { key: "forgotPassword" as AppRouteKey },
  },
];
