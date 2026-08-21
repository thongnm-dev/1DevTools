<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useI18n } from "vue-i18n";
import { useAuthStore } from "@/app/stores/auth";
import { useAppUpdater } from "@/shared/composables/useAppUpdater";
import { useLocale } from "@/shared/composables/useLocale";
import { applyTheme, type ThemeMode } from "@/shared/config/themeTokens";
import Popover from "primevue/popover";
import type { SystemInfo } from "@/models/system";

const props = defineProps<{
  info: SystemInfo;
  isSidebarCollapsed: boolean;
}>();

const emit = defineEmits<{
  logout: [];
  profile: [];
  toggleCollapse: [];
}>();

const { t } = useI18n();
const auth = useAuthStore();
const updater = useAppUpdater();
const { locale, setLocale } = useLocale();

const SETTINGS_KEY = "msh.app.settings";
const popoverRef = ref();

function loadTheme(): ThemeMode {
  try {
    const saved = window.localStorage.getItem(SETTINGS_KEY);
    if (!saved) return "light";
    const parsed = JSON.parse(saved) as { theme?: string };
    return parsed.theme === "dark" ? "dark" : "light";
  } catch {
    return "light";
  }
}

const theme = ref<ThemeMode>(loadTheme());

function toggleLocale() {
  setLocale(locale.value === "vi" ? "en" : "vi");
}

function toggleTheme() {
  theme.value = theme.value === "dark" ? "light" : "dark";
  window.localStorage.setItem(SETTINGS_KEY, JSON.stringify({ theme: theme.value }));
  applyTheme(theme.value);
}

function togglePopover(event: Event) {
  popoverRef.value?.toggle(event);
}

function openProfile() {
  popoverRef.value?.hide();
  emit("profile");
}

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
    <button
      type="button"
      class="status-item flex items-center gap-2 rounded transition-colors hover:text-brand"
      :title="isSidebarCollapsed ? t('shell.expandSidebar') : t('shell.collapseSidebar')"
      @click="emit('toggleCollapse')"
    >
      <i :class="['pi shrink-0 text-brand', isSidebarCollapsed ? 'pi-chevron-right' : 'pi-chevron-left']" />
    </button>
    <div class="mx-0.5 h-4 w-px bg-divider" />
    <button
      type="button"
      class="status-item flex items-center gap-2 rounded transition-colors hover:text-brand"
      :title="t('bottomBar.loginTooltip')"
      @click="togglePopover"
    >
      <i class="pi pi-user shrink-0 text-brand" />
      <strong class="min-w-0 truncate text-ink">{{ auth.user?.full_name || auth.user?.username || '-' }}</strong>
    </button>
    <span v-if="auth.user?.email" class="status-item flex items-center gap-2" :title="t('bottomBar.emailTooltip')">
      <i class="pi pi-envelope shrink-0 text-brand" />
      <strong class="min-w-0 truncate text-ink">{{ auth.user.email }}</strong>
    </span>
    <span v-if="auth.user?.phone" class="status-item flex items-center gap-2" :title="t('bottomBar.phoneTooltip')">
      <i class="pi pi-phone shrink-0 text-brand" />
      <strong class="min-w-0 truncate text-ink">{{ auth.user.phone }}</strong>
    </span>
    <div class="mx-0.5 h-4 w-px bg-divider" />
    <span class="status-item flex items-center gap-2" :title="t('bottomBar.dateTimeTooltip')">
      <i class="pi pi-clock shrink-0 text-brand" />
      <strong class="min-w-0 truncate text-ink">{{ formatDateTime(props.info.timestamp) }}</strong>
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

  <Popover ref="popoverRef">
    <div class="flex min-w-[180px] flex-col py-1">
      <button
        type="button"
        class="flex w-full items-center gap-3 rounded px-3 py-2 text-sm transition-colors hover:bg-surface-hover"
        :title="t('common.profile')"
        @click="openProfile"
      >
        <i class="pi pi-cog text-brand" />
        <span>{{ t('common.profile') }}</span>
      </button>

      <div class="my-1 border-t border-divider" />

      <button
        type="button"
        class="flex w-full items-center gap-3 rounded px-3 py-2 text-sm transition-colors hover:bg-surface-hover"
        :title="t('common.switchLanguage')"
        @click="toggleLocale"
      >
        <i class="pi pi-language text-brand" />
        <span>{{ locale === 'vi' ? 'English' : 'Tiếng Việt' }}</span>
      </button>

      <button
        type="button"
        class="flex w-full items-center gap-3 rounded px-3 py-2 text-sm transition-colors hover:bg-surface-hover"
        :title="theme === 'dark' ? t('common.switchToLightMode') : t('common.switchToDarkMode')"
        @click="toggleTheme"
      >
        <i :class="['pi text-brand', theme === 'dark' ? 'pi-sun' : 'pi-moon']" />
        <span>{{ theme === 'dark' ? t('common.switchToLightMode') : t('common.switchToDarkMode') }}</span>
      </button>

      <div class="my-1 border-t border-divider" />

      <button
        type="button"
        class="flex w-full items-center gap-3 rounded px-3 py-2 text-sm text-red-500 transition-colors hover:bg-surface-hover"
        :title="t('common.logout')"
        @click="emit('logout')"
      >
        <i class="pi pi-sign-out" />
        <span>{{ t('common.logout') }}</span>
      </button>
    </div>
  </Popover>
</template>
