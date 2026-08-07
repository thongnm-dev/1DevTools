/** @type {import('tailwindcss').Config} */

// Theme colors are set as "R G B" CSS custom properties at runtime
// (see src/shared/config/themeTokens.ts), so utilities need the
// rgb(var(...) / <alpha-value>) wrapper to support opacity modifiers.
const withOpacity = (varName) => `rgb(var(${varName}) / <alpha-value>)`;

export default {
  content: ["./index.html", "./src/**/*.{vue,ts}"],
  darkMode: ["selector", '[data-theme="dark"]'],
  theme: {
    extend: {
      colors: {
        canvas: withOpacity("--color-canvas"),
        panel: withOpacity("--color-panel"),
        surface: withOpacity("--color-panel"),
        ink: withOpacity("--color-ink"),
        brand: withOpacity("--color-brand"),
        "on-brand": withOpacity("--color-on-brand"),
        muted: withOpacity("--color-text-muted"),
        secondary: withOpacity("--color-text-secondary"),
        border: withOpacity("--color-border"),
        divider: withOpacity("--color-border"),
        "divider-light": withOpacity("--color-border-light"),

        sidebar: withOpacity("--color-sidebar-bg"),
        "sidebar-border": withOpacity("--color-sidebar-border"),
        "sidebar-text": withOpacity("--color-sidebar-text"),
        "sidebar-text-active": withOpacity("--color-sidebar-text-active"),
        "sidebar-active": withOpacity("--color-sidebar-active-bg"),
        "sidebar-hover": withOpacity("--color-sidebar-hover-bg"),
        "sidebar-title": withOpacity("--color-sidebar-title"),

        bar: withOpacity("--color-bar-bg"),
        "bar-border": withOpacity("--color-bar-border"),
        "bar-text": withOpacity("--color-bar-text"),
        "bar-accent": withOpacity("--color-bar-accent"),
        "bar-strong": withOpacity("--color-bar-strong"),

        danger: withOpacity("--color-danger"),
        "danger-soft": withOpacity("--color-danger-soft"),
        "danger-border": withOpacity("--color-danger-border"),
        warning: withOpacity("--color-warning"),
        "warning-soft": withOpacity("--color-warning-soft"),
        "warning-border": withOpacity("--color-warning-border"),
        success: withOpacity("--color-success"),
        "success-soft": withOpacity("--color-success-soft"),
        "success-border": withOpacity("--color-success-border"),
        info: withOpacity("--color-info"),
        "info-soft": withOpacity("--color-info-soft"),
        "info-border": withOpacity("--color-info-border"),

        "code-bg": withOpacity("--color-code-bg"),
        "code-fg": withOpacity("--color-code-fg"),
      },
      fontSize: {
        "2xs": ["0.6875rem", { lineHeight: "1rem" }], // 11px
      },
      boxShadow: {
        card: "var(--shadow-card)",
        "card-panel": "var(--shadow-panel)",
        float: "var(--shadow-float)",
      },
    },
  },
  plugins: [],
};
