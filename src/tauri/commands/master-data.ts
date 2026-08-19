import { safeInvoke } from "./_base";
import type { MasterData, MasterDataRequest } from "@/models/master-data";

export function masterDataList() {
  return safeInvoke<MasterData[]>("master_data_list");
}

export function masterDataCreate(request: MasterDataRequest) {
  return safeInvoke<MasterData>("master_data_create", { request });
}

export function masterDataUpdate(id: number, request: MasterDataRequest) {
  return safeInvoke<MasterData>("master_data_update", { id, request });
}

export function masterDataDelete(id: number) {
  return safeInvoke<void>("master_data_delete", { id });
}
