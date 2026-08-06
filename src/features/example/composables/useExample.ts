import { ref } from "vue";
import { createExampleItem, friendlyError, listExampleItems } from "@/tauri/commands";
import type { ExampleItem } from "@/models/example";

export function useExample() {
  const items = ref<ExampleItem[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function fetchItems() {
    loading.value = true;
    error.value = null;
    try {
      items.value = await listExampleItems();
    } catch (e) {
      error.value = friendlyError(e);
    } finally {
      loading.value = false;
    }
  }

  async function addItem(name: string) {
    error.value = null;
    try {
      const created = await createExampleItem({ name });
      items.value.push(created);
    } catch (e) {
      error.value = friendlyError(e);
    }
  }

  return { items, loading, error, fetchItems, addItem };
}
