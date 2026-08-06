import { createI18n } from "vue-i18n";
import vi from "./locales/vi";
import en from "./locales/en";

export type AppLocale = "vi" | "en";

export const SUPPORTED_LOCALES: AppLocale[] = ["vi", "en"];
export const DEFAULT_LOCALE: AppLocale = "vi";

export function isAppLocale(value: unknown): value is AppLocale {
  return typeof value === "string" && SUPPORTED_LOCALES.includes(value as AppLocale);
}

export const i18n = createI18n({
  legacy: false,
  locale: DEFAULT_LOCALE,
  fallbackLocale: "en",
  messages: { vi, en },
});
