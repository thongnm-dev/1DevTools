//! Business logic cho Master Data — chuẩn hoá dữ liệu, kiểm tra tên hợp lệ /
//! không trùng rồi uỷ quyền xuống `master_data_store`.

use crate::app::error::AppError;
use crate::app::result::AppResult;
use crate::database::master_data_store;
use crate::models::master_data::{MasterData, MasterDataRequest};

/// Liệt kê toàn bộ danh mục (theo keygroup + display_order).
pub async fn list_items() -> AppResult<Vec<MasterData>> {
    master_data_store::list_all().await
}

/// Thêm danh mục mới: tên không rỗng và chưa trùng.
pub async fn create_item(request: MasterDataRequest) -> AppResult<MasterData> {
    let name = request.name.trim().to_string();
    if name.is_empty() {
        return Err(AppError::new("Master data name is required."));
    }
    if master_data_store::name_exists(&name, None).await? {
        return Err(AppError::new(format!("Master data '{name}' already exists.")));
    }

    master_data_store::insert(
        &name,
        request.icon.trim(),
        request.keygroup.trim(),
        request.display_order,
        request.description.trim(),
    )
    .await
}

/// Cập nhật danh mục: kiểm tra tồn tại và tên mới không trùng mục khác.
pub async fn update_item(id: i32, request: MasterDataRequest) -> AppResult<MasterData> {
    let name = request.name.trim().to_string();
    if name.is_empty() {
        return Err(AppError::new("Master data name is required."));
    }

    master_data_store::find_by_id(id)
        .await?
        .ok_or_else(|| AppError::new(format!("Master data '{id}' not found.")))?;

    if master_data_store::name_exists(&name, Some(id)).await? {
        return Err(AppError::new(format!("Master data '{name}' already exists.")));
    }

    master_data_store::update(
        id,
        &name,
        request.icon.trim(),
        request.keygroup.trim(),
        request.display_order,
        request.description.trim(),
    )
    .await
}

/// Xoá danh mục khỏi hệ thống.
pub async fn delete_item(id: i32) -> AppResult<()> {
    if !master_data_store::delete_by_id(id).await? {
        return Err(AppError::new(format!("Master data '{id}' not found.")));
    }
    Ok(())
}
