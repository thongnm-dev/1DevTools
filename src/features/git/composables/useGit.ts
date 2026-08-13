import { computed, onUnmounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { open } from "@tauri-apps/plugin-dialog";

import { canUseTauriRuntime, friendlyError } from "@/tauri/commands/_base";
import { explorerOpen } from "@/tauri/commands/explorer";
import { onGitRepoChanged } from "@/tauri/events";
import {
  gitAddRepo,
  gitBlame,
  gitBranches,
  gitCheckoutBranch,
  gitCherryPick,
  gitCherryPickAbort,
  gitCherryPickContinue,
  gitCleanupDelete,
  gitCleanupScan,
  gitClone,
  gitAmendCommit,
  gitCommitNoEdit,
  gitCompare,
  gitCompareFileDiff,
  gitCreatePullRequest,
  gitGraph,
  gitListConflicts,
  gitListPullRequests,
  gitMerge,
  gitMergeAbort,
  gitOpenTerminal,
  gitOpenUrl,
  gitOpenVscode,
  gitResolveConflict,
  gitTagCreate,
  gitTagDelete,
  gitTagList,
  gitCommit,
  gitCommitDetail,
  gitCommitFileDiff,
  gitCreateBranch,
  gitDeleteBranch,
  gitDiscard,
  gitFetch,
  gitFileDiff,
  gitListRepos,
  gitLog,
  gitLogSearch,
  gitPull,
  gitPush,
  gitRebase,
  gitRebaseAbort,
  gitRebaseContinue,
  gitRemoveRepo,
  gitRepoInfo,
  gitReset,
  gitRevert,
  gitRevertAbort,
  gitStage,
  gitStashApply,
  gitStashDrop,
  gitStashList,
  gitStashSave,
  gitStatus,
  gitTouchRepo,
  gitUndoLastCommit,
  gitUnstage,
  gitWatchStart,
  gitWatchStop,
  gitWorktreeAdd,
  gitWorktreeList,
  gitWorktreeRemove,
} from "@/tauri/commands/git";
import type {
  GitBlameLine,
  GitBranch,
  GitCommit,
  GitCommitDetail,
  GitComparison,
  GitDiff,
  GitFileChange,
  GitGraphCommit,
  GitProgress,
  GitPullRequest,
  GitRepo,
  GitRepoInfo,
  GitStash,
  GitTag,
  GitWorktree,
} from "@/models/git";
import { useToast } from "@/shared/composables/useToast";

const HISTORY_PAGE_SIZE = 20;
const ACTIVE_REPO_KEY = "git.activeRepoId";

/** File đang được chọn để xem diff. */
type SelectedFile = {
  path: string;
  staged: boolean;
  untracked: boolean;
};

export type GitTab = "changes" | "history";

export function useGit() {
  const { t } = useI18n();
  const toast = useToast();

  const repos = ref<GitRepo[]>([]);
  const activeRepo = ref<GitRepo | null>(null);
  const info = ref<GitRepoInfo | null>(null);

  const staged = ref<GitFileChange[]>([]);
  const unstaged = ref<GitFileChange[]>([]);
  const branches = ref<GitBranch[]>([]);
  const stashes = ref<GitStash[]>([]);
  const worktrees = ref<GitWorktree[]>([]);
  const tags = ref<GitTag[]>([]);
  const commits = ref<GitCommit[]>([]);
  const historyHasMore = ref(true);
  const historyLoadingMore = ref(false);

  const comparison = ref<GitComparison | null>(null);
  const comparisonDiff = ref<GitDiff | null>(null);

  const pullRequests = ref<GitPullRequest[]>([]);
  const pullRequestsLoading = ref(false);

  const conflicts = ref<string[]>([]);

  const graphCommits = ref<GitGraphCommit[]>([]);
  const graphLoading = ref(false);

  // Commit browser (dialog duyệt commit + copy SHA).
  const browserCommits = ref<GitCommit[]>([]);
  const browserFiles = ref<GitFileChange[]>([]);
  const browserDiff = ref<GitDiff | null>(null);
  const browserLoading = ref(false);

  // Git blame (ai sửa dòng nào lần cuối).
  const blameFile = ref("");
  const blameLines = ref<GitBlameLine[]>([]);
  const blameLoading = ref(false);
  const blameSelectedHash = ref("");
  const blameDetail = ref<GitCommitDetail | null>(null);

  const selectedFile = ref<SelectedFile | null>(null);
  const diff = ref<GitDiff | null>(null);
  const diffLoading = ref(false);

  const selectedCommit = ref<GitCommit | null>(null);
  const commitDetail = ref<GitCommitDetail | null>(null);
  const commitFileDiff = ref<GitDiff | null>(null);

  const commitMessage = ref("");
  const tab = ref<GitTab>("changes");

  // Cờ trạng thái — dùng cho spinner cục bộ, không blank cả màn hình.
  const loadingRepo = ref(false);
  const refreshing = ref(false);
  const committing = ref(false);
  const syncing = ref(false);
  const busyMessage = ref("");
  const syncProgress = ref<GitProgress | null>(null);

  const runtimeAvailable = computed(() => canUseTauriRuntime());
  const hasChanges = computed(() => staged.value.length + unstaged.value.length > 0);
  const canCommit = computed(
    () => staged.value.length > 0 && commitMessage.value.trim().length > 0 && !committing.value,
  );

  const localBranches = computed(() => branches.value.filter((b) => !b.is_remote));
  const remoteBranches = computed(() => branches.value.filter((b) => b.is_remote));

  function reportError(prefix: string, e: unknown) {
    toast.error(`${prefix}: ${friendlyError(e)}`);
  }

  // === Danh sách repo ===

  async function loadRepos() {
    if (!runtimeAvailable.value) return;
    try {
      repos.value = await gitListRepos();
      const savedId = Number(localStorage.getItem(ACTIVE_REPO_KEY) ?? "");
      const target =
        repos.value.find((r) => r.id === savedId) ?? repos.value[0] ?? null;
      if (target) await openRepo(target);
    } catch (e) {
      reportError(t("git.toast.reposLoadFailed"), e);
    }
  }

  async function addRepoFromDialog() {
    if (!runtimeAvailable.value) return;
    try {
      const picked = await open({ directory: true, title: t("git.toast.selectGitRepoDirTitle") });
      if (!picked || typeof picked !== "string") return;
      const repo = await gitAddRepo(picked);
      if (!repos.value.some((r) => r.id === repo.id)) {
        repos.value = [repo, ...repos.value];
      }
      await openRepo(repo);
      toast.success(t("git.toast.repoAdded", { name: repo.name }));
    } catch (e) {
      reportError(t("git.toast.repoAddFailed"), e);
    }
  }

  async function removeRepo(repo: GitRepo) {
    try {
      await gitRemoveRepo(repo.id);
      repos.value = repos.value.filter((r) => r.id !== repo.id);
      if (activeRepo.value?.id === repo.id) {
        activeRepo.value = null;
        resetRepoState();
        const next = repos.value[0];
        if (next) {
          await openRepo(next);
        } else {
          stopWatch();
        }
      }
      toast.success(t("git.toast.repoRemoved", { name: repo.name }));
    } catch (e) {
      reportError(t("git.toast.repoRemoveFailed"), e);
    }
  }

  function resetRepoState() {
    info.value = null;
    staged.value = [];
    unstaged.value = [];
    branches.value = [];
    stashes.value = [];
    commits.value = [];
    historyHasMore.value = true;
    historyLoadingMore.value = false;
    selectedFile.value = null;
    diff.value = null;
    selectedCommit.value = null;
    commitDetail.value = null;
    commitFileDiff.value = null;
  }

  /**
   * `persist`: mặc định `true` — ghi lại repo vừa mở vào `ACTIVE_REPO_KEY` để
   * lần sau mở màn Git Desktop tự chọn lại. Truyền `false` khi dùng instance
   * này để nhúng 1 repo cố định vào nơi khác (ví dụ panel Git trong
   * Workspaces) — tránh việc mở repo ở đó ghi đè "repo đang xem" của màn Git
   * Desktop độc lập.
   */
  async function openRepo(repo: GitRepo, options?: { persist?: boolean }) {
    activeRepo.value = repo;
    if (options?.persist !== false) {
      localStorage.setItem(ACTIVE_REPO_KEY, String(repo.id));
    }
    loadingRepo.value = true;
    resetRepoState();
    try {
      await Promise.all([refreshStatusAndInfo(), refreshBranches(), refreshStashes()]);
      gitTouchRepo(repo.id).catch(() => {});
      if (watchedPath && watchedPath !== repo.path) {
        gitWatchStop(watchedPath).catch(() => {});
      }
      watchedPath = repo.path;
      gitWatchStart(repo.path).catch(() => {});
    } catch (e) {
      reportError(t("git.toast.repoOpenFailed"), e);
    } finally {
      loadingRepo.value = false;
    }
  }

  const repoPath = () => activeRepo.value?.path ?? "";

  // === Theo dõi thay đổi file trên đĩa (auto-refresh tab Changes) ===
  //
  // `watchedPath` theo dõi path mà RIÊNG instance `useGit()` này đang xem —
  // mỗi instance tự quản lý watcher của mình (backend hỗ trợ nhiều path theo
  // dõi đồng thời), nên nhiều instance (ví dụ mỗi workspace 1 instance) có
  // thể cùng theo dõi các repo khác nhau mà không dừng lẫn nhau.

  let watchedPath = "";
  function stopWatch() {
    if (!watchedPath) return;
    const path = watchedPath;
    watchedPath = "";
    gitWatchStop(path).catch(() => {});
  }

  let repoChangedUnlisten: (() => void) | null = null;
  onGitRepoChanged((changedPath) => {
    if (changedPath === repoPath()) void refreshStatusAndInfo();
  }).then((un) => {
    repoChangedUnlisten = un;
  });

  onUnmounted(() => {
    stopWatch();
    repoChangedUnlisten?.();
  });

  // === Refresh (giữ dữ liệu cũ trong lúc tải để tránh nháy màn hình) ===

  async function refreshStatusAndInfo() {
    const path = repoPath();
    if (!path) return;
    refreshing.value = true;
    try {
      const [st, nfo] = await Promise.all([gitStatus(path), gitRepoInfo(path)]);
      staged.value = st.staged;
      unstaged.value = st.unstaged;
      info.value = nfo;
      reconcileSelectedFile();
    } catch (e) {
      reportError(t("git.toast.statusLoadFailed"), e);
    } finally {
      refreshing.value = false;
    }
  }

  async function refreshBranches() {
    const path = repoPath();
    if (!path) return;
    try {
      branches.value = await gitBranches(path);
    } catch (e) {
      reportError(t("git.toast.branchesLoadFailed"), e);
    }
  }

  async function refreshStashes() {
    const path = repoPath();
    if (!path) return;
    try {
      stashes.value = await gitStashList(path);
    } catch (e) {
      reportError(t("git.toast.stashesLoadFailed"), e);
    }
  }

  /** Nếu file đang chọn không còn trong danh sách → bỏ chọn; ngược lại nạp lại diff. */
  function reconcileSelectedFile() {
    const sel = selectedFile.value;
    if (!sel) return;
    const list = sel.staged ? staged.value : unstaged.value;
    const found = list.find((f) => f.path === sel.path);
    if (!found) {
      selectedFile.value = null;
      diff.value = null;
    } else {
      void loadDiff(found, sel.staged);
    }
  }

  // === Diff (Changes tab) ===

  async function selectFile(file: GitFileChange, isStaged: boolean) {
    selectedFile.value = { path: file.path, staged: isStaged, untracked: file.untracked };
    await loadDiff(file, isStaged);
  }

  async function loadDiff(file: GitFileChange, isStaged: boolean) {
    const path = repoPath();
    if (!path) return;
    diffLoading.value = true;
    try {
      diff.value = await gitFileDiff(path, file.path, isStaged, file.untracked && !isStaged);
    } catch (e) {
      reportError(t("git.toast.diffReadFailed"), e);
      diff.value = null;
    } finally {
      diffLoading.value = false;
    }
  }

  // === Staging ===

  async function stageFiles(files: string[]) {
    await mutate(() => gitStage(repoPath(), files), t("git.toast.stageFailed"));
  }
  async function unstageFiles(files: string[]) {
    await mutate(() => gitUnstage(repoPath(), files), t("git.toast.unstageFailed"));
  }
  async function stageAll() {
    await mutate(() => gitStage(repoPath(), []), t("git.toast.stageAllFailed"));
  }
  async function unstageAll() {
    await mutate(() => gitUnstage(repoPath(), []), t("git.toast.unstageAllFailed"));
  }
  async function discardFiles(files: string[]) {
    await mutate(() => gitDiscard(repoPath(), files), t("git.toast.discardFailed"));
  }

  /** Chạy một mutation rồi refresh status — giữ UI mượt (không blank). */
  async function mutate(fn: () => Promise<unknown>, errPrefix: string) {
    if (!repoPath()) return;
    try {
      await fn();
      await refreshStatusAndInfo();
    } catch (e) {
      reportError(errPrefix, e);
    }
  }

  // === Commit ===

  async function commit() {
    const path = repoPath();
    if (!path || !canCommit.value) return;
    committing.value = true;
    try {
      await gitCommit(path, commitMessage.value.trim());
      commitMessage.value = "";
      selectedFile.value = null;
      diff.value = null;
      await Promise.all([refreshStatusAndInfo(), refreshBranches()]);
      if (tab.value === "history") await loadHistory();
      toast.success(t("git.toast.commitDone"));
    } catch (e) {
      reportError(t("git.toast.commitFailed"), e);
    } finally {
      committing.value = false;
    }
  }

  // === Sync: fetch / pull / push ===

  async function fetch() {
    await runSync(
      (onP) => gitFetch(repoPath(), onP),
      t("git.toast.fetching"),
      t("git.toast.fetchDone"),
      t("git.toast.fetchFailed"),
    );
  }
  async function pull() {
    await runSync(
      (onP) => gitPull(repoPath(), onP),
      t("git.toast.pulling"),
      t("git.toast.pullDone"),
      t("git.toast.pullFailed"),
    );
  }
  async function push() {
    await runSync(
      (onP) => gitPush(repoPath(), onP),
      t("git.toast.pushing"),
      t("git.toast.pushDone"),
      t("git.toast.pushFailed"),
    );
  }

  async function runSync(
    fn: (onProgress: (p: GitProgress) => void) => Promise<string>,
    busy: string,
    ok: string,
    errPrefix: string,
  ) {
    const path = repoPath();
    if (!path || syncing.value) return;
    syncing.value = true;
    busyMessage.value = busy;
    syncProgress.value = null;
    try {
      await fn((p) => {
        syncProgress.value = p;
      });
      await Promise.all([refreshStatusAndInfo(), refreshBranches()]);
      if (tab.value === "history") await loadHistory();
      toast.success(ok);
    } catch (e) {
      reportError(errPrefix, e);
    } finally {
      syncing.value = false;
      busyMessage.value = "";
      syncProgress.value = null;
    }
  }

  // === History tab ===

  async function loadHistory() {
    const path = repoPath();
    if (!path) return;
    try {
      const page = await gitLogSearch(path, "", "", "", "", "", 0, HISTORY_PAGE_SIZE);
      commits.value = page;
      historyHasMore.value = page.length === HISTORY_PAGE_SIZE;
      if (commits.value.length && !selectedCommit.value) {
        await selectCommit(commits.value[0]);
      }
    } catch (e) {
      reportError(t("git.toast.historyLoadFailed"), e);
    }
  }

  /** Nạp thêm 20 commit tiếp theo khi cuộn tới cuối danh sách history. */
  async function loadMoreHistory() {
    const path = repoPath();
    if (!path || historyLoadingMore.value || !historyHasMore.value) return;
    historyLoadingMore.value = true;
    try {
      const page = await gitLogSearch(path, "", "", "", "", "", commits.value.length, HISTORY_PAGE_SIZE);
      commits.value = [...commits.value, ...page];
      historyHasMore.value = page.length === HISTORY_PAGE_SIZE;
    } catch (e) {
      reportError(t("git.toast.historyLoadFailed"), e);
    } finally {
      historyLoadingMore.value = false;
    }
  }

  /** Refresh sau khi lịch sử/HEAD thay đổi (undo, reset, checkout commit…). */
  async function refreshAfterHistoryChange() {
    selectedCommit.value = null;
    commits.value = [];
    historyHasMore.value = true;
    selectedFile.value = null;
    diff.value = null;
    await Promise.all([refreshStatusAndInfo(), refreshBranches()]);
    if (tab.value === "history") await loadHistory();
  }

  /** Undo commit gần nhất (giữ thay đổi ở staged). */
  async function undoLastCommit() {
    const path = repoPath();
    if (!path) return;
    busyMessage.value = t("git.toast.undoing");
    try {
      await gitUndoLastCommit(path);
      await refreshAfterHistoryChange();
      toast.success(t("git.toast.undone"));
    } catch (e) {
      reportError(t("git.toast.undoFailed"), e);
    } finally {
      busyMessage.value = "";
    }
  }

  async function amendCommit(message: string) {
    const path = repoPath();
    if (!path || !message.trim()) return;
    busyMessage.value = t("git.toast.amending");
    try {
      await gitAmendCommit(path, message.trim());
      await refreshAfterHistoryChange();
      toast.success(t("git.toast.amended"));
    } catch (e) {
      reportError(t("git.toast.amendFailed"), e);
    } finally {
      busyMessage.value = "";
    }
  }

  /** Reset branch hiện tại về một commit. */
  async function resetTo(hash: string, mode: "soft" | "mixed" | "hard") {
    const path = repoPath();
    if (!path) return;
    busyMessage.value = t("git.toast.resetting");
    try {
      await gitReset(path, hash, mode);
      await refreshAfterHistoryChange();
      toast.success(t("git.toast.resetDone", { mode }));
    } catch (e) {
      reportError(t("git.toast.resetFailed"), e);
    } finally {
      busyMessage.value = "";
    }
  }

  /** Checkout một commit (detached HEAD). */
  async function checkoutCommit(hash: string) {
    const path = repoPath();
    if (!path) return;
    busyMessage.value = t("git.toast.checkingOutCommit");
    try {
      await gitCheckoutBranch(path, hash);
      await refreshAfterHistoryChange();
      toast.success(t("git.toast.checkoutCommitDone"));
    } catch (e) {
      reportError(t("git.toast.checkoutCommitFailed"), e);
    } finally {
      busyMessage.value = "";
    }
  }

  /** Tạo branch mới tại một commit cụ thể rồi checkout sang đó. */
  async function createBranchAt(name: string, from: string) {
    const path = repoPath();
    if (!path || !name.trim()) return;
    try {
      await gitCreateBranch(path, name.trim(), from);
      await refreshAfterHistoryChange();
      toast.success(t("git.toast.branchAtCreated", { name: name.trim() }));
    } catch (e) {
      reportError(t("git.toast.branchCreateFailed"), e);
    }
  }

  /** Copy một đoạn text vào clipboard. */
  async function copyText(text: string, label: string) {
    try {
      await navigator.clipboard.writeText(text);
      toast.success(t("git.toast.copied", { label }));
    } catch (e) {
      reportError(t("git.toast.copyFailed"), e);
    }
  }

  async function selectCommit(c: GitCommit) {
    const path = repoPath();
    if (!path) return;
    selectedCommit.value = c;
    commitFileDiff.value = null;
    try {
      commitDetail.value = await gitCommitDetail(path, c.hash);
    } catch (e) {
      reportError(t("git.toast.commitReadFailed"), e);
    }
  }

  async function selectCommitFile(file: GitFileChange) {
    const path = repoPath();
    const c = selectedCommit.value;
    if (!path || !c) return;
    diffLoading.value = true;
    try {
      commitFileDiff.value = await gitCommitFileDiff(path, c.hash, file.path);
    } catch (e) {
      reportError(t("git.toast.diffReadFailed"), e);
    } finally {
      diffLoading.value = false;
    }
  }

  function switchTab(next: GitTab) {
    tab.value = next;
    if (next === "history" && commits.value.length === 0) void loadHistory();
  }

  // === Branch ===

  async function checkoutBranch(name: string) {
    const path = repoPath();
    if (!path) return;
    busyMessage.value = t("git.toast.switchingTo", { name });
    try {
      await gitCheckoutBranch(path, name);
      selectedFile.value = null;
      diff.value = null;
      selectedCommit.value = null;
      commits.value = [];
      await Promise.all([refreshStatusAndInfo(), refreshBranches()]);
      if (tab.value === "history") await loadHistory();
      toast.success(t("git.toast.switchedTo", { name }));
    } catch (e) {
      reportError(t("git.toast.branchSwitchFailed"), e);
    } finally {
      busyMessage.value = "";
    }
  }

  async function createBranch(name: string) {
    const path = repoPath();
    if (!path || !name.trim()) return;
    try {
      await gitCreateBranch(path, name.trim());
      await Promise.all([refreshStatusAndInfo(), refreshBranches()]);
      toast.success(t("git.toast.branchCreatedAndSwitched", { name: name.trim() }));
    } catch (e) {
      reportError(t("git.toast.branchCreateFailed"), e);
    }
  }

  async function deleteBranch(name: string, force: boolean) {
    const path = repoPath();
    if (!path) return;
    try {
      await gitDeleteBranch(path, name, force);
      await refreshBranches();
      toast.success(t("git.toast.branchDeleted", { name }));
    } catch (e) {
      reportError(t("git.toast.branchDeleteFailed"), e);
    }
  }

  // === Stash ===

  async function stashSave(message: string) {
    const path = repoPath();
    if (!path) return;
    try {
      await gitStashSave(path, message);
      await Promise.all([refreshStatusAndInfo(), refreshStashes()]);
      toast.success(t("git.toast.stashSaved"));
    } catch (e) {
      reportError(t("git.toast.stashSaveFailed"), e);
    }
  }

  async function stashApply(reference: string, pop: boolean) {
    const path = repoPath();
    if (!path) return;
    try {
      await gitStashApply(path, reference, pop);
      await Promise.all([refreshStatusAndInfo(), refreshStashes()]);
      toast.success(pop ? t("git.toast.stashApplyPopDone") : t("git.toast.stashApplyDone"));
    } catch (e) {
      reportError(t("git.toast.stashApplyFailed"), e);
    }
  }

  async function stashDrop(reference: string) {
    const path = repoPath();
    if (!path) return;
    try {
      await gitStashDrop(path, reference);
      await refreshStashes();
      toast.success(t("git.toast.stashDropped"));
    } catch (e) {
      reportError(t("git.toast.stashDropFailed"), e);
    }
  }

  // === Revert ===

  async function revert(hash: string) {
    const path = repoPath();
    if (!path) return;
    busyMessage.value = t("git.toast.reverting");
    try {
      await gitRevert(path, hash);
      await Promise.all([refreshStatusAndInfo(), refreshBranches(), loadHistory()]);
      toast.success(t("git.toast.reverted"));
    } catch (e) {
      reportError(t("git.toast.revertFailed"), e);
    } finally {
      busyMessage.value = "";
    }
  }

  async function revertAbort() {
    const path = repoPath();
    if (!path) return;
    try {
      await gitRevertAbort(path);
      await refreshStatusAndInfo();
      toast.success(t("git.toast.revertAborted"));
    } catch (e) {
      reportError(t("git.toast.revertAbortFailed"), e);
    }
  }

  // === Rebase ===

  async function rebaseOnto(onto: string) {
    const path = repoPath();
    if (!path || !onto.trim()) return;
    busyMessage.value = t("git.toast.rebasing", { onto });
    try {
      await gitRebase(path, onto);
      selectedCommit.value = null;
      commits.value = [];
      await Promise.all([refreshStatusAndInfo(), refreshBranches()]);
      if (tab.value === "history") await loadHistory();
      toast.success(t("git.toast.rebased", { onto }));
    } catch (e) {
      reportError(t("git.toast.rebaseFailed"), e);
      await refreshStatusAndInfo();
    } finally {
      busyMessage.value = "";
    }
  }

  async function rebaseAbort() {
    const path = repoPath();
    if (!path) return;
    busyMessage.value = t("git.toast.rebaseAborting");
    try {
      await gitRebaseAbort(path);
      await Promise.all([refreshStatusAndInfo(), refreshBranches()]);
      toast.success(t("git.toast.rebaseAborted"));
    } catch (e) {
      reportError(t("git.toast.rebaseAbortFailed"), e);
    } finally {
      busyMessage.value = "";
    }
  }

  async function rebaseContinue() {
    const path = repoPath();
    if (!path) return;
    busyMessage.value = t("git.toast.rebaseContinuing");
    try {
      await gitRebaseContinue(path);
      await Promise.all([refreshStatusAndInfo(), refreshBranches()]);
      if (tab.value === "history") await loadHistory();
      toast.success(t("git.toast.rebaseContinued"));
    } catch (e) {
      reportError(t("git.toast.rebaseContinueFailed"), e);
      await refreshStatusAndInfo();
    } finally {
      busyMessage.value = "";
    }
  }

  // === Cherry-pick ===

  async function cherryPick(hash: string) {
    const path = repoPath();
    if (!path) return;
    busyMessage.value = t("git.toast.cherryPicking");
    try {
      await gitCherryPick(path, hash);
      await Promise.all([refreshStatusAndInfo(), refreshBranches()]);
      if (tab.value === "history") await loadHistory();
      toast.success(t("git.toast.cherryPicked"));
    } catch (e) {
      reportError(t("git.toast.cherryPickFailed"), e);
      await refreshStatusAndInfo();
    } finally {
      busyMessage.value = "";
    }
  }

  async function cherryPickAbort() {
    const path = repoPath();
    if (!path) return;
    busyMessage.value = t("git.toast.cherryPickAborting");
    try {
      await gitCherryPickAbort(path);
      await refreshStatusAndInfo();
      toast.success(t("git.toast.cherryPickAborted"));
    } catch (e) {
      reportError(t("git.toast.cherryPickAbortFailed"), e);
    } finally {
      busyMessage.value = "";
    }
  }

  async function cherryPickContinue() {
    const path = repoPath();
    if (!path) return;
    busyMessage.value = t("git.toast.cherryPickContinuing");
    try {
      await gitCherryPickContinue(path);
      await Promise.all([refreshStatusAndInfo(), refreshBranches()]);
      if (tab.value === "history") await loadHistory();
      toast.success(t("git.toast.cherryPickContinued"));
    } catch (e) {
      reportError(t("git.toast.cherryPickContinueFailed"), e);
      await refreshStatusAndInfo();
    } finally {
      busyMessage.value = "";
    }
  }

  // === Worktree ===

  async function loadWorktrees() {
    const path = repoPath();
    if (!path) return;
    try {
      worktrees.value = await gitWorktreeList(path);
    } catch (e) {
      reportError(t("git.toast.worktreesLoadFailed"), e);
    }
  }

  /** Tạo worktree. Trả về đường dẫn đã tạo (rỗng nếu thất bại). */
  async function worktreeAdd(
    worktreePath: string,
    branch: string,
    newBranch: string,
  ): Promise<string> {
    const path = repoPath();
    if (!path || !worktreePath.trim()) return "";
    busyMessage.value = t("git.toast.worktreeCreating");
    try {
      const created = await gitWorktreeAdd(path, worktreePath, branch, newBranch);
      await Promise.all([loadWorktrees(), refreshBranches()]);
      toast.success(t("git.toast.worktreeCreated"));
      return created;
    } catch (e) {
      reportError(t("git.toast.worktreeCreateFailed"), e);
      return "";
    } finally {
      busyMessage.value = "";
    }
  }

  async function worktreeRemove(worktreePath: string, force: boolean) {
    const path = repoPath();
    if (!path) return;
    try {
      await gitWorktreeRemove(path, worktreePath, force);
      await loadWorktrees();
      toast.success(t("git.toast.worktreeRemoved"));
    } catch (e) {
      reportError(t("git.toast.worktreeRemoveFailed"), e);
    }
  }

  /** Thêm một đường dẫn (vd. worktree vừa tạo) vào danh sách repo và mở nó. */
  async function openPathAsRepo(targetPath: string) {
    if (!targetPath.trim()) return;
    try {
      const repo = await gitAddRepo(targetPath);
      if (!repos.value.some((r) => r.id === repo.id)) {
        repos.value = [repo, ...repos.value];
      }
      await openRepo(repo);
    } catch (e) {
      reportError(t("git.toast.openDirFailed"), e);
    }
  }

  // === Tag ===

  async function loadTags() {
    const path = repoPath();
    if (!path) return;
    try {
      tags.value = await gitTagList(path);
    } catch (e) {
      reportError(t("git.toast.tagsLoadFailed"), e);
    }
  }

  async function createTag(
    name: string,
    hash: string,
    message: string,
    annotated: boolean,
    push: boolean,
  ): Promise<boolean> {
    const path = repoPath();
    if (!path || !name.trim()) return false;
    busyMessage.value = t("git.toast.tagCreating");
    try {
      await gitTagCreate(path, name.trim(), hash, message, annotated, push);
      await loadTags();
      toast.success(
        push
          ? t("git.toast.tagCreatedAndPushed", { name: name.trim() })
          : t("git.toast.tagCreated", { name: name.trim() }),
      );
      return true;
    } catch (e) {
      reportError(t("git.toast.tagCreateFailed"), e);
      return false;
    } finally {
      busyMessage.value = "";
    }
  }

  async function deleteTag(name: string, remote: boolean) {
    const path = repoPath();
    if (!path) return;
    try {
      await gitTagDelete(path, name, remote);
      await loadTags();
      toast.success(t("git.toast.tagDeleted", { name }));
    } catch (e) {
      reportError(t("git.toast.tagDeleteFailed"), e);
    }
  }

  // === Merge ===

  async function mergeBranch(branch: string, squash: boolean, message: string): Promise<boolean> {
    const path = repoPath();
    if (!path || !branch.trim()) return false;
    busyMessage.value = squash ? t("git.toast.squashMerging") : t("git.toast.merging");
    try {
      await gitMerge(path, branch, squash, message);
      await Promise.all([refreshStatusAndInfo(), refreshBranches()]);
      if (tab.value === "history") await loadHistory();
      toast.success(
        squash ? t("git.toast.squashMerged", { branch }) : t("git.toast.merged", { branch }),
      );
      return true;
    } catch (e) {
      reportError(t("git.toast.mergeFailed"), e);
      await refreshStatusAndInfo();
      return false;
    } finally {
      busyMessage.value = "";
    }
  }

  async function mergeAbort() {
    const path = repoPath();
    if (!path) return;
    busyMessage.value = t("git.toast.mergeAborting");
    try {
      await gitMergeAbort(path);
      await Promise.all([refreshStatusAndInfo(), refreshBranches()]);
      toast.success(t("git.toast.mergeAborted"));
    } catch (e) {
      reportError(t("git.toast.mergeAbortFailed"), e);
    } finally {
      busyMessage.value = "";
    }
  }

  // === Resolve conflict ===

  async function loadConflicts() {
    const path = repoPath();
    if (!path) return;
    try {
      conflicts.value = await gitListConflicts(path);
    } catch (e) {
      reportError(t("git.toast.conflictsLoadFailed"), e);
    }
  }

  async function resolveConflict(file: string, side: "ours" | "theirs") {
    const path = repoPath();
    if (!path) return;
    try {
      await gitResolveConflict(path, file, side);
      await Promise.all([loadConflicts(), refreshStatusAndInfo()]);
    } catch (e) {
      reportError(t("git.toast.conflictResolveFailed"), e);
    }
  }

  /** Đánh dấu file đã tự xử lý (stage nó). */
  async function markResolved(file: string) {
    const path = repoPath();
    if (!path) return;
    try {
      await gitStage(path, [file]);
      await Promise.all([loadConflicts(), refreshStatusAndInfo()]);
    } catch (e) {
      reportError(t("git.toast.stageFailed"), e);
    }
  }

  /** Hoàn tất sau khi hết xung đột: tùy trạng thái mà continue/commit. */
  async function finishConflict() {
    const path = repoPath();
    if (!path) return;
    if (info.value?.rebase_in_progress) return rebaseContinue();
    if (info.value?.cherry_pick_in_progress) return cherryPickContinue();
    // merge (kể cả pull dạng merge)
    busyMessage.value = t("git.toast.finishingMerge");
    try {
      await gitCommitNoEdit(path);
      await Promise.all([refreshStatusAndInfo(), refreshBranches()]);
      if (tab.value === "history") await loadHistory();
      toast.success(t("git.toast.mergeFinished"));
    } catch (e) {
      reportError(t("git.toast.mergeFinishFailed"), e);
      await refreshStatusAndInfo();
    } finally {
      busyMessage.value = "";
    }
  }

  // === Cleanup branch đã merge ===

  async function cleanupScan(): Promise<string[]> {
    const path = repoPath();
    if (!path) return [];
    busyMessage.value = t("git.toast.cleanupScanning");
    try {
      return await gitCleanupScan(path);
    } catch (e) {
      reportError(t("git.toast.cleanupScanFailed"), e);
      return [];
    } finally {
      busyMessage.value = "";
    }
  }

  async function cleanupDelete(list: string[]) {
    const path = repoPath();
    if (!path || !list.length) return;
    try {
      const deleted = await gitCleanupDelete(path, list);
      await refreshBranches();
      toast.success(t("git.toast.cleanupDone", { count: deleted.length }));
    } catch (e) {
      reportError(t("git.toast.cleanupFailed"), e);
    }
  }

  // === Compare / Pull Request ===

  async function compareBranches(base: string, head: string) {
    const path = repoPath();
    if (!path || !base.trim() || !head.trim()) return;
    comparisonDiff.value = null;
    busyMessage.value = t("git.toast.comparing");
    try {
      comparison.value = await gitCompare(path, base, head);
    } catch (e) {
      reportError(t("git.toast.compareFailed"), e);
    } finally {
      busyMessage.value = "";
    }
  }

  async function compareSelectFile(file: GitFileChange) {
    const path = repoPath();
    const cmp = comparison.value;
    if (!path || !cmp) return;
    try {
      comparisonDiff.value = await gitCompareFileDiff(path, cmp.base, cmp.head, file.path);
    } catch (e) {
      reportError(t("git.toast.diffReadFailed"), e);
    }
  }

  async function createPullRequest(base: string, head: string) {
    const path = repoPath();
    if (!path) return;
    try {
      const url = await gitCreatePullRequest(path, base, head);
      toast.success(t("git.toast.prCreated", { url }));
    } catch (e) {
      reportError(t("git.toast.prCreateFailed"), e);
    }
  }

  /** Lấy danh sách Pull Request từ host (GitHub/GitLab), tận dụng credential đã lưu. */
  async function loadPullRequests(state: string) {
    const path = repoPath();
    if (!path) return;
    pullRequestsLoading.value = true;
    try {
      pullRequests.value = await gitListPullRequests(path, state);
    } catch (e) {
      pullRequests.value = [];
      reportError(t("git.toast.prListFailed"), e);
    } finally {
      pullRequestsLoading.value = false;
    }
  }

  /** Mở một URL bằng trình duyệt mặc định. */
  async function openUrl(url: string) {
    try {
      await gitOpenUrl(url);
    } catch (e) {
      reportError(t("git.toast.openLinkFailed"), e);
    }
  }

  /** Mở terminal tại thư mục repo hiện tại. */
  async function openTerminal() {
    const path = repoPath();
    if (!path) return;
    try {
      await gitOpenTerminal(path);
    } catch (e) {
      reportError(t("git.toast.openTerminalFailed"), e);
    }
  }

  /** Mở repo hiện tại bằng VS Code. */
  async function openVscode() {
    const path = repoPath();
    if (!path) return;
    try {
      await gitOpenVscode(path);
    } catch (e) {
      reportError(t("git.toast.openVscodeFailed"), e);
    }
  }

  /** Hiện một file/thư mục trong file explorer của hệ điều hành. */
  async function showInFolder(absolutePath: string) {
    try {
      await explorerOpen(absolutePath);
    } catch (e) {
      reportError(t("git.toast.openDirFailed"), e);
    }
  }

  // === Visualization (đồ thị commit) ===

  async function loadGraph(limit = 300) {
    const path = repoPath();
    if (!path) return;
    graphLoading.value = true;
    try {
      graphCommits.value = await gitGraph(path, limit);
    } catch (e) {
      reportError(t("git.toast.graphLoadFailed"), e);
    } finally {
      graphLoading.value = false;
    }
  }

  // === Commit browser ===

  async function loadBrowserCommits() {
    const path = repoPath();
    if (!path) return;
    browserLoading.value = true;
    browserFiles.value = [];
    browserDiff.value = null;
    try {
      browserCommits.value = await gitLog(path, 200);
    } catch (e) {
      reportError(t("git.toast.browserCommitsLoadFailed"), e);
    } finally {
      browserLoading.value = false;
    }
  }

  async function focusBrowserCommit(hash: string) {
    const path = repoPath();
    if (!path) return;
    browserDiff.value = null;
    try {
      const detail = await gitCommitDetail(path, hash);
      browserFiles.value = detail.files;
    } catch (e) {
      reportError(t("git.toast.commitReadFailed"), e);
    }
  }

  async function selectBrowserFile(hash: string, file: string) {
    const path = repoPath();
    if (!path) return;
    try {
      browserDiff.value = await gitCommitFileDiff(path, hash, file);
    } catch (e) {
      reportError(t("git.toast.diffReadFailed"), e);
    }
  }

  /** Lấy danh sách đường dẫn file thay đổi trong một commit (tương đương `git show --name-only`). */
  async function commitChangedFiles(hash: string): Promise<string[]> {
    const path = repoPath();
    if (!path) return [];
    try {
      const detail = await gitCommitDetail(path, hash);
      return detail.files.map((f) => f.path);
    } catch (e) {
      reportError(t("git.toast.commitReadFailed"), e);
      return [];
    }
  }

  // === Blame ===

  /** Tải `git blame` cho một file (rev rỗng = HEAD + working tree). */
  async function loadBlame(file: string, rev = "") {
    const path = repoPath();
    if (!path) return;
    blameFile.value = file;
    blameLines.value = [];
    blameSelectedHash.value = "";
    blameDetail.value = null;
    blameLoading.value = true;
    try {
      const result = await gitBlame(path, file, rev);
      blameLines.value = result.lines;
    } catch (e) {
      reportError(t("git.toast.blameReadFailed"), e);
    } finally {
      blameLoading.value = false;
    }
  }

  /** Chọn một dòng blame để xem chi tiết commit tương ứng. */
  async function selectBlameLine(hash: string) {
    const path = repoPath();
    if (!path || !hash) return;
    blameSelectedHash.value = hash;
    try {
      blameDetail.value = await gitCommitDetail(path, hash);
    } catch (e) {
      reportError(t("git.toast.commitReadFailed"), e);
    }
  }

  // === Clone ===

  async function cloneRepo(url: string, destParent: string): Promise<boolean> {
    if (!runtimeAvailable.value || !url.trim() || !destParent.trim()) return false;

    const name = repoNameFromUrl(url);
    const sep = destParent.includes("\\") ? "\\" : "/";
    const dest = `${destParent.replace(/[/\\]+$/, "")}${sep}${name}`;

    syncing.value = true;
    busyMessage.value = t("git.toast.cloning", { name });
    syncProgress.value = null;
    try {
      await gitClone(url.trim(), dest, (p) => {
        syncProgress.value = p;
      });
      const repo = await gitAddRepo(dest);
      if (!repos.value.some((r) => r.id === repo.id)) {
        repos.value = [repo, ...repos.value];
      }
      await openRepo(repo);
      toast.success(t("git.toast.cloned", { name }));
      return true;
    } catch (e) {
      reportError(t("git.toast.cloneFailed"), e);
      return false;
    } finally {
      syncing.value = false;
      busyMessage.value = "";
      syncProgress.value = null;
    }
  }

  function repoNameFromUrl(url: string): string {
    const trimmed = url.trim().replace(/\/+$/, "");
    const last = trimmed.split(/[/:]/).pop() ?? "repo";
    return last.replace(/\.git$/i, "") || "repo";
  }

  return {
    // state
    repos,
    activeRepo,
    info,
    staged,
    unstaged,
    branches,
    localBranches,
    remoteBranches,
    stashes,
    worktrees,
    tags,
    commits,
    historyHasMore,
    historyLoadingMore,
    comparison,
    comparisonDiff,
    pullRequests,
    pullRequestsLoading,
    conflicts,
    browserCommits,
    browserFiles,
    browserDiff,
    browserLoading,
    blameFile,
    blameLines,
    blameLoading,
    blameSelectedHash,
    blameDetail,
    graphCommits,
    graphLoading,
    selectedFile,
    diff,
    diffLoading,
    selectedCommit,
    commitDetail,
    commitFileDiff,
    commitMessage,
    tab,
    loadingRepo,
    refreshing,
    committing,
    syncing,
    busyMessage,
    syncProgress,
    // computed
    runtimeAvailable,
    hasChanges,
    canCommit,
    // actions
    loadRepos,
    addRepoFromDialog,
    removeRepo,
    openRepo,
    refreshStatusAndInfo,
    refreshBranches,
    refreshStashes,
    selectFile,
    stageFiles,
    unstageFiles,
    stageAll,
    unstageAll,
    discardFiles,
    commit,
    fetch,
    pull,
    push,
    loadHistory,
    loadMoreHistory,
    selectCommit,
    selectCommitFile,
    switchTab,
    checkoutBranch,
    createBranch,
    deleteBranch,
    stashSave,
    stashApply,
    stashDrop,
    cloneRepo,
    undoLastCommit,
    amendCommit,
    resetTo,
    checkoutCommit,
    createBranchAt,
    copyText,
    revert,
    revertAbort,
    rebaseOnto,
    rebaseAbort,
    rebaseContinue,
    cherryPick,
    cherryPickAbort,
    cherryPickContinue,
    loadTags,
    createTag,
    deleteTag,
    mergeBranch,
    mergeAbort,
    compareBranches,
    compareSelectFile,
    createPullRequest,
    loadPullRequests,
    openUrl,
    openTerminal,
    openVscode,
    showInFolder,
    loadBrowserCommits,
    focusBrowserCommit,
    selectBrowserFile,
    commitChangedFiles,
    loadBlame,
    selectBlameLine,
    loadGraph,
    loadConflicts,
    resolveConflict,
    markResolved,
    finishConflict,
    cleanupScan,
    cleanupDelete,
    loadWorktrees,
    worktreeAdd,
    worktreeRemove,
    openPathAsRepo,
  };
}

export type GitApi = ReturnType<typeof useGit>;
