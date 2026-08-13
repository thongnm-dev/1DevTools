import { useTerminal } from "@/features/terminal/composables/useTerminal";

/**
 * Mỗi workspace có tối đa 1 tab terminal riêng cho panel "Terminal" ở
 * sidebar — map này (module-scope, không reactive) nhớ tab key đã tạo cho
 * từng workspace, để đóng/mở panel (unmount/remount component) không làm
 * mất phiên PTY đang chạy (giữ nguyên buffer + process, chỉ "chuyển nhà"
 * DOM container — xem `useTerminal.bindContainer`).
 */
const tabKeyByWorkspace = new Map<number, string>();

export function useWorkspaceTerminal() {
  const term = useTerminal();

  /** Lấy tab key hiện có của workspace, hoặc tạo mới nếu chưa có / đã bị đóng. */
  function ensureTab(workspaceId: number, title: string, startDir: string): string | null {
    const existing = tabKeyByWorkspace.get(workspaceId);
    if (existing && term.tabs.value.some((t) => t.key === existing)) {
      return existing;
    }
    const key = term.addTab({ title, startDir });
    if (key) tabKeyByWorkspace.set(workspaceId, key);
    return key;
  }

  /** Đóng phiên terminal riêng của workspace (gọi khi đóng cả workspace). */
  async function closeTabFor(workspaceId: number) {
    const key = tabKeyByWorkspace.get(workspaceId);
    if (!key) return;
    tabKeyByWorkspace.delete(workspaceId);
    await term.closeTab(key);
  }

  return { term, ensureTab, closeTabFor };
}
