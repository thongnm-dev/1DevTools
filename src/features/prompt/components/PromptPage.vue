<script setup lang="ts">
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";
import Button from "primevue/button";
import Dialog from "primevue/dialog";
import InputText from "primevue/inputtext";
import Textarea from "primevue/textarea";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import { usePrompt } from "../composables/usePrompt";
import type { Prompt } from "@/models/prompt";
import { extractPromptVariables } from "@/models/prompt";

const { t } = useI18n();
const ctrl = usePrompt();

function formatDate(iso: string): string {
  return new Date(iso).toLocaleDateString([], { month: "short", day: "numeric" });
}

// --- Add/Edit dialog ---
const showDialog = ref(false);
const editingId = ref<number | null>(null);
const title = ref("");
const category = ref("");
const body = ref("");
const tagsText = ref("");

const bodyVariables = computed(() => extractPromptVariables(body.value));
const bodyVariableLabels = computed(() => bodyVariables.value.map((v) => "{{" + v + "}}"));

function openCreateDialog() {
  editingId.value = null;
  title.value = "";
  category.value = "";
  body.value = "";
  tagsText.value = "";
  showDialog.value = true;
}

function openEditDialog(prompt: Prompt) {
  editingId.value = prompt.id;
  title.value = prompt.title;
  category.value = prompt.category;
  body.value = prompt.body;
  tagsText.value = prompt.tags.join(", ");
  showDialog.value = true;
}

async function savePrompt() {
  const trimmedTitle = title.value.trim();
  if (!trimmedTitle) return;
  const request = {
    title: trimmedTitle,
    category: category.value.trim(),
    body: body.value,
    tags: tagsText.value.split(",").map((s) => s.trim()).filter(Boolean),
  };
  if (editingId.value !== null) {
    await ctrl.updatePrompt(editingId.value, request);
  } else {
    await ctrl.createPrompt(request);
  }
  showDialog.value = false;
}

// --- Delete confirmation ---
const showDeleteDialog = ref(false);
const deleteTarget = ref<Prompt | null>(null);

function confirmDelete(prompt: Prompt) {
  deleteTarget.value = prompt;
  showDeleteDialog.value = true;
}

async function executeDelete() {
  if (!deleteTarget.value) return;
  await ctrl.deletePrompt(deleteTarget.value.id);
  showDeleteDialog.value = false;
  deleteTarget.value = null;
}
</script>

<template>
  <div class="flex flex-1 flex-col gap-3 overflow-hidden">
    <div class="flex shrink-0 items-center gap-2">
      <span class="flex flex-1 items-center gap-2 rounded-md border border-divider bg-panel px-2">
        <i class="pi pi-search text-xs text-muted" />
        <InputText
          v-model="ctrl.searchQuery.value"
          class="embedded-input w-full border-0 !bg-transparent !py-1.5 !text-xs"
          :placeholder="t('prompt.searchPlaceholder')"
        />
      </span>
      <Button icon="pi pi-plus" :label="t('prompt.newPrompt')" @click="openCreateDialog" />
    </div>

    <div class="flex-1 overflow-auto rounded-lg border border-divider bg-panel shadow-sm">
      <div v-if="ctrl.isLoading.value" class="p-8 text-center text-sm text-muted">{{ t("common.loading") }}</div>
      <div
        v-else-if="ctrl.filteredPrompts.value.length === 0"
        class="flex h-full flex-col items-center justify-center gap-3 p-12 text-center"
      >
        <i class="pi pi-comment text-4xl text-muted/60" />
        <p class="text-sm text-muted">{{ t("prompt.emptyState") }}</p>
        <Button icon="pi pi-plus" :label="t('prompt.newPrompt')" @click="openCreateDialog" />
      </div>
      <div v-else class="divide-y divide-divider">
        <div v-for="prompt in ctrl.filteredPrompts.value" :key="prompt.id" class="flex items-center gap-3 px-4 py-2.5">
          <i class="pi pi-comment shrink-0 text-muted" />
          <div class="min-w-0 flex-1">
            <div class="flex items-center gap-2">
              <span class="truncate text-sm font-bold text-ink">{{ prompt.title }}</span>
              <span v-if="prompt.category" class="shrink-0 rounded-full bg-canvas px-2 py-0.5 text-[10px] font-bold text-secondary">{{ prompt.category }}</span>
            </div>
            <div class="mt-0.5 flex flex-wrap items-center gap-1.5 text-[11px] text-muted">
              <span>{{ formatDate(prompt.updated_at) }}</span>
              <span v-if="prompt.usage_count">· {{ t("prompt.usedCount", { count: prompt.usage_count }) }}</span>
              <span v-for="tag in prompt.tags" :key="tag" class="rounded-full bg-canvas px-1.5 text-[10px] text-secondary">{{ tag }}</span>
            </div>
          </div>
          <div class="flex shrink-0 items-center gap-0.5">
            <Button icon="pi pi-copy" text rounded size="small" :title="t('prompt.copy')" @click="ctrl.copyPrompt(prompt)" />
            <Button icon="pi pi-pencil" text rounded size="small" :title="t('common.edit')" @click="openEditDialog(prompt)" />
            <Button icon="pi pi-trash" text rounded size="small" severity="danger" :title="t('common.delete')" @click="confirmDelete(prompt)" />
          </div>
        </div>
      </div>
    </div>

    <!-- Add/Edit Prompt Dialog -->
    <Dialog
      :visible="showDialog"
      class="w-full max-w-lg rounded-lg bg-panel shadow-xl"
      :closable="true"
      modal
      @update:visible="showDialog = $event"
    >
      <template #header>
        <h3 class="section-title">{{ editingId !== null ? t("prompt.dialog.editTitle") : t("prompt.dialog.newTitle") }}</h3>
      </template>

      <div class="space-y-4">
        <div class="flex items-end gap-3">
          <label class="block min-w-0 flex-1">
            <span class="text-xs font-bold text-muted">{{ t("prompt.dialog.title") }} <span class="text-red-500">*</span></span>
            <InputText v-model="title" class="mt-1 w-full" :placeholder="t('prompt.dialog.titlePlaceholder')" autofocus />
          </label>
          <label class="block min-w-0 flex-1">
            <span class="text-xs font-bold text-muted">{{ t("prompt.dialog.category") }}</span>
            <InputText v-model="category" class="mt-1 w-full" :placeholder="t('prompt.dialog.categoryPlaceholder')" />
          </label>
        </div>

        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("prompt.dialog.body") }}</span>
          <Textarea v-model="body" rows="8" class="mt-1 w-full !text-xs" :placeholder="t('prompt.dialog.bodyPlaceholder')" />
          <div class="mt-1.5 flex flex-wrap items-center gap-1.5 text-xs text-muted">
            <span>{{ t("prompt.dialog.variablesFound") }}</span>
            <template v-if="bodyVariableLabels.length">
              <span v-for="label in bodyVariableLabels" :key="label" class="rounded-full bg-canvas px-2 py-0.5 font-mono text-[11px] text-brand">{{ label }}</span>
            </template>
            <span v-else class="italic">{{ t("prompt.dialog.noVariables") }}</span>
          </div>
        </label>

        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("prompt.dialog.tags") }}</span>
          <InputText v-model="tagsText" class="mt-1 w-full" :placeholder="t('prompt.dialog.tagsPlaceholder')" />
        </label>
      </div>

      <template #footer>
        <DialogFooter
          cancel-icon="pi pi-times"
          cancel-severity="danger"
          :confirm-label="editingId !== null ? t('common.save') : t('common.create')"
          :confirm-icon="editingId !== null ? 'pi pi-check' : 'pi pi-plus'"
          :confirm-disabled="!title.trim()"
          @cancel="showDialog = false"
          @confirm="savePrompt"
        />
      </template>
    </Dialog>

    <!-- Delete Confirmation Dialog -->
    <Dialog
      :visible="showDeleteDialog"
      class="w-full max-w-sm rounded-lg bg-panel shadow-xl"
      :closable="true"
      modal
      @update:visible="showDeleteDialog = $event"
    >
      <template #header>
        <h3 class="section-title">{{ t("prompt.deleteConfirm.title") }}</h3>
      </template>

      <p class="text-sm text-ink">{{ t("prompt.deleteConfirm.message", { title: deleteTarget?.title ?? "" }) }}</p>

      <template #footer>
        <DialogFooter
          cancel-icon="pi pi-times"
          cancel-severity="danger"
          :confirm-label="t('common.delete')"
          confirm-icon="pi pi-trash"
          confirm-severity="danger"
          @cancel="showDeleteDialog = false"
          @confirm="executeDelete"
        />
      </template>
    </Dialog>
  </div>
</template>
