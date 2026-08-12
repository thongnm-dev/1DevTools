/** Types cho chức năng Dev Runner — khớp với DTO ở `src-tauri/src/models/dev_runner.rs`. */

export type CommandSource = "auto" | "custom";

export type CommandCategory =
  | "npm"
  | "flutter"
  | "maven"
  | "gradle"
  | "cargo"
  | "go"
  | "python"
  | "dotnet"
  | "make"
  | "docker"
  | "custom";

export interface DevCommand {
  id: string;
  label: string;
  command: string;
  category: CommandCategory;
  source: CommandSource;
  source_file: string;
}
