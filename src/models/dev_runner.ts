/** Types cho chức năng Dev Runner — khớp với DTO ở `src-tauri/src/models/dev_runner.rs`. */

export type CommandSource = "auto" | "custom";

export type CommandKind = "run" | "test" | "build";

export const COMMAND_KIND_META: Record<CommandKind, { icon: string; badgeClass: string }> = {
  run: { icon: "pi-play", badgeClass: "bg-emerald-100 text-emerald-700" },
  test: { icon: "pi-check-circle", badgeClass: "bg-sky-100 text-sky-700" },
  build: { icon: "pi-box", badgeClass: "bg-amber-100 text-amber-700" },
};

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
  kind: CommandKind;
}
