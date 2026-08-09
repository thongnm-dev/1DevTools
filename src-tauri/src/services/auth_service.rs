//! Business logic xác thực: đăng nhập và quy trình đặt lại mật khẩu qua email.
//!
//! Các lỗi trả về đều gắn mã (`AppError::with_code`) để frontend hiển thị thông
//! điệp phù hợp và không lộ chi tiết (ví dụ sai user vs sai mật khẩu đều trả cùng
//! mã `AUTH_INVALID_CREDENTIALS`).

use crate::app::error::AppError;
use crate::app::result::AppResult;
use crate::database::auth_store;
use crate::models::auth::{LoginRequest, LoginResponse};
use crate::utils::email;

/// Đăng nhập: kiểm tra trường bắt buộc, tài khoản tồn tại và đang active, xác
/// minh mật khẩu bằng bcrypt, rồi trả về thông tin phiên kèm danh sách role.
pub async fn login(request: LoginRequest) -> AppResult<LoginResponse> {
    let username = request.username.trim();
    let password = request.password.trim();

    if username.is_empty() || password.is_empty() {
        return Err(AppError::with_code(
            "AUTH_REQUIRED_FIELDS",
            "Username and password are required.",
        ));
    }

    let found = auth_store::find_user_by_username(username).await?;

    let user = found
        .ok_or_else(|| AppError::with_code("AUTH_INVALID_CREDENTIALS", "Invalid username or password."))?;

    if !user.is_active {
        return Err(AppError::with_code(
            "AUTH_ACCOUNT_DISABLED",
            "Account is disabled. Please contact administrator.",
        ));
    }

    let valid = bcrypt::verify(password, &user.password_hash)
        .map_err(|e| AppError::with_code("INTERNAL_ERROR", format!("Password verification failed: {e}")))?;

    if !valid {
        return Err(AppError::with_code("AUTH_INVALID_CREDENTIALS", "Invalid username or password."));
    }

    let roles = auth_store::get_user_roles(user.id).await?;

    Ok(LoginResponse {
        user_id: user.id,
        username: user.username,
        full_name: user.full_name,
        email: user.email,
        roles,
    })
}

/// Bước 1 quên mật khẩu: kiểm tra tài khoản hợp lệ & có email, sinh mã 6 số,
/// gửi email và lưu mã (hết hạn sau 30 phút). Trả về email đã che (masked) để UI hiển thị.
pub async fn request_password_reset(username: &str) -> AppResult<String> {
    let username = username.trim();
    if username.is_empty() {
        return Err(AppError::with_code("AUTH_USERNAME_REQUIRED", "Username is required."));
    }

    let user = auth_store::find_user_by_username(username)
        .await?
        .ok_or_else(|| AppError::with_code("AUTH_ACCOUNT_NOT_FOUND", "Account not found."))?;

    if !user.is_active {
        return Err(AppError::with_code(
            "AUTH_ACCOUNT_DISABLED",
            "Account is disabled. Please contact administrator.",
        ));
    }

    if user.email.trim().is_empty() {
        return Err(AppError::with_code(
            "AUTH_EMAIL_NOT_CONFIGURED",
            "Account has no email configured. Please contact administrator.",
        ));
    }

    let code = generate_code();
    let expires_at = chrono::Utc::now() + chrono::Duration::minutes(30);

    email::send_reset_code(&user.email, &user.full_name, &code).await?;

    auth_store::save_reset_code(user.id, &code, expires_at).await?;

    Ok(mask_email(&user.email))
}

/// Bước 2 quên mật khẩu: xác minh `code`. Nếu đúng, đặt lại mật khẩu về giá trị
/// mặc định và trả về mật khẩu đó cho UI. Phân biệt lỗi mã sai vs mã hết hạn.
pub async fn verify_password_reset(username: &str, code: &str) -> AppResult<String> {
    let username = username.trim();
    let code = code.trim();

    if username.is_empty() || code.is_empty() {
        return Err(AppError::with_code("AUTH_FIELDS_REQUIRED", "All fields are required."));
    }

    let user = auth_store::find_user_by_username(username)
        .await?
        .ok_or_else(|| AppError::with_code("AUTH_ACCOUNT_NOT_FOUND", "Account not found."))?;

    let valid = auth_store::verify_reset_code(user.id, code).await?;
    if !valid {
        if !auth_store::has_unexpired_code(user.id).await? {
            return Err(AppError::with_code(
                "AUTH_RESET_CODE_EXPIRED",
                "Verification code has expired. Please request a new one.",
            ));
        }
        return Err(AppError::with_code("AUTH_RESET_CODE_INVALID", "Verification code is incorrect."));
    }

    let default_password = "Aa@123456";
    let password_hash = bcrypt::hash(default_password, 12)
        .map_err(|e| AppError::with_code("INTERNAL_ERROR", format!("Failed to hash password: {e}")))?;

    let updated = auth_store::reset_password(user.id, &password_hash).await?;
    if !updated {
        return Err(AppError::with_code("AUTH_RESET_FAILED", "Failed to reset password."));
    }

    Ok(default_password.to_string())
}

/// Sinh mã xác nhận ngẫu nhiên 6 chữ số (0-padded), ví dụ "004271".
fn generate_code() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen_range(0..1_000_000);
    format!("{n:06}")
}

/// Che bớt phần local của email để hiển thị an toàn, ví dụ `jo***n@gmail.com`.
fn mask_email(email: &str) -> String {
    let parts: Vec<&str> = email.splitn(2, '@').collect();
    if parts.len() != 2 {
        return "***".to_string();
    }
    let local = parts[0];
    let domain = parts[1];
    let masked_local = if local.len() <= 2 {
        format!("{}***", &local[..1])
    } else {
        format!("{}***{}", &local[..2], &local[local.len() - 1..])
    };
    format!("{masked_local}@{domain}")
}
