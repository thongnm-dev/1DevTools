import { createApp } from "vue";
import { registerPlugins } from "@/app/plugins";
import { router } from "@/app/router";
import { applyStoredTheme, injectForceLightStyle } from "@/shared/config/themeTokens";
import { applyStoredLocale } from "@/shared/composables/useLocale";
import App from "@/App.vue";

import "primeicons/primeicons.css";
import "./styles.css";

injectForceLightStyle();
applyStoredTheme();
applyStoredLocale();

const app = createApp(App);
registerPlugins(app);
app.use(router);
app.mount("#app");
