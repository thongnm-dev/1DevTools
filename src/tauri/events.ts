import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { canUseTauriRuntime } from "./commands/_base";

// Register listeners for events the Rust backend pushes (background poll
// results, file-watcher notifications, etc). Call once from main.ts.
export async function registerBackendEventListeners() {
  if (!canUseTauriRuntime()) return;

  await listen("example-items-updated", () => {
    // e.g. trigger a refetch in the relevant composable/store
  });
}

/** Event backend bắn khi file watcher phát hiện thay đổi trong working tree của repo Git đang theo dõi. */
export const GIT_REPO_CHANGED_EVENT = "git-repo-changed";

/**
 * Lắng nghe event `git-repo-changed` (file watcher nền cho repo Git đang mở).
 * Payload là đường dẫn repo đã thay đổi. Trả về hàm huỷ đăng ký; no-op nếu
 * không chạy trong Tauri runtime.
 */
export async function onGitRepoChanged(handler: (path: string) => void): Promise<UnlistenFn> {
  if (!canUseTauriRuntime()) {
    return () => {};
  }
  return listen<string>(GIT_REPO_CHANGED_EVENT, (event) => handler(event.payload));
}
