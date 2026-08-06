/** Quyền menu của user sau khi gộp role + override riêng. */
export type EffectiveMenuPermission = {
  menu_key: string;
  is_allowed: boolean;
  /** Quyền suy ra từ các role của user, trước khi áp override riêng. */
  role_allowed: boolean;
  source: "user" | "role";
};
