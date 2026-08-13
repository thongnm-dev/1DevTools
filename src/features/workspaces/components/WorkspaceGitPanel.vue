<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import Button from "primevue/button";
import InputText from "primevue/inputtext";
import Popover from "primevue/popover";
import Textarea from "primevue/textarea";
import { useGit } from "@/features/git/composables/useGit";
import { baseName, statusMeta } from "@/features/git/utils/fileStatus";
import type { GitBranch, GitRepo } from "@/models/git";

const props = defineProps<{ repo: GitRepo; workspaceId: number }>();
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
  <!-- Nút branch được teleport ra header của WorkspacesPage.vue (ngay sau tên
       workspace) — vẫn dùng chung `git` instance của panel này, chỉ đổi vị trí
       hiển thị. Chỉ hiện khi repo đã load được info (tức folder có .git hợp lệ). -->
  <Teleport v-if="git.info.value" :to="`#ws-branch-slot-${workspaceId}`">
    <button
      class="flex h-7 items-center gap-1.5 rounded-md px-2 text-xs transition-colors hover:bg-canvas"
      :title="t('workspaces.git.switchBranch')"
      @click="toggleBranchMenu($event)"
    >
      <i class="pi pi-sitemap shrink-0 text-[11px] text-brand" />
      <span class="max-w-[160px] truncate font-medium text-ink">
        {{ git.info.value.detached ? t("workspaces.git.detachedAt", { name: git.info.value.current_branch }) : (git.info.value.current_branch || "—") }}
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
  </Teleport>

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
      <Textarea v-model="git.commitMessage.value" rows="2" class="!flex-1 !text-xs" :placeholder="t('workspaces.git.commitPlaceholder')" />
      <Button
        icon="pi pi-check"
        :label="t('workspaces.git.commit')"
        size="small"
        :disabled="!git.canCommit.value"
        :loading="git.committing.value"
        @click="git.commit()"
      />
    </div>
  </div>
</template>
