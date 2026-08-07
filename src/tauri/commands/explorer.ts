import { safeInvoke } from "./_base";

/** Hiển thị `path` trong Explorer/Finder — dùng bởi màn hình Git Desktop ("Show in folder"). */
export function explorerOpen(path: string) {
  return safeInvoke<void>("explorer_open", { path });
}
