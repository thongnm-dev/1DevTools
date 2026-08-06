// Maps stable backend error codes (AppErrorPayload.code) to i18n keys.
// A code with no entry here falls back to the backend's English message.
export const ERROR_CODE_KEYS: Record<string, string> = {
  AUTH_REQUIRED_FIELDS: "errors.auth.requiredFields",
  AUTH_INVALID_CREDENTIALS: "errors.auth.invalidCredentials",
  AUTH_ACCOUNT_DISABLED: "errors.auth.accountDisabled",
  AUTH_EMAIL_NOT_CONFIGURED: "errors.auth.emailNotConfigured",
  AUTH_USERNAME_REQUIRED: "errors.auth.usernameRequired",
  AUTH_ACCOUNT_NOT_FOUND: "errors.auth.accountNotFound",
  AUTH_FIELDS_REQUIRED: "errors.auth.fieldsRequired",
  AUTH_RESET_CODE_EXPIRED: "errors.auth.resetCodeExpired",
  AUTH_RESET_CODE_INVALID: "errors.auth.resetCodeInvalid",
  AUTH_RESET_FAILED: "errors.auth.resetFailed",
  INTERNAL_ERROR: "errors.common.internal",
};
