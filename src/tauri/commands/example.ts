import { safeInvoke } from "./_base";
import type { CreateExampleItemRequest, ExampleItem } from "@/models/example";

export function listExampleItems() {
  return safeInvoke<ExampleItem[]>("example_list_items");
}

export function createExampleItem(request: CreateExampleItemRequest) {
  return safeInvoke<ExampleItem>("example_create_item", { request });
}
