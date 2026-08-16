/** Types cho thư viện Skill — khớp với DTO ở `src-tauri/src/models/skill.rs`. */

export type SkillType =
  | "general"
  | "frontend"
  | "backend"
  | "mobile"
  | "devops"
  | "translation"
  | "design"
  | "writing"
  | "data"
  | "custom";

// Backward compat alias (dùng trong các component cũ)
export type SkillCategory = SkillType;

export interface Skill {
  id: number;
  name: string;
  description: string;
  icon: string;
  category: SkillType;
  stack: string;
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
  category: SkillType;
  stack: string;
  instructions: string;
  tags: string[];
}

export interface SkillTypeMeta {
  icon: string;
  badgeClass: string;
}

export const SKILL_TYPE_META: Record<SkillType, SkillTypeMeta> = {
  general:     { icon: "pi pi-star",      badgeClass: "bg-slate-100 text-slate-700" },
  frontend:    { icon: "pi pi-desktop",   badgeClass: "bg-blue-100 text-blue-700" },
  backend:     { icon: "pi pi-server",    badgeClass: "bg-violet-100 text-violet-700" },
  mobile:      { icon: "pi pi-mobile",    badgeClass: "bg-cyan-100 text-cyan-700" },
  devops:      { icon: "pi pi-wrench",    badgeClass: "bg-orange-100 text-orange-700" },
  translation: { icon: "pi pi-language",  badgeClass: "bg-emerald-100 text-emerald-700" },
  design:      { icon: "pi pi-palette",   badgeClass: "bg-pink-100 text-pink-700" },
  writing:     { icon: "pi pi-file-edit", badgeClass: "bg-amber-100 text-amber-700" },
  data:        { icon: "pi pi-chart-bar", badgeClass: "bg-teal-100 text-teal-700" },
  custom:      { icon: "pi pi-cog",       badgeClass: "bg-canvas text-muted" },
};

// Backward compat
export const SKILL_CATEGORY_META = SKILL_TYPE_META;

export const STACK_SUGGESTIONS: Record<SkillType, string[]> = {
  general:     [],
  frontend:    ["Vue 3", "React", "Angular", "Svelte", "Next.js", "Nuxt.js", "TypeScript", "Tailwind CSS"],
  backend:     ["Python", "FastAPI", "Django", "Node.js", "NestJS", "Express", "Go", "Rust", "Java", "Spring Boot", "PHP", "Laravel"],
  mobile:      ["React Native", "Flutter", "Swift", "Kotlin", "Expo"],
  devops:      ["Docker", "Kubernetes", "GitHub Actions", "Terraform", "AWS", "GCP", "Azure"],
  translation: ["EN → VI", "VI → EN", "JA → EN", "EN → JA", "VI → JA", "JA → VI", "ZH → EN"],
  design:      ["Figma", "CSS", "Tailwind CSS", "Framer", "Adobe XD", "Sketch"],
  writing:     ["Technical Docs", "Blog", "API Docs", "User Guide", "SEO", "Copywriting"],
  data:        ["Python", "SQL", "Pandas", "PySpark", "dbt", "Tableau", "Power BI"],
  custom:      [],
};
