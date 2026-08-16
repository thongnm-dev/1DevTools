<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import InputText from "primevue/inputtext";
import Popover from "primevue/popover";
import Textarea from "primevue/textarea";
import { useGit } from "@/features/git/composables/useGit";
import { baseName, statusMeta } from "@/features/git/utils/fileStatus";
import { guessBase } from "@/features/git/utils/gitRefs";
import type { GitBranch, GitRepo } from "@/models/git";
import GitMoreActionsMenu from "@/features/git/components/GitMoreActionsMenu.vue";
import GitRebaseDialog from "@/features/git/components/GitRebaseDialog.vue";
import GitWorktreeCreateDialog from "@/features/git/components/GitWorktreeCreateDialog.vue";
import GitWorktreeListDialog from "@/features/git/components/GitWorktreeListDialog.vue";
import GitStashListDialog from "@/features/git/components/GitStashListDialog.vue";
import GitTagDialog from "@/features/git/components/GitTagDialog.vue";
import GitTagListDialog from "@/features/git/components/GitTagListDialog.vue";
import GitMergeDialog from "@/features/git/components/GitMergeDialog.vue";
import GitCompareDialog from "@/features/git/components/GitCompareDialog.vue";
import GitPullRequestsDialog from "@/features/git/components/GitPullRequestsDialog.vue";
import GitUpdateFromMainDialog from "@/features/git/components/GitUpdateFromMainDialog.vue";
import GitResetHeadDialog from "@/features/git/components/GitResetHeadDialog.vue";
import GitCleanupDialog from "@/features/git/components/GitCleanupDialog.vue";
import GitCommitBrowserDialog from "@/features/git/components/GitCommitBrowserDialog.vue";
import GitLogDialog from "@/features/git/components/GitLogDialog.vue";
import GitGraphDialog from "@/features/git/components/GitGraphDialog.vue";

const props = defineProps<{ repo: GitRepo }>();
const { t } = useI18n();
const git = useGit();

// `persist: false` — panel này chỉ hiển thị 1 repo cố định theo workspace,
// không ghi vào ACTIVE_REPO_KEY (key đó dành cho màn Git Desktop độc lập).
watch(
  () => props.repo.path,
  () => void git.openRepo(props.repo, { persist: false }),
  { immediate: true },
);

const branchPopover = ref<InstanceType<typeof Popover> | null>(null);
const branchFilter = ref("");

const filteredBranches = computed(() => {
  const q = branchFilter.value.trim().toLowerCase();
  const list = git.branches.value;
  if (!q) return list;
  return list.filter((b) => b.name.toLowerCase().includes(q));
});

function toggleBranchMenu(e: Event) {
  branchFilter.value = "";
  branchPopover.value?.toggle(e);
}

async function selectBranch(b: GitBranch) {
  branchPopover.value?.hide();
  if (b.is_current) return;
  const name = b.is_remote ? b.name.replace(/^[^/]+\//, "") : b.name;
  await git.checkoutBranch(name);
}

const isOnBaseBranch = computed(() => {
  const cur = git.info.value?.current_branch;
  if (!cur) return false;
  const base = guessBase(
    git.branches.value.map((b) => b.name),
    "",
    git.info.value?.upstream,
  );
  return cur === base || cur === base.replace(/^origin\//, "");
});

// === More actions dialogs ===
const moreMenuOpen = ref(false);
const rebaseDialogVisible = ref(false);
const worktreeCreateDialogVisible = ref(false);
const worktreeListDialogVisible = ref(false);
const stashListDialogVisible = ref(false);
const tagDialogVisible = ref(false);
const tagTarget = ref<{ hash: string; label: string }>({ hash: "", label: "HEAD" });
const tagListDialogVisible = ref(false);
const mergeDialogVisible = ref(false);
const compareDialogVisible = ref(false);
const prDialogVisible = ref(false);
const updateDialogVisible = ref(false);
const resetHeadDialogVisible = ref(false);
const cleanupDialogVisible = ref(false);
const browserDialogVisible = ref(false);
const logDialogVisible = ref(false);
const graphDialogVisible = ref(false);

function handleMoreAction(name: string) {
  switch (name) {
    case "rebase": rebaseDialogVisible.value = true; break;
    case "create-worktree": worktreeCreateDialogVisible.value = true; break;
    case "manage-worktrees": worktreeListDialogVisible.value = true; break;
    case "manage-stash": stashListDialogVisible.value = true; break;
    case "create-tag": tagTarget.value = { hash: "", label: "HEAD" }; tagDialogVisible.value = true; break;
    case "manage-tags": tagListDialogVisible.value = true; break;
    case "merge": mergeDialogVisible.value = true; break;
    case "compare": compareDialogVisible.value = true; break;
    case "view-prs": prDialogVisible.value = true; break;
    case "update-from-main": updateDialogVisible.value = true; break;
    case "reset-head": resetHeadDialogVisible.value = true; break;
    case "cleanup": cleanupDialogVisible.value = true; break;
    case "browse-commits": browserDialogVisible.value = true; break;
    case "show-log": logDialogVisible.value = true; break;
    case "show-graph": graphDialogVisible.value = true; break;
  }
}

function handleCommitKeydown(e: KeyboardEvent) {
  if (e.key === "Enter" && (e.ctrlKey || e.metaKey)) {
    e.preventDefault();
    if (git.canCommit.value) void git.commit();
  }
}

function lineClass(kind: string): string {
  switch (kind) {
    case "add":
      return "bg-emerald-500/10 text-emerald-700";
    case "del":
      return "bg-red-500/10 text-red-700";
    case "hunk":
      return "text-brand font-bold";
    case "meta":
      return "text-muted";
    default:
      return "text-secondary";
  }
}
</script>

<template>
  <div class="flex h-full min-h-0 flex-col gap-3 overflow-hidden p-3">
    <div class="flex min-h-0 flex-1 gap-3 overflow-hidden">
      <!-- File list -->
      <div class="flex w-60 shrink-0 flex-col overflow-hidden rounded-lg border border-divider bg-canvas">
        <div class="flex-1 overflow-auto">
          <div v-if="git.staged.value.length === 0 && git.unstaged.value.length === 0" class="px-3 py-6 text-center text-xs text-muted">
            {{ t("workspaces.git.noChanges") }}
          </div>
          <template v-else>
            <template v-if="git.staged.value.length">
              <div class="flex items-center justify-between border-b border-divider px-3 py-1.5">
                <span class="text-[11px] font-bold uppercase text-muted">{{ t("workspaces.git.staged") }} ({{ git.staged.value.length }})</span>
                <button class="text-[10px] font-bold text-brand hover:underline" @click="git.unstageAll()">{{ t("workspaces.git.unstageAll") }}</button>
              </div>
              <button
                v-for="f in git.staged.value"
                :key="'s-' + f.path"
                class="group flex w-full items-center gap-1.5 px-3 py-1.5 text-left transition-colors hover:bg-panel"
                :class="{ 'bg-brand/10': git.selectedFile.value?.path === f.path && git.selectedFile.value?.staged }"
                @click="git.selectFile(f, true)"
              >
                <span :class="['w-4 shrink-0 text-center text-[10px] font-bold', statusMeta(f.status).cls]">{{ f.status }}</span>
                <span class="min-w-0 flex-1 truncate text-xs">{{ baseName(f.path) }}</span>
                <i
                  class="pi pi-minus shrink-0 text-[10px] text-muted opacity-0 hover:text-red-500 group-hover:opacity-100"
                  :title="t('workspaces.git.unstage')"
                  @click.stop="git.unstageFiles([f.path])"
                />
              </button>
            </template>

            <template v-if="git.unstaged.value.length">
              <div class="flex items-center justify-between border-b border-t border-divider px-3 py-1.5">
                <span class="text-[11px] font-bold uppercase text-muted">{{ t("workspaces.git.unstaged") }} ({{ git.unstaged.value.length }})</span>
                <button class="text-[10px] font-bold text-brand hover:underline" @click="git.stageAll()">{{ t("workspaces.git.stageAll") }}</button>
              </div>
              <button
                v-for="f in git.unstaged.value"
                :key="'u-' + f.path"
                class="group flex w-full items-center gap-1.5 px-3 py-1.5 text-left transition-colors hover:bg-panel"
                :class="{ 'bg-brand/10': git.selectedFile.value?.path === f.path && !git.selectedFile.value?.staged }"
                @click="git.selectFile(f, false)"
              >
                <span :class="['w-4 shrink-0 text-center text-[10px] font-bold', statusMeta(f.status).cls]">{{ f.status }}</span>
                <span class="min-w-0 flex-1 truncate text-xs">{{ baseName(f.path) }}</span>
                <i
                  class="pi pi-plus shrink-0 text-[10px] text-muted opacity-0 hover:text-emerald-600 group-hover:opacity-100"
                  :title="t('workspaces.git.stage')"
                  @click.stop="git.stageFiles([f.path])"
                />
              </button>
            </template>
          </template>
        </div>
      </div>

      <!-- Diff viewer -->
      <div class="min-h-0 flex-1 overflow-auto rounded-lg border border-divider bg-canvas">
        <div v-if="git.diffLoading.value" class="flex h-full items-center justify-center text-xs text-muted">{{ t("common.loading") }}</div>
        <div v-else-if="!git.diff.value" class="flex h-full items-center justify-center text-xs text-muted">{{ t("workspaces.git.selectFileHint") }}</div>
        <div v-else-if="git.diff.value.is_binary" class="flex h-full items-center justify-center text-xs text-muted">{{ t("workspaces.git.binaryFile") }}</div>
        <div v-else class="overflow-auto p-3 font-mono text-[11px] leading-5">
          <div v-for="(line, i) in git.diff.value.lines" :key="i" :class="['whitespace-pre', lineClass(line.kind)]">{{ line.content }}</div>
        </div>
      </div>
    </div>

    <!-- Commit box -->
    <div class="flex shrink-0 items-end gap-2 rounded-lg border border-divider bg-canvas p-2">
      <Textarea
        v-model="git.commitMessage.value"
        rows="2"
        class="!flex-1 !text-xs"
        :placeholder="t('workspaces.git.commitPlaceholder')"
        :disabled="git.committing.value"
        @keydown="handleCommitKeydown"
      />
    </div>

    <!-- Toolbar -->
    <div class="flex shrink-0 items-center gap-1 rounded-lg border border-divider bg-canvas px-2 py-1.5">
      <!-- Branch picker -->
      <button
        class="flex h-7 items-center gap-1.5 rounded-md px-2 text-xs transition-colors hover:bg-panel"
        :title="t('workspaces.git.switchBranch')"
        @click="toggleBranchMenu($event)"
      >
        <i class="pi pi-sitemap shrink-0 text-[11px] text-brand" />
        <span class="max-w-[140px] truncate font-medium text-ink">
          {{ git.info.value ? (git.info.value.detached ? t("workspaces.git.detachedAt", { name: git.info.value.current_branch }) : (git.info.value.current_branch || "—")) : "—" }}
        </span>
        <i class="pi pi-chevron-down shrink-0 text-[9px] text-muted" />
      </button>

      <Popover ref="branchPopover">
        <div class="w-[280px] p-1">
          <div class="relative mb-1">
            <i class="pi pi-search pointer-events-none absolute left-2.5 top-1/2 -translate-y-1/2 text-xs text-muted" />
            <InputText
              v-model="branchFilter"
              :placeholder="t('workspaces.git.filterBranchPlaceholder')"
              class="h-8 w-full !pl-8 !text-xs"
            />
          </div>
          <div class="max-h-64 overflow-y-auto">
            <button
              v-for="b in filteredBranches"
              :key="(b.is_remote ? 'r:' : 'l:') + b.name"
              class="group flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-left text-xs transition-colors hover:bg-canvas"
              @click="selectBranch(b)"
            >
              <i
                class="pi shrink-0 text-xs"
                :class="b.is_current ? 'pi-check text-brand' : b.is_remote ? 'pi-cloud text-muted' : 'pi-sitemap text-muted'"
              />
              <span class="min-w-0 flex-1 truncate" :class="b.is_current ? 'font-semibold text-brand' : 'text-ink'">{{ b.name }}</span>
              <i
                v-if="!b.is_remote && !b.is_current"
                class="pi pi-trash invisible shrink-0 rounded p-1 text-muted transition-colors hover:text-red-600 group-hover:visible"
                :title="t('workspaces.git.deleteBranch')"
                @click.stop="git.deleteBranch(b.name, false)"
              />
            </button>
            <div v-if="!filteredBranches.length" class="px-2.5 py-3 text-center text-xs text-muted">
              {{ t("workspaces.git.noBranchesMatch") }}
            </div>
          </div>
        </div>
      </Popover>

      <div class="mx-1 h-4 w-px bg-divider" />

      <button
        class="flex items-center gap-1.5 rounded-md px-2.5 py-1 text-xs text-ink transition-colors hover:bg-panel disabled:cursor-not-allowed disabled:opacity-40"
        :disabled="git.syncing.value"
        :title="t('workspaces.git.fetch')"
        @click="git.fetch()"
      >
        <i class="pi pi-cloud-download text-[11px]" />
        <span>{{ t('workspaces.git.fetch') }}</span>
      </button>
      <button
        class="flex items-center gap-1.5 rounded-md px-2.5 py-1 text-xs text-ink transition-colors hover:bg-panel disabled:cursor-not-allowed disabled:opacity-40"
        :disabled="git.syncing.value"
        :title="t('workspaces.git.pull')"
        @click="git.pull()"
      >
        <i class="pi pi-arrow-down text-[11px]" />
        <span>{{ t('workspaces.git.pull') }}</span>
      </button>
      <button
        class="flex items-center gap-1.5 rounded-md px-2.5 py-1 text-xs text-ink transition-colors hover:bg-panel disabled:cursor-not-allowed disabled:opacity-40"
        :disabled="git.syncing.value"
        :title="t('workspaces.git.push')"
        @click="git.push()"
      >
        <i class="pi pi-arrow-up text-[11px]" />
        <span>{{ t('workspaces.git.push') }}</span>
      </button>

      <div class="mx-1 h-4 w-px bg-divider" />

      <button
        class="flex items-center gap-1.5 rounded-md px-2.5 py-1 text-xs text-ink transition-colors hover:bg-panel disabled:cursor-not-allowed disabled:opacity-40"
        :disabled="git.syncing.value || !git.hasChanges.value"
        :title="t('workspaces.git.stash')"
        @click="git.stashSave('')"
      >
        <i class="pi pi-inbox text-[11px]" />
        <span>{{ t('workspaces.git.stash') }}</span>
      </button>
      <button
        class="flex items-center gap-1.5 rounded-md px-2.5 py-1 text-xs text-ink transition-colors hover:bg-panel disabled:cursor-not-allowed disabled:opacity-40"
        :disabled="git.syncing.value || !git.stashes.value.length"
        :title="t('workspaces.git.popStash')"
        @click="git.stashApply(git.stashes.value[0]?.reference ?? '', true)"
      >
        <i class="pi pi-arrow-circle-up text-[11px]" />
        <span>{{ t('workspaces.git.popStash') }}</span>
      </button>

      <div class="mx-1 h-4 w-px bg-divider" />

      <button
        class="flex items-center gap-1.5 rounded-md px-2.5 py-1 text-xs text-ink transition-colors hover:bg-panel disabled:cursor-not-allowed disabled:opacity-40"
        :disabled="git.committing.value"
        :title="t('workspaces.git.undoCommit')"
        @click="git.undoLastCommit()"
      >
        <i class="pi pi-undo text-[11px]" />
        <span>{{ t('workspaces.git.undoCommit') }}</span>
      </button>

      <div class="ml-auto flex items-center gap-1">
        <div v-if="git.syncing.value" class="flex items-center gap-1.5 text-[10px] text-muted">
          <i class="pi pi-spin pi-spinner text-[10px]" />
          <span>{{ git.busyMessage.value || '...' }}</span>
        </div>
        <GitMoreActionsMenu
          v-model:open="moreMenuOpen"
          :git="git"
          :is-on-base-branch="isOnBaseBranch"
          @action="handleMoreAction"
        />
      </div>
    </div>
  </div>

  <GitRebaseDialog v-model:visible="rebaseDialogVisible" :git="git" />
  <GitWorktreeCreateDialog v-model:visible="worktreeCreateDialogVisible" :git="git" />
  <GitWorktreeListDialog
    v-model:visible="worktreeListDialogVisible"
    :git="git"
    @create-worktree="worktreeCreateDialogVisible = true"
  />
  <GitStashListDialog v-model:visible="stashListDialogVisible" :git="git" />
  <GitTagDialog v-model:visible="tagDialogVisible" :git="git" :target="tagTarget" />
  <GitTagListDialog
    v-model:visible="tagListDialogVisible"
    :git="git"
    @create-tag="tagListDialogVisible = false; tagTarget = { hash: '', label: 'HEAD' }; tagDialogVisible = true"
  />
  <GitMergeDialog v-model:visible="mergeDialogVisible" :git="git" />
  <GitCompareDialog v-model:visible="compareDialogVisible" :git="git" :pr="null" :on-file-context="() => {}" />
  <GitPullRequestsDialog
    v-model:visible="prDialogVisible"
    :git="git"
    @open-compare="compareDialogVisible = true"
  />
  <GitUpdateFromMainDialog v-model:visible="updateDialogVisible" :git="git" />
  <GitResetHeadDialog v-model:visible="resetHeadDialogVisible" :git="git" />
  <GitCleanupDialog v-model:visible="cleanupDialogVisible" :git="git" />
  <GitCommitBrowserDialog v-model:visible="browserDialogVisible" :git="git" :on-file-context="() => {}" />
  <GitLogDialog v-model:visible="logDialogVisible" :git="git" />
  <GitGraphDialog v-model:visible="graphDialogVisible" :git="git" :on-file-context="() => {}" />
</template>
