<script setup lang="ts">
import { onMounted, ref } from "vue";
import Button from "primevue/button";
import InputText from "primevue/inputtext";
import { useExample } from "../composables/useExample";

const { items, loading, error, fetchItems, addItem } = useExample();
const newItemName = ref("");

onMounted(fetchItems);

async function handleAdd() {
  const name = newItemName.value.trim();
  if (!name) return;
  await addItem(name);
  newItemName.value = "";
}
</script>

<template>
  <div class="flex flex-col gap-4">
    <h1 class="text-lg font-semibold text-ink">Example</h1>

    <div class="flex gap-2">
      <InputText v-model="newItemName" placeholder="New item name" class="flex-1" @keyup.enter="handleAdd" />
      <Button label="Add" @click="handleAdd" />
    </div>

    <p v-if="error" class="text-sm text-red-500">{{ error }}</p>
    <p v-else-if="loading" class="text-sm text-muted">Loading...</p>

    <ul v-else class="flex flex-col gap-1">
      <li v-for="item in items" :key="item.id" class="rounded border border-border bg-surface px-3 py-2 text-ink">
        {{ item.name }}
      </li>
      <li v-if="items.length === 0" class="text-sm text-muted">No items yet.</li>
    </ul>
  </div>
</template>
