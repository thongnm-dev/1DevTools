/** Types cho thư viện Rule — khớp với DTO ở `src-tauri/src/models/rule.rs`. */

export interface Rule {
  id: number;
  name: string;
  description: string;
  content: string;
  tags: string[];
  created_at: string;
  updated_at: string;
}

export interface RuleRequest {
  name: string;
  description: string;
  content: string;
  tags: string[];
}
