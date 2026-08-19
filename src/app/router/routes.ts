import type { RouteRecordRaw } from "vue-router";
import type { AppRouteKey, MenuKey } from "@/models/app";

export type AppRoute = {
  key: AppRouteKey;
  path: string;
  requiresAuth?: boolean;
  title: string;
  subtitle: string;
  /** i18n keys (resolved via `t()` in AppHeader), e.g. "shell.breadcrumbs.governance". */
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
    key: "git",
    path: "/git",
    title: "Git",
    subtitle: "Browse, commit, and manage local Git repositories.",
  },
  {
    key: "terminal",
    path: "/terminal",
    title: "Terminal",
    subtitle: "Embedded shell with multiple tabs and a working-directory file tree.",
  },
  {
    key: "docker",
    path: "/docker",
    title: "Docker",
    subtitle: "Manage local Docker containers, images, and saved build/compose projects.",
  },
  {
    key: "gov-users",
    path: "/governance/users",
    title: "Users",
    subtitle: "Manage user accounts, roles, and access.",
    breadcrumbs: ["shell.breadcrumbs.governance", "shell.breadcrumbs.users"],
  },
  {
    key: "gov-roles",
    path: "/governance/roles",
    title: "Roles",
    subtitle: "Define roles and the users assigned to them.",
    breadcrumbs: ["shell.breadcrumbs.governance", "shell.breadcrumbs.roles"],
  },
  {
    key: "gov-menus",
    path: "/governance/menus",
    title: "Menus",
    subtitle: "Configure sidebar menu items, groups, and ordering.",
    breadcrumbs: ["shell.breadcrumbs.governance", "shell.breadcrumbs.menus"],
  },
  {
    key: "gov-permissions",
    path: "/governance/permissions",
    title: "Permissions",
    subtitle: "Grant menu access per role, with per-user overrides.",
    breadcrumbs: ["shell.breadcrumbs.governance", "shell.breadcrumbs.permissions"],
  },
  {
    key: "app-config",
    path: "/governance/app-config",
    title: "App Config",
    subtitle: "Edit the application configuration file (config.ini).",
    breadcrumbs: ["shell.breadcrumbs.governance", "shell.breadcrumbs.appConfig"],
  },
  {
    key: "master-data",
    path: "/governance/master-data",
    title: "Master Data",
    subtitle: "Manage shared categories used across the app.",
    breadcrumbs: ["shell.breadcrumbs.governance", "shell.breadcrumbs.masterData"],
  },
  {
    key: "ai-usage",
    path: "/ai/usage",
    title: "AI Usage",
    subtitle: "Manage AI accounts, monitor usage limits, and auto-switch providers.",
    breadcrumbs: ["shell.breadcrumbs.ai", "shell.breadcrumbs.usage"],
  },
  {
    key: "ai-providers",
    path: "/governance/providers",
    title: "AI Agent Providers",
    subtitle: "Register and enable which AI agents can be used across the system.",
    breadcrumbs: ["shell.breadcrumbs.governance", "shell.breadcrumbs.providers"],
  },
  {
    key: "ai-provider-models",
    path: "/governance/provider-models",
    title: "AI Provider Models",
    subtitle: "Register and enable which provider models can be used across the system.",
    breadcrumbs: ["shell.breadcrumbs.governance", "shell.breadcrumbs.providerModels"],
  },
  {
    key: "ai-tasks",
    path: "/ai/tasks",
    title: "AI Tasks",
    subtitle: "Track tasks and the workflow step they are currently on.",
    breadcrumbs: ["shell.breadcrumbs.ai", "shell.breadcrumbs.tasks"],
  },
  {
    key: "ai-cowork",
    path: "/ai/cowork",
    title: "AI Cowork",
    subtitle: "Run tasks through a workflow's steps, one terminal per step.",
    breadcrumbs: ["shell.breadcrumbs.ai", "shell.breadcrumbs.cowork"],
  },
  {
    key: "workspaces",
    path: "/workspaces",
    title: "Workspaces",
    subtitle: "Open and switch between multiple projects in one window.",
  },
  {
    key: "workflow",
    path: "/workflow",
    title: "Workflow",
    subtitle: "Build reusable step sequences (skill, prompt, runner, terminal) for your projects.",
  },
  {
    key: "skill",
    path: "/skill",
    title: "Skill",
    subtitle: "Reusable instructions you can load into an agent session.",
  },
  {
    key: "prompt",
    path: "/prompt",
    title: "Prompt",
    subtitle: "Reusable prompt snippets, with {{variable}} placeholders.",
  },
  {
    key: "settings",
    path: "/settings",
    title: "Settings",
    subtitle: "Manage your profile and preferences.",
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
    path: "/git",
    component: () => import("@/features/git/components/GitDesktopPage.vue"),
    meta: { key: "git" as MenuKey },
  },
  {
    path: "/terminal",
    component: () => import("@/features/terminal/components/TerminalPage.vue"),
    meta: { key: "terminal" as MenuKey },
  },
  {
    path: "/docker",
    component: () => import("@/features/docker/components/DockerPage.vue"),
    meta: { key: "docker" as MenuKey },
  },
  {
    path: "/governance/users",
    component: () => import("@/features/governance/components/GovernanceUsersPage.vue"),
    meta: { key: "gov-users" as MenuKey },
  },
  {
    path: "/governance/roles",
    component: () => import("@/features/governance/components/GovernanceRolesPage.vue"),
    meta: { key: "gov-roles" as MenuKey },
  },
  {
    path: "/governance/menus",
    component: () => import("@/features/governance/components/GovernanceMenusPage.vue"),
    meta: { key: "gov-menus" as MenuKey },
  },
  {
    path: "/governance/permissions",
    component: () => import("@/features/governance/components/GovernancePermissionsPage.vue"),
    meta: { key: "gov-permissions" as MenuKey },
  },
  {
    path: "/governance/app-config",
    component: () => import("@/features/governance/components/AppConfigPage.vue"),
    meta: { key: "app-config" as MenuKey },
  },
  {
    path: "/governance/master-data",
    component: () => import("@/features/master-data/components/MasterDataPage.vue"),
    meta: { key: "master-data" as MenuKey },
  },
  {
    path: "/ai/usage",
    component: () => import("@/features/ai-agent/components/AiUsagePage.vue"),
    meta: { key: "ai-usage" as MenuKey },
  },
  {
    path: "/governance/providers",
    component: () => import("@/features/ai-agent/components/AgentProviderPage.vue"),
    meta: { key: "ai-providers" as MenuKey },
  },
  {
    path: "/governance/provider-models",
    component: () => import("@/features/ai-agent/components/AgentProviderModelPage.vue"),
    meta: { key: "ai-provider-models" as MenuKey },
  },
  {
    path: "/ai/tasks",
    component: () => import("@/features/task/components/TaskPage.vue"),
    meta: { key: "ai-tasks" as MenuKey },
  },
  {
    path: "/ai/cowork",
    component: () => import("@/features/ai-agent/components/AiCoworkPage.vue"),
    meta: { key: "ai-cowork" as MenuKey },
  },
  {
    path: "/workspaces",
    component: () => import("@/features/workspaces/components/WorkspacesPage.vue"),
    meta: { key: "workspaces" as MenuKey },
  },
  {
    path: "/workflow",
    component: () => import("@/features/workflow/components/WorkflowPage.vue"),
    meta: { key: "workflow" as MenuKey },
  },
  {
    path: "/skill",
    component: () => import("@/features/skill/components/SkillPage.vue"),
    meta: { key: "skill" as MenuKey },
  },
  {
    path: "/prompt",
    component: () => import("@/features/prompt/components/PromptPage.vue"),
    meta: { key: "prompt" as MenuKey },
  },
  {
    path: "/settings",
    component: () => import("@/features/settings/components/SettingsPage.vue"),
    meta: { key: "settings" as MenuKey },
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
