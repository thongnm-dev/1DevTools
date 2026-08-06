//! Service kiểm tra kết nối mạng (internet).

use crate::utils::network::is_internet_reachable;

/// Kiểm tra ứng dụng có thể kết nối internet hay không.
pub async fn check_connection() -> bool {
    is_internet_reachable().await
}
