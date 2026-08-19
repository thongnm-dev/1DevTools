/** Types cho quản lý Master Data (danh mục dùng chung) — khớp với DTO ở
 * `src-tauri/src/models/master_data.rs`. */

export interface MasterData {
  id: number;
  /** Tên hiển thị (duy nhất). */
  name: string;
  /** Icon (lớp PrimeIcons, VD: "pi pi-tag"). */
  icon: string;
  /** Nhóm phân loại danh mục (VD: "category", "status"). */
  keygroup: string;
  /** Thứ tự hiển thị trong nhóm. */
  display_order: number;
  /** Mô tả ngắn. */
  description: string;
  created_at: string;
  updated_at: string;
}

/** Request đăng ký / cập nhật một mục danh mục. */
export interface MasterDataRequest {
  name: string;
  icon: string;
  keygroup: string;
  display_order: number;
  description: string;
}

export const DEFAULT_MASTER_DATA_ICON = "pi pi-tag";
