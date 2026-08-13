<script setup lang="ts">
import { onMounted } from "vue";
import { useI18n } from "vue-i18n";
import { useAiUsage } from "@/features/ai-agent/composables/useAiUsage";
import { AI_PROVIDER_LABEL } from "@/models/ai-usage";

const { t } = useI18n();
const ctrl = useAiUsage();

onMounted(() => void ctrl.start());
</script>

<template>
  <div class="flex h-full flex-col overflow-hidden bg-sidebar text-sidebar-text">
    <div class="flex shrink-0 items-center border-b border-sidebar-border px-3 py-2">
      <span class="flex-1 truncate text-[11px] font-semibold uppercase tracking-wide">{{ t("workspaces.sidebar.agents") }}</span>
    </div>

    <div class="min-h-0 flex-1 overflow-y-auto p-2">
      <div v-if="ctrl.isLoading.value" class="flex items-center gap-2 px-1 py-2 text-xs text-sidebar-text">
        <i class="pi pi-spinner pi-spin" /> {{ t("common.loading") }}
      </div>
      <p v-else-if="!ctrl.accounts.value.length" class="px-1 py-2 text-xs text-sidebar-text">
        {{ t("workspaces.sidebar.agentsEmpty") }}
      </p>
      <button
        v-for="account in ctrl.accounts.value"
        :key="account.id"
        type="button"
        class="mb-1 flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-left text-xs transition-colors hover:bg-sidebar-hover"
        :class="account.is_active ? 'bg-sidebar-active text-sidebar-text-active' : 'text-sidebar-text'"
        :disabled="account.is_active"
        :title="AI_PROVIDER_LABEL[account.provider]"
        @click="ctrl.setActive(account.id)"
      >
        <i class="pi pi-sparkles shrink-0" />
        <span class="min-w-0 flex-1 truncate">{{ account.name }}</span>
        <i v-if="account.is_active" class="pi pi-check-circle shrink-0 text-brand" />
      </button>
    </div>
  </div>
</template>
