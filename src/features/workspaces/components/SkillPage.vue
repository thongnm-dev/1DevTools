<script setup lang="ts">
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";
import Button from "primevue/button";
import Dialog from "primevue/dialog";
import InputText from "primevue/inputtext";
import Select from "primevue/select";
import Textarea from "primevue/textarea";
import IconPickerDialog from "@/shared/components/IconPickerDialog.vue";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import { useSkill } from "../composables/useSkill";
import { useToast } from "@/shared/composables/useToast";
import { friendlyError } from "@/tauri/commands/_base";
import type { Skill, SkillCategory } from "@/models/skill";
import { DEFAULT_SKILL_ICON, SKILL_CATEGORY_META } from "@/models/skill";

const { t } = useI18n();
const toast = useToast();
const ctrl = useSkill();

const categoryOptions = computed(() =>
  (Object.keys(SKILL_CATEGORY_META) as SkillCategory[]).map((value) => ({ label: t(`skill.category.${value}`), value })),
);

function categoryBadgeClass(category: SkillCategory): string {
  return SKILL_CATEGORY_META[category]?.badgeClass ?? "bg-canvas text-muted";
}

function categoryLabel(category: SkillCategory): string {
  return t(`skill.category.${category}`);
}

// --- Add/Edit dialog ---
const showDialog = ref(false);
const editingId = ref<number | null>(null);
const name = ref("");
const description = ref("");
const icon = ref(DEFAULT_SKILL_ICON);
const category = ref<SkillCategory>("custom");
const instructions = ref("");
const tagsText = ref("");
const showIconPicker = ref(false);

function openCreateDialog() {
  editingId.value = null;
  name.value = "";
  description.value = "";
  icon.value = DEFAULT_SKILL_ICON;
  category.value = "custom";
  instructions.value = "";
  tagsText.value = "";
  showDialog.value = true;
}

function openEditDialog(skill: Skill) {
  editingId.value = skill.id;
  name.value = skill.name;
  description.value = skill.description;
  icon.value = skill.icon;
  category.value = skill.category;
  instructions.value = skill.instructions;
  tagsText.value = skill.tags.join(", ");
  showDialog.value = true;
}

async function saveSkill() {
  const trimmedName = name.value.trim();
  if (!trimmedName) return;
  const request = {
    name: trimmedName,
    description: description.value.trim(),
    icon: icon.value,
    category: category.value,
    instructions: instructions.value,
    tags: tagsText.value.split(",").map((s) => s.trim()).filter(Boolean),
  };
  if (editingId.value !== null) {
    await ctrl.updateSkill(editingId.value, request);
  } else {
    await ctrl.createSkill(request);
  }
  showDialog.value = false;
}

async function copyInstructions(skill: Skill) {
  try {
    await navigator.clipboard.writeText(skill.instructions);
    toast.success(t("skill.toast.copied"));
  } catch (e) {
    toast.error(friendlyError(e));
  }
}

// --- Delete confirmation ---
const showDeleteDialog = ref(false);
const deleteTarget = ref<Skill | null>(null);

function confirmDelete(skill: Skill) {
  deleteTarget.value = skill;
  showDeleteDialog.value = true;
}

async function executeDelete() {
  if (!deleteTarget.value) return;
  await ctrl.deleteSkill(deleteTarget.value.id);
  showDeleteDialog.value = false;
  deleteTarget.value = null;
}

const selectPt = {
  root: { class: "!bg-panel !border-divider" },
  label: { class: "!flex !items-center !text-xs !py-1.5 !text-ink" },
  option: { class: "!text-xs" },
};
</script>

<template>
  <div class="flex flex-1 flex-col gap-3 overflow-hidden">
    <div class="flex shrink-0 items-center gap-2">
      <span class="flex flex-1 items-center gap-2 rounded-md border border-divider bg-panel px-2">
        <i class="pi pi-search text-xs text-muted" />
        <InputText
          v-model="ctrl.searchQuery.value"
          class="embedded-input w-full border-0 !bg-transparent !py-1.5 !text-xs"
          :placeholder="t('skill.searchPlaceholder')"
        />
      </span>
      <Button icon="pi pi-plus" :label="t('skill.newSkill')" @click="openCreateDialog" />
    </div>

    <div class="flex-1 overflow-auto">
      <div v-if="ctrl.isLoading.value" class="p-8 text-center text-sm text-muted">{{ t("common.loading") }}</div>
      <div
        v-else-if="ctrl.filteredSkills.value.length === 0"
        class="flex h-full flex-col items-center justify-center gap-3 rounded-lg border border-dashed border-divider bg-panel/50 p-12 text-center"
      >
        <i class="pi pi-book text-4xl text-muted/60" />
        <p class="text-sm text-muted">{{ t("skill.emptyState") }}</p>
        <Button icon="pi pi-plus" :label="t('skill.newSkill')" @click="openCreateDialog" />
      </div>
      <div v-else class="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3">
        <div v-for="skill in ctrl.filteredSkills.value" :key="skill.id" class="flex flex-col gap-2 rounded-lg border border-divider bg-panel p-4 shadow-sm">
          <div class="flex items-start gap-2">
            <i :class="[skill.icon, 'text-xl text-brand']" />
            <div class="min-w-0 flex-1">
              <h3 class="section-title truncate">{{ skill.name }}</h3>
              <span :class="['mt-1 inline-block rounded-full px-2 py-0.5 text-[10px] font-bold', categoryBadgeClass(skill.category)]">
                {{ categoryLabel(skill.category) }}
              </span>
            </div>
            <div class="flex shrink-0 items-center gap-0.5">
              <Button icon="pi pi-copy" text rounded size="small" :title="t('skill.copyInstructions')" @click="copyInstructions(skill)" />
              <Button icon="pi pi-pencil" text rounded size="small" :title="t('skill.edit')" @click="openEditDialog(skill)" />
              <Button icon="pi pi-trash" text rounded size="small" severity="danger" :title="t('skill.delete')" @click="confirmDelete(skill)" />
            </div>
          </div>
          <p class="line-clamp-2 text-xs text-muted">{{ skill.description || t("skill.noDescription") }}</p>
          <div v-if="skill.tags.length" class="flex flex-wrap gap-1">
            <span v-for="tag in skill.tags" :key="tag" class="rounded-full bg-canvas px-2 py-0.5 text-[10px] text-secondary">{{ tag }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Add/Edit Skill Dialog -->
    <Dialog
      :visible="showDialog"
      class="w-full max-w-lg rounded-lg bg-panel shadow-xl"
      :closable="true"
      modal
      @update:visible="showDialog = $event"
    >
      <template #header>
        <h3 class="section-title">{{ editingId !== null ? t("skill.dialog.editTitle") : t("skill.dialog.newTitle") }}</h3>
      </template>

      <div class="space-y-4">
        <div class="flex items-end gap-3">
          <label class="block min-w-0 flex-1">
            <span class="text-xs font-bold text-muted">{{ t("skill.dialog.name") }} <span class="text-red-500">*</span></span>
            <InputText v-model="name" class="mt-1 w-full" :placeholder="t('skill.dialog.namePlaceholder')" autofocus />
          </label>
          <div class="block">
            <span class="text-xs font-bold text-muted">{{ t("skill.dialog.icon") }}</span>
            <div class="mt-1 flex items-center gap-2">
              <div class="flex h-10 items-center gap-2 rounded-md border border-divider bg-panel px-3">
                <i :class="[icon, 'text-brand']" />
                <InputText v-model="icon" class="embedded-input w-24 border-0 !bg-transparent !p-0 !text-sm" placeholder="pi pi-book" />
              </div>
              <Button icon="pi pi-th-large" severity="secondary" outlined :title="t('skill.dialog.browseIcons')" @click="showIconPicker = true" />
            </div>
          </div>
        </div>

        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("skill.dialog.category") }}</span>
          <Select v-model="category" :options="categoryOptions" optionLabel="label" optionValue="value" class="mt-1 w-full" :pt="selectPt" />
        </label>

        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("skill.dialog.description") }}</span>
          <InputText v-model="description" class="mt-1 w-full" :placeholder="t('skill.dialog.descriptionPlaceholder')" />
        </label>

        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("skill.dialog.instructions") }}</span>
          <Textarea v-model="instructions" rows="8" class="mt-1 w-full !text-xs" :placeholder="t('skill.dialog.instructionsPlaceholder')" />
        </label>

        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("skill.dialog.tags") }}</span>
          <InputText v-model="tagsText" class="mt-1 w-full" :placeholder="t('skill.dialog.tagsPlaceholder')" />
        </label>
      </div>

      <template #footer>
        <DialogFooter
          cancel-icon="pi pi-times"
          cancel-severity="danger"
          :confirm-label="editingId !== null ? t('common.save') : t('skill.dialog.create')"
          :confirm-icon="editingId !== null ? 'pi pi-check' : 'pi pi-plus'"
          :confirm-disabled="!name.trim()"
          @cancel="showDialog = false"
          @confirm="saveSkill"
        />
      </template>
    </Dialog>

    <!-- Skill Icon Picker Dialog -->
    <IconPickerDialog
      :visible="showIconPicker"
      :selected="icon"
      @update:visible="showIconPicker = $event"
      @select="(picked: string) => (icon = 'pi ' + picked)"
    />

    <!-- Delete Confirmation Dialog -->
    <Dialog
      :visible="showDeleteDialog"
      class="w-full max-w-sm rounded-lg bg-panel shadow-xl"
      :closable="true"
      modal
      @update:visible="showDeleteDialog = $event"
    >
      <template #header>
        <h3 class="section-title">{{ t("skill.deleteConfirm.title") }}</h3>
      </template>

      <p class="text-sm text-ink">{{ t("skill.deleteConfirm.message", { name: deleteTarget?.name ?? "" }) }}</p>

      <template #footer>
        <DialogFooter
          cancel-icon="pi pi-times"
          cancel-severity="danger"
          :confirm-label="t('skill.delete')"
          confirm-icon="pi pi-trash"
          confirm-severity="danger"
          @cancel="showDeleteDialog = false"
          @confirm="executeDelete"
        />
      </template>
    </Dialog>
  </div>
</template>
