import { invoke } from "@tauri-apps/api/core";
import { i18n } from "@/shared/i18n";
import { ERROR_CODE_KEYS } from "@/shared/i18n/errorCodes";

type TauriWindow = Window & {
  __TAURI_INTERNALS__?: { invoke?: unknown };
};

type BackendErrorPayload = { code: string; message: string };

function isBackendErrorPayload(value: unknown): value is BackendErrorPayload {
  return (
    typeof value === "object" &&
    value !== null &&
    typeof (value as BackendErrorPayload).code === "string" &&
    typeof (value as BackendErrorPayload).message === "string"
  );
}

export function canUseTauriRuntime(): boolean {
  return typeof window !== "undefined" && typeof (window as TauriWindow).__TAURI_INTERNALS__?.invoke === "function";
}

/**
 * True when a handful of commands (login, menu, permissions) should return
 * mock data instead of calling the real Rust backend, so the layout can be
 * reviewed without one. Applies to plain browser dev (no Tauri runtime at
 * all) and, when explicitly opted in via VITE_MOCK=true (see
 * .env.development.local), to `tauri:dev` too — there the Tauri runtime is
 * present but the Postgres backend may not be configured/reachable yet.
 * Never active outside of dev builds.
 */
export function isMockMode(): boolean {
  if (!import.meta.env.DEV) return false;
  return !canUseTauriRuntime() || import.meta.env.VITE_MOCK === "true";
}

export function friendlyError(error: unknown): string {
  if (isBackendErrorPayload(error)) {
    const key = ERROR_CODE_KEYS[error.code];
    return key ? i18n.global.t(key) : error.message;
  }

  const text = String(error);
  if (text.includes("__TAURI_INTERNALS__") || text.includes("reading 'invoke'")) {
    return i18n.global.t("common.tauriRuntimeNotAvailable");
  }
  return text;
}

export async function safeInvoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  if (!canUseTauriRuntime()) {
    throw new Error(i18n.global.t("common.tauriRuntimeNotAvailable"));
  }
  return invoke<T>(command, args);
}
