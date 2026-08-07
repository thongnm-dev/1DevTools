import { safeInvoke } from "./_base";

export interface FileEntry {
  name: string;
  path: string;
  is_dir: boolean;
  size: number;
  modified: string;
  extension: string;
}

export interface ReadDirResult {
  path: string;
  entries: FileEntry[];
}

/** Hiển thị `path` trong Explorer/Finder — dùng bởi màn hình Git Desktop ("Show in folder"). */
export function explorerOpen(path: string) {
  return safeInvoke<void>("explorer_open", { path });
}

/** Liệt kê nội dung một thư mục — dùng bởi cây thư mục của màn hình Terminal. */
export function explorerReadDir(path: string) {
  return safeInvoke<ReadDirResult>("explorer_read_dir", { path });
}

/** Mở file bằng ứng dụng mặc định của hệ điều hành (khác `explorerOpen`, vốn chỉ chọn file trong Explorer). */
export function explorerOpenFile(path: string) {
  return safeInvoke<void>("explorer_open_file", { path });
}
