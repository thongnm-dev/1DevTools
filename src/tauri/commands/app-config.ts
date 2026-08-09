import { safeInvoke } from "./_base";
import type { AppConfigData, SaveAppConfigRequest } from "@/models/app-config";

export function getAppConfig() {
  return safeInvoke<AppConfigData>("get_app_config");
}

export function saveAppConfig(request: SaveAppConfigRequest) {
  return safeInvoke<AppConfigData>("save_app_config", { request });
}
