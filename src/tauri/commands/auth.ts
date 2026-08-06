import { isMockMode, safeInvoke } from "./_base";
import { mockLoginResponse } from "./_mock-data";
import type { LoginRequest, LoginResponse } from "@/models/auth";

export function login(request: LoginRequest) {
  if (isMockMode()) {
    return new Promise<LoginResponse>((resolve) =>
      window.setTimeout(() => resolve({ ...mockLoginResponse, username: request.username || mockLoginResponse.username }), 400),
    );
  }
  return safeInvoke<LoginResponse>("login", { request });
}

export function requestPasswordReset(username: string) {
  return safeInvoke<string>("request_password_reset", { username });
}

export function verifyPasswordReset(username: string, code: string) {
  return safeInvoke<string>("verify_password_reset", { username, code });
}
