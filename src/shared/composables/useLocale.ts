import { i18n, DEFAULT_LOCALE, isAppLocale, type AppLocale } from "@/shared/i18n";

// Same storage key/object used by the theme toggle (see themeTokens.ts) so both
// preferences live together under one `msh.app.settings` blob.
const SETTINGS_KEY = "1devtools.settings";

type StoredSettings = { theme?: string; locale?: string };

function readSettings(): StoredSettings {
  try {
    const saved = window.localStorage.getItem(SETTINGS_KEY);
    return saved ? (JSON.parse(saved) as StoredSettings) : {};
  } catch {
    return {};
  }
}

function writeSettings(patch: StoredSettings) {
  window.localStorage.setItem(SETTINGS_KEY, JSON.stringify({ ...readSettings(), ...patch }));
}

function loadStoredLocale(): AppLocale {
  const saved = readSettings().locale;
  return isAppLocale(saved) ? saved : DEFAULT_LOCALE;
}

/** Applies the persisted locale to the i18n instance. Call once on app boot. */
export function applyStoredLocale() {
  i18n.global.locale.value = loadStoredLocale();
}

export function useLocale() {
  function setLocale(locale: AppLocale) {
    i18n.global.locale.value = locale;
    writeSettings({ locale });
  }

  return {
    locale: i18n.global.locale,
    setLocale,
  };
}
