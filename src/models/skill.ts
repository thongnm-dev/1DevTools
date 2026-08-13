/** Types cho thư viện Skill — khớp với DTO ở `src-tauri/src/models/skill.rs`. */

export type SkillCategory = "implement" | "review" | "test" | "release" | "docs" | "custom";

export interface Skill {
  id: number;
  name: string;
  description: string;
  icon: string;
  category: SkillCategory;
  instructions: string;
  tags: string[];
  created_at: string;
  updated_at: string;
}

export const DEFAULT_SKILL_ICON = "pi pi-book";

export interface SkillRequest {
  name: string;
  description: string;
  icon: string;
  category: SkillCategory;
  instructions: string;
  tags: string[];
}

export const SKILL_CATEGORY_META: Record<SkillCategory, { badgeClass: string }> = {
  implement: { badgeClass: "bg-violet-100 text-violet-700" },
  review: { badgeClass: "bg-amber-100 text-amber-700" },
  test: { badgeClass: "bg-sky-100 text-sky-700" },
  release: { badgeClass: "bg-emerald-100 text-emerald-700" },
  docs: { badgeClass: "bg-fuchsia-100 text-fuchsia-700" },
  custom: { badgeClass: "bg-canvas text-muted" },
};
