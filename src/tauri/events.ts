import { listen } from "@tauri-apps/api/event";
import { canUseTauriRuntime } from "./commands/_base";

// Register listeners for events the Rust backend pushes (background poll
// results, file-watcher notifications, etc). Call once from main.ts.
export async function registerBackendEventListeners() {
  if (!canUseTauriRuntime()) return;

  await listen("example-items-updated", () => {
    // e.g. trigger a refetch in the relevant composable/store
  });
}
