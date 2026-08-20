/** Types cho thư viện Skill — khớp với DTO ở `src-tauri/src/models/skill.rs`. */

/**
 * Danh mục skill — giá trị là `name` của một mục `master_data` có
 * `keygroup = "SKILL_TYPE"` (xem `useSkill.ts` / trang Master Data).
 */
export type SkillType = string;

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

/** Gợi ý stack theo danh mục (bổ trợ UI — không phải nguồn dữ liệu category). */
export const STACK_SUGGESTIONS: Record<string, string[]> = {
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

/** Bảng màu badge theo thứ tự hiển thị của category — dùng vòng lặp nên áp dụng được cho danh mục mới thêm qua Master Data. */
export const CATEGORY_BADGE_PALETTE: string[] = [
  "bg-slate-100 text-slate-700",
  "bg-blue-100 text-blue-700",
  "bg-violet-100 text-violet-700",
  "bg-cyan-100 text-cyan-700",
  "bg-orange-100 text-orange-700",
  "bg-emerald-100 text-emerald-700",
  "bg-pink-100 text-pink-700",
  "bg-amber-100 text-amber-700",
  "bg-teal-100 text-teal-700",
];
