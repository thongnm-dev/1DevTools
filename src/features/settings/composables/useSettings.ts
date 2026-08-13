import { computed, ref, watch } from "vue";
import { applyTheme } from "@/shared/config/themeTokens";
import { canUseTauriRuntime, friendlyError, getSettings, saveSettings } from "@/tauri/commands";
import type { AppSettings as TauriAppSettings } from "@/tauri/commands/settings";
import { useAuthStore } from "@/app/stores/auth";
import { useToast } from "@/shared/composables/useToast";
import { useTabNavigation } from "@/shared/composables/useTabNavigation";
import { useLocale } from "@/shared/composables/useLocale";
import type { AppLocale } from "@/shared/i18n";

export type ThemeMode = "light" | "dark";
export type LanguageCode = AppLocale;

export type UserSettings = {
  username: string;
  password: string;
  fullName: string;
  email: string;
  phone: string;
  address: string;
  position: string;
};

type StoredSettings = {
  language: LanguageCode;
  theme: ThemeMode;
  tabMode: boolean;
  user: UserSettings;
};

const SETTINGS_KEY = "1devtools.settings";

const defaultUser: UserSettings = {
  username: "",
  password: "",
  fullName: "",
  email: "",
  phone: "",
  address: "",
  position: "",
};

function isLanguageCode(v: unknown): v is LanguageCode {
  return v === "vi" || v === "en";
}

function cloneSettings(s: StoredSettings): StoredSettings {
  return JSON.parse(JSON.stringify(s));
}

function fromTauri(ts: TauriAppSettings): StoredSettings {
  return {
    user: {
      username: ts.user.username,
      password: "",
      fullName: ts.user.full_name,
      email: ts.user.email,
      phone: ts.user.phone,
      address: ts.user.address,
      position: ts.user.position,
    },
    theme: ts.theme === "dark" ? "dark" : "light",
    language: isLanguageCode(ts.language) ? ts.language : "vi",
    tabMode: ts.tab_mode ?? false,
  };
}

function toTauriRequest(s: StoredSettings, userId: number) {
  return {
    user_id: userId,
    user: {
      username: s.user.username,
      password: "",
      full_name: s.user.fullName,
      email: s.user.email,
      phone: s.user.phone,
      address: s.user.address,
      position: s.user.position,
    },
    theme: s.theme,
    language: s.language,
    tab_mode: s.tabMode,
  };
}

function loadFromLocal(authUser: { username: string; full_name: string; email: string } | null): StoredSettings {
  try {
    const saved = window.localStorage.getItem(SETTINGS_KEY);
    const parsed = saved ? (JSON.parse(saved) as Partial<StoredSettings & { locale?: string }>) : {};
    return {
      user: {
        ...defaultUser,
        ...parsed.user,
        username: parsed.user?.username || authUser?.username || "",
        fullName: parsed.user?.fullName || authUser?.full_name || "",
        email: parsed.user?.email || authUser?.email || "",
      },
      theme: parsed.theme === "dark" ? "dark" : "light",
      language: isLanguageCode(parsed.language ?? parsed.locale) ? (parsed.language ?? parsed.locale) as LanguageCode : "vi",
      tabMode: parsed.tabMode === true,
    };
  } catch {
    return {
      user: {
        ...defaultUser,
        username: authUser?.username || "",
        fullName: authUser?.full_name || "",
        email: authUser?.email || "",
      },
      theme: "light",
      language: "vi",
      tabMode: false,
    };
  }
}

export function useSettings() {
  const authStore = useAuthStore();
  const toast = useToast();
  const tabNav = useTabNavigation();
  const locale = useLocale();

  const userId = computed(() => authStore.user?.user_id ?? 0);
  const savedSnapshot = ref<StoredSettings>(loadFromLocal(authStore.user));
  const settings = ref<StoredSettings>(cloneSettings(savedSnapshot.value));
  const loading = ref(false);
  const error = ref<string | null>(null);

  const isDirty = computed(() => JSON.stringify(settings.value) !== JSON.stringify(savedSnapshot.value));

  watch(
    () => settings.value.theme,
    (theme) => applyTheme(theme),
  );

  watch(
    () => settings.value.tabMode,
    (enabled) => tabNav.setTabMode(enabled),
  );

  async function loadFromBackend() {
    if (!canUseTauriRuntime() || !userId.value) return;
    loading.value = true;
    error.value = null;
    try {
      const result = await getSettings(userId.value);
      const loaded = fromTauri(result);
      savedSnapshot.value = loaded;
      settings.value = cloneSettings(loaded);
    } catch (e) {
      error.value = friendlyError(e);
    } finally {
      loading.value = false;
    }
  }

  loadFromBackend();

  async function save() {
    if (canUseTauriRuntime() && userId.value) {
      loading.value = true;
      error.value = null;
      try {
        const result = await saveSettings(toTauriRequest(settings.value, userId.value));
        const saved = fromTauri(result);
        savedSnapshot.value = saved;
        settings.value = cloneSettings(saved);
        locale.setLocale(settings.value.language);
        toast.success("Settings saved successfully.");
      } catch (e) {
        error.value = friendlyError(e);
        toast.error(error.value);
      } finally {
        loading.value = false;
      }
    } else {
      window.localStorage.setItem(SETTINGS_KEY, JSON.stringify(settings.value));
      savedSnapshot.value = cloneSettings(settings.value);
      locale.setLocale(settings.value.language);
      toast.success("Settings saved successfully.");
    }
  }

  function discard() {
    settings.value = cloneSettings(savedSnapshot.value);
  }

  function updateUser(key: keyof UserSettings, value: string) {
    settings.value.user[key] = value;
  }

  function updateTheme(theme: ThemeMode) {
    settings.value.theme = theme;
  }

  function updateLanguage(language: LanguageCode) {
    settings.value.language = language;
  }

  function updateTabMode(enabled: boolean) {
    settings.value.tabMode = enabled;
  }

  return {
    settings,
    isDirty,
    loading,
    error,
    save,
    discard,
    updateUser,
    updateTheme,
    updateLanguage,
    updateTabMode,
  };
}
