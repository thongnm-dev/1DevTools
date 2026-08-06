<script setup lang="ts">
import { onMounted } from "vue";
import { useI18n } from "vue-i18n";
import { useAuthStore } from "@/app/stores/auth";
import { useAppUpdater } from "@/shared/composables/useAppUpdater";
import type { SystemInfo } from "@/models/system";

const props = defineProps<{
  info: SystemInfo;
}>();

const { t } = useI18n();
const auth = useAuthStore();

const updater = useAppUpdater();

onMounted(() => {
  updater.startPolling();
});

function onUpdateClick(): void {
  if (updater.status.value === "ready") {
    void updater.install();
  }
}

function onCheckUpdate(): void {
  void updater.checkNow();
}

function formatDateTime(value: string): string {
  const match = value.match(/^(\d{4})-(\d{2})-(\d{2}) (\d{2}:\d{2}:\d{2})$/);
  if (match) {
    return `${match[1]}/${match[2]}/${match[3]} ${match[4]}`;
  }

  const date = new Date(value);
  if (Number.isNaN(date.getTime())) {
    return value;
  }

  const pad = (part: number) => part.toString().padStart(2, "0");
  return `${date.getFullYear()}/${pad(date.getMonth() + 1)}/${pad(date.getDate())} ${pad(date.getHours())}:${pad(date.getMinutes())}:${pad(date.getSeconds())}`;
}
</script>

<template>
  <footer class="flex items-center gap-6 overflow-hidden border-t border-divider px-4 py-2 text-sm text-muted">
    <span class="status-item flex items-center gap-2" :title="t('bottomBar.loginTooltip')">
      <i class="pi pi-user shrink-0 text-brand" />
      <strong class="min-w-0 truncate text-ink">{{ auth.user?.full_name || auth.user?.username || '-' }}</strong>
    </span>
    <span class="status-item flex items-center gap-2" :title="t('bottomBar.dateTimeTooltip')">
      <i class="pi pi-clock shrink-0 text-brand" />
      <strong class="min-w-0 truncate text-ink">{{ formatDateTime(props.info.timestamp) }}</strong>
    </span>
    <span class="status-item flex items-center gap-2" :title="t('bottomBar.ipTooltip')">
      <i class="pi pi-globe shrink-0 text-brand" />
      <strong class="min-w-0 truncate text-ink">{{ props.info.ip_address }}</strong>
    </span>

    <template v-if="updater.isTauri">
      <span v-if="updater.status.value === 'checking'" class="status-item ml-auto flex items-center gap-2"
        :title="t('bottomBar.checkingUpdate')">
        <i class="pi pi-spin pi-spinner shrink-0 text-brand" />
        <span class="min-w-0 truncate">{{ t('bottomBar.checkingUpdate') }}</span>
      </span>

      <span v-else-if="updater.status.value === 'downloading'" class="status-item ml-auto flex items-center gap-2"
        :title="t('bottomBar.downloadingUpdateTitle', { version: updater.version.value ?? '' })">
        <i class="pi pi-spin pi-spinner shrink-0 text-brand" />
        <span class="min-w-0 truncate">
          {{ t('bottomBar.downloadingUpdate') }}
          <template v-if="updater.downloadPercent.value !== null">
            {{ updater.downloadPercent.value }}%
          </template>
          <template v-else>…</template>
        </span>
      </span>

      <button v-else-if="updater.status.value === 'ready'" type="button"
        class="status-item update-ready ml-auto flex items-center gap-2 rounded font-medium text-brand hover:underline"
        :title="t('bottomBar.updateReadyTooltip')" @click="onUpdateClick">
        <i class="pi pi-download shrink-0" />
        <span class="min-w-0 truncate">{{ t('bottomBar.updateReady') }}</span>
      </button>

      <span v-else-if="updater.status.value === 'installing'" class="status-item ml-auto flex items-center gap-2"
        :title="t('bottomBar.installingTooltip')">
        <i class="pi pi-spin pi-spinner shrink-0 text-brand" />
        <span class="min-w-0 truncate">{{ t('bottomBar.installing') }}</span>
      </span>

      <button v-else-if="updater.status.value === 'error'" type="button"
        class="status-item ml-auto flex items-center gap-2 rounded text-red-600 hover:underline"
        :title="updater.errorMessage.value ?? t('bottomBar.updateFailedTooltip')" @click="onCheckUpdate">
        <i class="pi pi-exclamation-triangle shrink-0" />
        <span class="min-w-0 truncate">{{ t('bottomBar.updateFailed') }}</span>
      </button>

      <button v-else type="button"
        class="status-item ml-auto flex items-center gap-2 rounded text-muted hover:text-brand"
        :title="t('bottomBar.checkUpdate')" @click="onCheckUpdate">
        <i class="pi pi-sync shrink-0" />
        <span class="min-w-0 truncate">{{ t('bottomBar.checkUpdate') }}</span>
      </button>
    </template>

    <span class="status-item flex items-center gap-2" :class="updater.isTauri ? '' : 'ml-auto'" :title="t('bottomBar.versionTooltip')">
      <i class="pi pi-desktop shrink-0 text-brand" />
      <strong class="min-w-0 truncate text-ink">{{ props.info.version }}</strong>
    </span>
  </footer>
</template>
