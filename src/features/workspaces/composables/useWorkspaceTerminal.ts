import { reactive } from "vue";
import { useTerminal } from "@/features/terminal/composables/useTerminal";

/**
 * Mỗi workspace có thể mở NHIỀU tab terminal song song (chạy nhiều tiến trình
 * cùng lúc) thay vì chỉ 1 tab như trước. `workspaceOfTab` (module-scope, không
 * reactive) nhớ tab nào thuộc workspace nào để lọc ra từ danh sách tab toàn cục
 * của `useTerminal`; đóng/mở panel (unmount/remount component) không làm mất
 * các phiên PTY đang chạy vì tab vẫn sống ở state module-scope của `useTerminal`.
 * `activeKeyByWorkspace` (reactive Map) nhớ tab đang active riêng của từng
 * workspace, độc lập với `activeKey` của trang Terminal chính (`/terminal`).
 */
const workspaceOfTab = new Map<string, number>();
const activeKeyByWorkspace = reactive(new Map<number, string>());

export function useWorkspaceTerminal() {
  const term = useTerminal();

  /** Danh sách tab (đúng thứ tự tạo) thuộc về một workspace. */
  function tabsFor(workspaceId: number) {
    return term.tabs.value.filter((t) => workspaceOfTab.get(t.key) === workspaceId);
  }

  /** Tab đang active của workspace; rơi về tab đầu tiên nếu chưa chọn hoặc tab active đã đóng. */
  function activeKeyFor(workspaceId: number): string {
    const tabs = tabsFor(workspaceId);
    const key = activeKeyByWorkspace.get(workspaceId);
    if (key && tabs.some((t) => t.key === key)) return key;
    const fallback = tabs[0]?.key ?? "";
    if (fallback) activeKeyByWorkspace.set(workspaceId, fallback);
    return fallback;
  }

  function setActive(workspaceId: number, key: string) {
    activeKeyByWorkspace.set(workspaceId, key);
    // Refit khi chuyển tab vì container trước đó có thể bị ẩn (kích thước 0).
    requestAnimationFrame(() => term.fit(key));
  }

  /** Mở tab terminal mới cho workspace và đưa nó thành active. */
  function addTab(workspaceId: number, title: string, startDir: string, autoCommand?: string, env?: Record<string, string>): string | null {
    const key = term.addTab({ title, startDir, autoCommand, env });
    if (key) {
      workspaceOfTab.set(key, workspaceId);
      setActive(workspaceId, key);
    }
    return key;
  }

  /** Đảm bảo workspace có ít nhất 1 tab — gọi khi panel terminal được mở lần đầu. */
  function ensureTab(workspaceId: number, title: string, startDir: string): string | null {
    const existing = activeKeyFor(workspaceId);
    if (existing) return existing;
    return addTab(workspaceId, title, startDir);
  }

  /** Đóng một tab cụ thể của workspace, tự chuyển active sang tab liền kề còn lại (nếu có). */
  async function closeTab(workspaceId: number, key: string) {
    const before = tabsFor(workspaceId);
    const idx = before.findIndex((t) => t.key === key);
    workspaceOfTab.delete(key);
    await term.closeTab(key);

    if (activeKeyByWorkspace.get(workspaceId) === key) {
      const remaining = tabsFor(workspaceId);
      const fallback = remaining[idx] ?? remaining[idx - 1] ?? remaining[0];
      if (fallback) setActive(workspaceId, fallback.key);
      else activeKeyByWorkspace.delete(workspaceId);
    }
  }

  /** Đóng toàn bộ tab terminal của workspace (gọi khi đóng cả workspace). */
  async function closeAllTabsFor(workspaceId: number) {
    for (const tab of tabsFor(workspaceId)) {
      workspaceOfTab.delete(tab.key);
      await term.closeTab(tab.key);
    }
    activeKeyByWorkspace.delete(workspaceId);
  }

  return { term, tabsFor, activeKeyFor, setActive, addTab, ensureTab, closeTab, closeAllTabsFor };
}
