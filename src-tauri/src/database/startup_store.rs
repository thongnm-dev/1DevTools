//! Khởi tạo database khi ứng dụng khởi động.
//!
//! Tạo bảng và stored procedure nếu chưa tồn tại.
//! Được gọi một lần duy nhất trong Tauri setup hook.

use crate::app::error::AppError;
use crate::app::result::AppResult;
use crate::utils::pgsql_connect;
use tokio_postgres::Client;

/// Khởi tạo database: kết nối, tạo bảng, tạo stored procedure, seed dữ liệu.
///
/// Tạo bảng trước (fail-fast nếu lỗi), sau đó chạy stored procedures và seed data
/// độc lập — nếu một số SP fail thì các SP còn lại và seed data vẫn được thực thi.
pub async fn init() -> AppResult<()> {
    let client = pgsql_connect::connect().await?;

    // client
    //     .batch_execute(include_str!("../../../docs/database/schema.sql"))
    //     .await
    //     .map_err(|e| AppError::new(format!("Failed to create tables: {e}")))?;

    let mut errors = Vec::new();

    if let Err(e) = ensure_stored_procedures(&client).await {
        errors.push(e.to_string());
    }

    if let Err(e) = seed_data(&client).await {
        errors.push(e.to_string());
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(AppError::new(errors.join("\n")))
    }
}

/// Seed dữ liệu mặc định: tạo role admin và user admin nếu chưa tồn tại.
async fn seed_data(client: &Client) -> AppResult<()> {
    // client
    //     .batch_execute(include_str!("../../../docs/database/seed_data.sql"))
    //     .await
    //     .map_err(|e| AppError::new(format!("Failed to seed data: {e}")))?;

    Ok(())
}

/// Tạo hoặc cập nhật toàn bộ stored procedure từ các file SQL.
///
/// Thực thi tất cả SP và thu thập lỗi thay vì dừng lại ở SP đầu tiên bị lỗi.
///
/// Chỉ còn các SP thực sự được dùng bởi phần đã giữ lại (auth, menu_configs,
/// effective menu permissions, danh sách user) — các SP khác (project, daily
/// report/notes, role governance, AWS storage, download/upload history, AI
/// workflow/task...) đã bị loại cùng domain nghiệp vụ cũ tương ứng.
async fn ensure_stored_procedures(client: &Client) -> AppResult<()> {
    let procedures: &[(&str, &str)] = &[
        // // === Auth ===
        // ("sp_auth_find_user_by_username", include_str!("../../../docs/store-procedure/sp_auth_find_user_by_username.sql")),
        // ("sp_auth_get_user_roles", include_str!("../../../docs/store-procedure/sp_auth_get_user_roles.sql")),
        // ("sp_auth_reset_code_save", include_str!("../../../docs/store-procedure/sp_auth_reset_code_save.sql")),
        // ("sp_auth_reset_code_verify", include_str!("../../../docs/store-procedure/sp_auth_reset_code_verify.sql")),
        // ("sp_auth_reset_code_has_valid", include_str!("../../../docs/store-procedure/sp_auth_reset_code_has_valid.sql")),
        // ("sp_auth_reset_password", include_str!("../../../docs/store-procedure/sp_auth_reset_password.sql")),
        // // === User (list only) ===
        // ("sp_user_select_list", include_str!("../../../docs/store-procedure/sp_user_select_list.sql")),
        // // === Menu Config ===
        // ("sp_menu_config_select_list", include_str!("../../../docs/store-procedure/sp_menu_config_select_list.sql")),
        // // === Menu Permission (effective only) ===
        // ("sp_menu_permission_effective_select", include_str!("../../../docs/store-procedure/sp_menu_permission_effective_select.sql")),
    ];

    let mut errors = Vec::new();

    for (name, sql) in procedures {
        if let Err(e) = client.batch_execute(sql).await {
            errors.push(format!("Failed to create {name}: {e}"));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(AppError::new(errors.join("\n")))
    }
}
