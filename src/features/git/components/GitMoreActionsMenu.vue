<script setup lang="ts">
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import type { GitApi } from "../composables/useGit";

const props = defineProps<{
  git: GitApi;
  isOnBaseBranch: boolean;
}>();

const open = defineModel<boolean>("open", { default: false });

const emit = defineEmits<{
  action: [name: string];
}>();

const { t } = useI18n();

const triggerRef = ref<HTMLButtonElement | null>(null);
const menuStyle = ref({ bottom: "0px", right: "0px" });

function toggle() {
  if (!open.value) {
    const rect = triggerRef.value?.getBoundingClientRect();
    if (rect) {
      menuStyle.value = {
        bottom: `${window.innerHeight - rect.top + 4}px`,
        right: `${window.innerWidth - rect.right}px`,
      };
    }
  }
  open.value = !open.value;
}

function act(name: string) {
  open.value = false;
  emit("action", name);
}

function onClickAway(e: MouseEvent) {
  if (!triggerRef.value?.contains(e.target as Node)) {
    open.value = false;
  }
}

watch(open, (val) => {
  if (val) {
    requestAnimationFrame(() => document.addEventListener("click", onClickAway));
  } else {
    document.removeEventListener("click", onClickAway);
  }
});
</script>

<template>
  <div>
    <button
      ref="triggerRef"
      class="flex h-7 items-center rounded-md px-2 text-secondary transition-colors hover:bg-canvas hover:text-brand"
      :title="t('git.page.moreActions')"
      @click="toggle"
    >
      <i class="pi pi-ellipsis-h text-[11px]" />
    </button>

    <Teleport to="body">
      <div
        v-if="open"
        class="fixed z-50 w-60 rounded-lg border border-divider bg-panel p-1.5 shadow-float"
        :style="menuStyle"
      >
        <button class="ctx-menu-item" @click="act('rebase')">
          <i class="pi pi-arrows-v text-xs" /> {{ t("git.page.menu.rebase") }}
        </button>
        <button class="ctx-menu-item" @click="act('create-worktree')">
          <i class="pi pi-clone text-xs" /> {{ t("git.page.menu.createWorktree") }}
        </button>
        <button class="ctx-menu-item" @click="act('manage-worktrees')">
          <i class="pi pi-list text-xs" /> {{ t("git.page.menu.manageWorktree") }}
        </button>
        <div class="my-1 border-t border-divider" />
        <button class="ctx-menu-item" @click="act('manage-stash')">
          <i class="pi pi-inbox text-xs" /> {{ t("git.page.menu.manageStash") }}
          <span v-if="git.stashes.value.length" class="ml-auto rounded-full bg-canvas px-1.5 text-[10px] font-semibold text-muted">
            {{ git.stashes.value.length }}
          </span>
        </button>
        <div class="my-1 border-t border-divider" />
        <button class="ctx-menu-item" @click="act('create-tag')">
          <i class="pi pi-tag text-xs" /> {{ t("git.page.menu.createTag") }}
        </button>
        <button class="ctx-menu-item" @click="act('manage-tags')">
          <i class="pi pi-tags text-xs" /> {{ t("git.page.menu.manageTags") }}
        </button>
        <button class="ctx-menu-item" @click="act('merge')">
          <i class="pi pi-code-branch text-xs" /> {{ t("git.page.menu.mergeBranch") }}
        </button>
        <button class="ctx-menu-item" @click="act('compare')">
          <i class="pi pi-arrows-h text-xs" /> {{ t("git.page.menu.compare") }}
        </button>
        <button class="ctx-menu-item" @click="act('view-prs')">
          <i class="pi pi-flag text-xs" /> {{ t("git.page.menu.viewPrs") }}
        </button>
        <div class="my-1 border-t border-divider" />
        <button v-if="!isOnBaseBranch" class="ctx-menu-item" @click="act('update-from-main')">
          <i class="pi pi-arrow-circle-down text-xs" /> {{ t("git.page.menu.updateFromMain") }}
        </button>
        <button class="ctx-menu-item" @click="act('reset-head')">
          <i class="pi pi-backward text-xs" /> {{ t("git.page.menu.resetHead") }}
        </button>
        <button class="ctx-menu-item" @click="act('cleanup')">
          <i class="pi pi-eraser text-xs" /> {{ t("git.page.menu.cleanup") }}
        </button>
        <button class="ctx-menu-item" @click="act('browse-commits')">
          <i class="pi pi-copy text-xs" /> {{ t("git.page.menu.browseCommits") }}
        </button>
        <button class="ctx-menu-item" @click="act('show-log')">
          <i class="pi pi-list-check text-xs" /> {{ t("git.page.menu.showLog") }}
        </button>
        <button class="ctx-menu-item" @click="act('show-graph')">
          <i class="pi pi-sitemap text-xs" /> {{ t("git.page.menu.showGraph") }}
        </button>
      </div>
    </Teleport>
  </div>
</template>
