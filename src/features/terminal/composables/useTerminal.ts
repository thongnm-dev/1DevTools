import { ref } from "vue";
import { useI18n } from "vue-i18n";
import { Terminal } from "@xterm/xterm";
import { FitAddon } from "@xterm/addon-fit";
import { useToast } from "@/shared/composables/useToast";
import { canUseTauriRuntime, friendlyError } from "@/tauri/commands/_base";
import { terminalKill, terminalResize, terminalSpawn, terminalWrite } from "@/tauri/commands/terminal";
import { gitRepoInfo, gitStatus } from "@/tauri/commands/git";

/** Thông tin Git rút gọn cho toolbar (repo/branch/số file thay đổi) của một tab. */
export interface TerminalGitInfo {
  repoName: string;
  branch: string;
  diffCount: number;
}

/** Một tab terminal hiển thị trên UI (phần reactive, nhẹ). */
export interface TerminalTab {
  /** Định danh cục bộ, ổn định cho `v-for` và map instance xterm. */
  key: string;
  /** Id phiên PTY ở backend, có sau khi spawn thành công. */
  sessionId: string | null;
  /** Tên hiển thị trên tab. */
  title: string;
  /** Tiến trình shell đã kết thúc hay chưa. */
  exited: boolean;
  /** Mã thoát của tiến trình (nếu có). */
  exitCode: number | null;
  /** Thư mục làm việc riêng của tab này (rỗng = thư mục home). */
  startDir: string;
  /** Thông tin Git của `startDir` (null nếu không phải Git repo hoặc chưa kiểm tra). */
  git: TerminalGitInfo | null;
  /** Lệnh CLI agent (claude/codex/copilot) sẽ tự gõ vào shell ngay sau khi spawn lần đầu. */
  autoCommand?: string;
  /** Biến môi trường bổ sung cho phiên PTY (vd. CLAUDE_CONFIG_DIR). */
  env?: Record<string, string>;
}

/** Tùy chọn khi tạo tab mới: tiêu đề tùy chỉnh, lệnh agent tự chạy, thư mục làm việc ban đầu. */
export interface AddTabOptions {
  title?: string;
  autoCommand?: string;
  startDir?: string;
  env?: Record<string, string>;
}

/**
 * Instance xterm + phụ trợ cho một tab. Cố tình giữ NGOÀI reactive state của
 * Vue: xterm là object nặng, tự quản lý DOM/buffer riêng — bọc reactive sẽ làm
 * chậm và có thể gây lỗi. Chỉ `tabs` (mảng nhẹ) mới là reactive.
 */
interface TermEntry {
  term: Terminal;
  fit: FitAddon;
  ro: ResizeObserver | null;
  sessionId: string | null;
  /** Container DOM hiện tại đã `open()` — dùng để bỏ qua các lần gọi `bindContainer` lặp lại
   * với cùng phần tử (vd. do Vue gọi lại function ref trên mỗi lần re-render). */
  el: HTMLElement | null;
}

/** Giải mã base64 (byte thô từ PTY) thành `Uint8Array` để nạp vào xterm. */
function base64ToBytes(b64: string): Uint8Array {
  const bin = atob(b64);
  const bytes = new Uint8Array(bin.length);
  for (let i = 0; i < bin.length; i++) bytes[i] = bin.charCodeAt(i);
  return bytes;
}

/** Bảng màu tối cho terminal, đồng bộ với tông nền của app. */
const TERMINAL_THEME = {
  background: "#0b0f19",
  foreground: "#e5e9f0",
  cursor: "#e5e9f0",
  selectionBackground: "#33415580",
};

/**
 * Backend PTY trên Windows dùng ConPTY (qua `portable-pty`), vốn không phát ra
 * đúng chuỗi bao dòng (line-wrap) như PTY thật trên Unix. Nếu không khai báo
 * `windowsPty`, xterm hiểu sai việc bao dòng khi ứng dụng full-screen (vd. Claude
 * Code CLI) vẽ lại toàn màn hình, dẫn tới layout vỡ như đường viền box bị đứt/lệch.
 * Xem: https://github.com/xtermjs/xterm.js — option `windowsPty`.
 */
const IS_WINDOWS = navigator.userAgent.includes("Windows");

/**
 * State singleton ở module scope (không tạo lại mỗi lần gọi `useTerminal()`).
 *
 * Terminal cần sống xuyên suốt vòng đời app, độc lập với việc trang `/terminal`
 * đang mounted hay không — vd. màn hình Git có thể mở một tab terminal mới
 * (chạy AI agent) rồi điều hướng sang `/terminal` để xem, và phiên đó phải tiếp
 * tục chạy kể cả khi người dùng quay lại xem Git trong lúc agent đang chạy.
 */
const tabs = ref<TerminalTab[]>([]);
const activeKey = ref<string>("");
// Map instance xterm theo key tab (không reactive).
const entries = new Map<string, TermEntry>();
let counter = 0;

export function useTerminal() {
  const { t } = useI18n();
  const toast = useToast();

  function nextKey(): string {
    counter += 1;
    return `tab-${Date.now()}-${counter}`;
  }

  /**
   * Thêm một tab mới (chưa spawn — spawn diễn ra khi container được bind). Mỗi tab bắt đầu
   * ở thư mục home, có thể đổi riêng sau. Truyền `autoCommand` để tự động gõ lệnh (vd. khởi
   * chạy một CLI agent) ngay khi shell của tab sẵn sàng.
   */
  function addTab(opts?: AddTabOptions): string | null {
    if (!canUseTauriRuntime()) {
      toast.error(t("common.tauriRuntimeNotAvailable"));
      return null;
    }
    const key = nextKey();
    const startDir = opts?.startDir ?? "";
    const tab: TerminalTab = {
      key,
      sessionId: null,
      title: opts?.title ?? `Terminal ${tabs.value.length + 1}`,
      exited: false,
      exitCode: null,
      startDir,
      git: null,
      autoCommand: opts?.autoCommand,
      env: opts?.env,
    };
    tabs.value.push(tab);
    activeKey.value = key;
    if (startDir) void refreshGitInfo(tab);
    return key;
  }

  function setActive(key: string) {
    activeKey.value = key;
    // Refit khi chuyển tab vì container trước đó có thể bị ẩn (kích thước 0).
    requestAnimationFrame(() => fit(key));
  }

  /**
   * Gắn xterm vào phần tử DOM của tab. Gọi bởi component khi container sẵn sàng.
   * Idempotent: nếu tab đã gắn thì chỉ refit lại.
   */
  async function bindContainer(key: string, el: HTMLElement) {
    const existing = entries.get(key);
    if (existing) {
      // Vue gọi lại function ref (`:ref="(el) => ..."`) trên MỌI lần re-render của component
      // chứa nó (vd. khi đổi tab khiến `v-show`/`:class` cập nhật), không chỉ lúc mount —
      // nếu container chưa đổi thì bỏ qua, tránh gọi lại `term.open()` làm hỏng vùng render
      // hiện có (xoá sạch màn hình về màu đen dù buffer/PTY vẫn còn sống).
      if (existing.el === el) return;

      // Container DOM cũ đã đổi thật sự (component unmount/remount, hoặc "chuyển nhà" sang
      // container khác) — `open()` lại vào container mới để giữ nguyên buffer/scrollback và
      // phiên PTY đang chạy thay vì tạo phiên mới.
      existing.ro?.disconnect();
      existing.term.open(el);
      existing.el = el;
      const ro = new ResizeObserver(() => fit(key));
      ro.observe(el);
      existing.ro = ro;
      existing.term.focus();
      fit(key);
      return;
    }

    const term = new Terminal({
      fontFamily: '"Cascadia Code", "JetBrains Mono", Menlo, Consolas, monospace',
      fontSize: 13,
      cursorBlink: true,
      scrollback: 5000,
      theme: TERMINAL_THEME,
      windowsPty: IS_WINDOWS ? { backend: "conpty", buildNumber: 0 } : undefined,
    });
    const fitAddon = new FitAddon();
    term.loadAddon(fitAddon);
    term.open(el);

    const entry: TermEntry = { term, fit: fitAddon, ro: null, sessionId: null, el };
    entries.set(key, entry);

    // Gõ phím → gửi xuống PTY.
    term.onData((data) => {
      if (entry.sessionId) void terminalWrite(entry.sessionId, data).catch(() => undefined);
    });
    // xterm đổi số hàng/cột → báo backend resize PTY tương ứng.
    term.onResize(({ rows, cols }) => {
      if (entry.sessionId) void terminalResize(entry.sessionId, rows, cols).catch(() => undefined);
    });

    // Theo dõi thay đổi kích thước container để tự fit.
    const ro = new ResizeObserver(() => fit(key));
    ro.observe(el);
    entry.ro = ro;

    // Đợi 2 animation frame trước khi đo/fit lần đầu: nếu container nằm trong flex/grid
    // lồng nhiều tầng (vd. lưới nhiều terminal ở Sessions), `clientHeight` đọc ngay sau
    // `term.open()` có thể phản ánh kích thước tạm thời trước khi trình duyệt hoàn tất
    // layout pass cuối. Spawn PTY với rows/cols sai khiến một số CLI full-screen (vd.
    // Claude Code) chỉ quyết định MỘT LẦN lúc khởi động có vẽ khung nhập lệnh hay không
    // dựa trên kích thước ban đầu đó, và không tự "nâng cấp" giao diện dù sau này resize.
    await new Promise<void>((resolve) => requestAnimationFrame(() => requestAnimationFrame(() => resolve())));
    fitAddon.fit();

    const tab = tabs.value.find((t) => t.key === key);
    await spawnSession(key, tab?.startDir || undefined, tab?.env);

    // Lệnh agent (nếu có) chỉ tự chạy một lần, ngay sau khi shell của tab spawn lần đầu.
    const autoCommand = tab?.autoCommand;
    if (autoCommand && tab) {
      tab.autoCommand = undefined;
      const spawnedEntry = entries.get(key);
      if (spawnedEntry?.sessionId) {
        void terminalWrite(spawnedEntry.sessionId, `${autoCommand}\r`).catch(() => undefined);
      }
    }
  }

  /** Spawn phiên PTY cho một tab đã có sẵn xterm entry, gắn callback output/exit. */
  async function spawnSession(key: string, dir: string | undefined, env?: Record<string, string>) {
    const entry = entries.get(key);
    if (!entry) return;
    const { term } = entry;

    try {
      const sessionId = await terminalSpawn(
        term.rows,
        term.cols,
        (dataBase64) => term.write(base64ToBytes(dataBase64)),
        (code) => {
          const tab = tabs.value.find((t) => t.key === key);
          if (tab) {
            tab.exited = true;
            tab.exitCode = code;
          }
          term.writeln(`\r\n\x1b[90m[process exited${code === null ? "" : ` with code ${code}`}]\x1b[0m`);
        },
        dir,
        undefined,
        env,
      );
      entry.sessionId = sessionId;
      const tab = tabs.value.find((t) => t.key === key);
      if (tab) {
        tab.sessionId = sessionId;
        tab.exited = false;
        tab.exitCode = null;
      }
      // Container có thể đã fit lại (đúng kích thước) trong lúc chờ spawn — lúc đó
      // sessionId còn null nên onResize không kịp báo cho backend. Đồng bộ lại ngay
      // để PTY khớp với kích thước xterm hiện tại, tránh app con vẽ theo cols cũ (hẹp).
      void terminalResize(sessionId, term.rows, term.cols).catch(() => undefined);
    } catch (e) {
      term.writeln(`\x1b[31m${friendlyError(e)}\x1b[0m`);
    }
  }

  /** Fit lại terminal của tab theo kích thước container hiện tại. */
  function fit(key: string) {
    const entry = entries.get(key);
    if (!entry) return;
    // Container đang ẩn (vd. tab không active, `display: none` qua v-show) có kích thước 0,
    // nhưng `FitAddon.fit()` KHÔNG throw trong trường hợp đó — nó tự kẹp về tối thiểu 2 cột x
    // 1 hàng rồi gọi `term.resize()` thật sự, xoá sạch nội dung đang hiển thị (render service
    // bị clear). ResizeObserver vẫn báo resize khi container ẩn/hiện (0 <-> kích thước thật),
    // nên phải tự chặn ở đây: bỏ qua khi container không có kích thước thật, chỉ fit khi
    // container đang thực sự hiển thị.
    if (!entry.el || entry.el.offsetWidth === 0 || entry.el.offsetHeight === 0) return;
    try {
      entry.fit.fit();
    } catch {
      // Phòng hờ các trường hợp khác khiến fit() throw.
    }
  }

  /**
   * Gõ một lệnh vào shell của tab đang hoạt động (vd. khởi chạy CLI agent). Nếu shell chưa
   * kịp spawn xong (đang khởi tạo), lệnh được xếp hàng vào `autoCommand` để tự chạy sau.
   */
  function runCommand(key: string, command: string) {
    const tab = tabs.value.find((t) => t.key === key);
    if (!tab) return;
    const entry = entries.get(key);
    if (entry?.sessionId) {
      void terminalWrite(entry.sessionId, `${command}\r`).catch(() => undefined);
    } else {
      tab.autoCommand = command;
    }
  }

  /** Đóng một tab: kill PTY, giải phóng xterm, gỡ observer và cập nhật danh sách. */
  async function closeTab(key: string) {
    const entry = entries.get(key);
    if (entry) {
      entry.ro?.disconnect();
      if (entry.sessionId) await terminalKill(entry.sessionId).catch(() => undefined);
      entry.term.dispose();
      entries.delete(key);
    }

    const idx = tabs.value.findIndex((t) => t.key === key);
    if (idx !== -1) tabs.value.splice(idx, 1);

    if (activeKey.value === key) {
      const fallback = tabs.value[idx] ?? tabs.value[idx - 1] ?? tabs.value[0];
      activeKey.value = fallback ? fallback.key : "";
      if (fallback) requestAnimationFrame(() => fit(fallback.key));
    }
  }

  /**
   * Đổi thư mục làm việc riêng của một tab. Nếu tab đã có phiên PTY đang chạy,
   * kill phiên cũ và spawn lại shell mới ngay trong tab đó (giữ nguyên xterm/buffer).
   */
  async function changeTabDir(key: string, dir: string) {
    const tab = tabs.value.find((t) => t.key === key);
    if (!tab) return;
    tab.startDir = dir;
    void refreshGitInfo(tab);

    const entry = entries.get(key);
    if (!entry) return; // Chưa bind container — sẽ dùng startDir khi spawn lần đầu.

    if (entry.sessionId) await terminalKill(entry.sessionId).catch(() => undefined);
    entry.sessionId = null;
    tab.sessionId = null;
    entry.term.writeln(`\x1b[90m[restarting shell in ${dir}]\x1b[0m`);
    await spawnSession(key, dir || undefined, tab.env);
  }

  /** Kiểm tra `tab.startDir` có phải Git repo không; nếu có, nạp branch + số file thay đổi. */
  async function refreshGitInfo(tab: TerminalTab) {
    if (!tab.startDir) {
      tab.git = null;
      return;
    }
    try {
      const [info, status] = await Promise.all([gitRepoInfo(tab.startDir), gitStatus(tab.startDir)]);
      const repoName = info.path.split(/[\\/]/).filter(Boolean).pop() ?? info.path;
      tab.git = {
        repoName,
        branch: info.current_branch || (info.detached ? "detached HEAD" : "HEAD"),
        diffCount: status.staged.length + status.unstaged.length,
      };
    } catch {
      // Không phải Git repo (hoặc `git` không dùng được) — bỏ qua, không phải lỗi cần báo.
      tab.git = null;
    }
  }

  /** Mở sẵn 1 tab đầu tiên nếu đang chạy trong Tauri runtime. */
  function init() {
    if (!canUseTauriRuntime()) return;
    if (tabs.value.length === 0) addTab();
  }

  /** Dọn dẹp toàn bộ: kill + dispose mọi phiên terminal. */
  async function dispose() {
    for (const [key, entry] of entries) {
      entry.ro?.disconnect();
      if (entry.sessionId) await terminalKill(entry.sessionId).catch(() => undefined);
      entry.term.dispose();
      entries.delete(key);
    }
    tabs.value = [];
    activeKey.value = "";
  }

  return {
    tabs,
    activeKey,
    addTab,
    setActive,
    bindContainer,
    closeTab,
    changeTabDir,
    runCommand,
    fit,
    init,
    dispose,
  };
}
