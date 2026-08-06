import { safeInvoke } from "./_base";
import type { LoginRequest, LoginResponse } from "@/models/auth";

export function login(request: LoginRequest) {
  return safeInvoke<LoginResponse>("login", { request });
}

export function requestPasswordReset(username: string) {
  return safeInvoke<string>("request_password_reset", { username });
}

export function verifyPasswordReset(username: string, code: string) {
  return safeInvoke<string>("verify_password_reset", { username, code });
}
