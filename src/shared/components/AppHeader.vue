<script setup lang="ts">
import { ref } from "vue";
import type { AppRoute } from "@/app/router/routes";
import Button from "primevue/button";
import { applyTheme, type ThemeMode } from "@/shared/config/themeTokens";
import { useNavigationHistory } from "@/shared/composables/useNavigationHistory";
import { useTabNavigation } from "@/shared/composables/useTabNavigation";

defineProps<{
  route: AppRoute;
  username?: string;
}>();

const emit = defineEmits<{
  logout: [];
}>();

const SETTINGS_KEY = "msh.app.settings";

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
const { canGoBack, backTitle, goBack } = useNavigationHistory();
const { tabMode } = useTabNavigation();

function toggleTheme() {
  theme.value = theme.value === "dark" ? "light" : "dark";
  window.localStorage.setItem(SETTINGS_KEY, JSON.stringify({ theme: theme.value }));
  applyTheme(theme.value);
}
</script>

<template>
  <header class="flex items-start justify-between gap-4">
    <div>
      <h2 class="text-xl font-bold leading-tight">{{ route.title }}</h2>
      <p class="mt-[2px] text-[11px] text-secondary">{{ route.subtitle }}</p>
      <nav class="mt-2 flex items-center gap-2 text-xs font-semibold text-muted" aria-label="Breadcrumb">
        <span>Home</span>
        <template v-if="route.breadcrumbs?.length">
          <template v-for="(crumb, i) in route.breadcrumbs" :key="i">
            <span class="text-divider">/</span>
            <span :class="i === route.breadcrumbs.length - 1 ? 'text-brand' : ''">{{ crumb }}</span>
          </template>
        </template>
        <template v-else>
          <span class="text-divider">/</span>
          <span class="text-brand">{{ route.title }}</span>
        </template>
      </nav>
      <Button
        v-if="canGoBack && !tabMode"
        icon="pi pi-arrow-left"
        :label="backTitle ? `Back to ${backTitle}` : 'Back'"
        severity="secondary"
        outlined
        size="small"
        :title="backTitle ? `Back to ${backTitle}` : 'Back'"
        class="mt-3"
        @click="goBack"
      />
    </div>
    <div class="flex shrink-0 items-center gap-2">
      <Button
        :icon="theme === 'dark' ? 'pi pi-sun' : 'pi pi-moon'"
        severity="secondary"
        outlined
        rounded
        size="small"
        :title="theme === 'dark' ? 'Switch to light mode' : 'Switch to dark mode'"
        @click="toggleTheme"
      />
      <Button
        v-if="username"
        icon="pi pi-sign-out"
        label="Logout"
        severity="secondary"
        outlined
        size="small"
        title="Logout"
        @click="emit('logout')"
      />
    </div>
  </header>
</template>
