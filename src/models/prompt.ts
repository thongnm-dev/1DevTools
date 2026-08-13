/** Types cho thư viện Prompt — khớp với DTO ở `src-tauri/src/models/prompt.rs`. */

export interface Prompt {
  id: number;
  title: string;
  body: string;
  tags: string[];
  category: string;
  usage_count: number;
  created_at: string;
  updated_at: string;
}

export interface PromptRequest {
  title: string;
  body: string;
  tags: string[];
  category: string;
}

/** Tên biến `{{var}}` xuất hiện trong `body` — parse ở frontend, không lưu backend. */
export function extractPromptVariables(body: string): string[] {
  const matches = body.matchAll(/\{\{(\w+)\}\}/g);
  const names = new Set<string>();
  for (const m of matches) names.add(m[1]);
  return [...names];
}
