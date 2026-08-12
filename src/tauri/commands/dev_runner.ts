import { safeInvoke } from "./_base";
import type { DevCommand } from "@/models/dev_runner";

export function detectDevCommands(repoPath: string) {
  return safeInvoke<DevCommand[]>("detect_dev_commands", { repoPath });
}

export function loadCustomCommands(repoPath: string) {
  return safeInvoke<DevCommand[]>("load_custom_commands", { repoPath });
}

export function saveCustomCommands(repoPath: string, commands: DevCommand[]) {
  return safeInvoke<void>("save_custom_commands", { repoPath, commands });
}
