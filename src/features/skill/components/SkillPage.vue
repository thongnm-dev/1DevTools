<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import Button from "primevue/button";
import InputText from "primevue/inputtext";
import Fieldset from "primevue/fieldset";
import SkillFormDialog from "./SkillFormDialog.vue";
import SkillDeleteDialog from "./SkillDeleteDialog.vue";
import { useSkill } from "../composables/useSkill";
import { useToast } from "@/shared/composables/useToast";
import { friendlyError } from "@/tauri/commands/_base";
import type { Skill, SkillType, SkillRequest } from "@/models/skill";
import { SKILL_TYPE_META } from "@/models/skill";

type ViewMode = "grid" | "list";
const viewMode = ref<ViewMode>("grid");

const { t } = useI18n();
const toast = useToast();
const ctrl = useSkill();

function categoryBadgeClass(category: SkillType): string {
  return SKILL_TYPE_META[category]?.badgeClass ?? "bg-canvas text-muted";
}

function categoryLabel(category: SkillType): string {
  return t(`skill.category.${category}`);
}

// --- Add/Edit dialog ---
const showDialog = ref(false);
const editingSkill = ref<Skill | null>(null);

function openCreateDialog() {
  editingSkill.value = null;
  showDialog.value = true;
}

function openEditDialog(skill: Skill) {
  editingSkill.value = skill;
  showDialog.value = true;
}

async function saveSkill(request: SkillRequest) {
  if (editingSkill.value !== null) {
    await ctrl.updateSkill(editingSkill.value.id, request);
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
</script>

<template>
  <section class="flex min-h-0 flex-1 flex-col gap-4 overflow-hidden">

    <!-- Action bar -->
    <section class="flex items-center justify-end rounded-lg border border-divider bg-panel p-4 shadow-sm">
      <Button icon="pi pi-plus" :label="t('skill.newSkill')" size="small" @click="openCreateDialog" />
    </section>

    <!-- Search fieldset -->
    <Fieldset class="rounded-lg border border-divider bg-panel p-4 shadow-md fieldset-nested" :legend="t('common.searchLegend')" toggleable>
      <span class="flex items-center gap-2 rounded-md border border-divider bg-canvas px-2">
        <i class="pi pi-search text-xs text-muted" />
        <InputText
          v-model="ctrl.searchQuery.value"
          class="embedded-input w-full border-0 !bg-transparent !py-1.5 !text-xs"
          :placeholder="t('skill.searchPlaceholder')"
        />
      </span>
    </Fieldset>

    <!-- Skills panel -->
    <div class="flex min-h-0 flex-1 flex-col overflow-hidden rounded-lg border border-divider bg-panel shadow-sm">

      <!-- Panel header -->
      <div class="flex shrink-0 items-center gap-2 border-b border-divider px-3 py-2">
        <span class="flex-1 text-xs font-semibold text-default">
          {{ t('skill.title') }}
          <span class="ml-1 text-muted">({{ ctrl.filteredSkills.value.length }})</span>
        </span>

        <!-- Sort controls -->
        <div class="flex items-center rounded-md border border-divider overflow-hidden">
          <select
            v-model="ctrl.sortBy.value"
            class="border-0 bg-panel px-2 py-1 text-xs text-default focus:outline-none"
          >
            <option value="name">{{ t('skill.sortName') }}</option>
            <option value="category">{{ t('skill.sortCategory') }}</option>
            <option value="created_at">{{ t('skill.sortDate') }}</option>
          </select>
          <button
            class="border-l border-divider bg-panel px-2 py-1 text-muted transition-colors hover:text-default"
            :title="ctrl.sortDir.value === 'asc' ? t('skill.sortAsc') : t('skill.sortDesc')"
            @click="ctrl.sortDir.value = ctrl.sortDir.value === 'asc' ? 'desc' : 'asc'"
          >
            <i :class="['pi text-xs', ctrl.sortDir.value === 'asc' ? 'pi-sort-amount-up' : 'pi-sort-amount-down']" />
          </button>
        </div>

        <!-- View toggle -->
        <div class="flex rounded-md border border-divider overflow-hidden">
          <button
            :class="['px-2 py-1 text-xs transition-colors', viewMode === 'grid' ? 'bg-brand text-white' : 'bg-panel text-muted hover:text-default']"
            :title="t('skill.viewGrid')"
            @click="viewMode = 'grid'"
          >
            <i class="pi pi-th-large text-xs" />
          </button>
          <button
            :class="['px-2 py-1 text-xs transition-colors border-l border-divider', viewMode === 'list' ? 'bg-brand text-white' : 'bg-panel text-muted hover:text-default']"
            :title="t('skill.viewList')"
            @click="viewMode = 'list'"
          >
            <i class="pi pi-list text-xs" />
          </button>
        </div>
      </div>

      <!-- Panel content -->
      <div class="flex-1 overflow-auto p-3">
        <div v-if="ctrl.isLoading.value" class="p-8 text-center text-sm text-muted">{{ t("common.loading") }}</div>
        <div
          v-else-if="ctrl.filteredSkills.value.length === 0"
          class="flex h-full flex-col items-center justify-center gap-3 rounded-lg border border-dashed border-divider bg-canvas/50 p-12 text-center"
        >
          <i class="pi pi-book text-4xl text-muted/60" />
          <p class="text-sm text-muted">{{ t("skill.emptyState") }}</p>
          <Button icon="pi pi-plus" :label="t('skill.newSkill')" @click="openCreateDialog" />
        </div>

        <!-- Grid view -->
        <div v-else-if="viewMode === 'grid'" class="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3">
          <div v-for="skill in ctrl.filteredSkills.value" :key="skill.id" class="flex flex-col gap-2 rounded-lg border border-divider bg-canvas p-4 shadow-sm">
            <div class="flex items-start gap-2">
              <i :class="[skill.icon, 'text-xl text-brand']" />
              <div class="min-w-0 flex-1">
                <h3 class="section-title truncate">{{ skill.name }}</h3>
                <div class="mt-1 flex flex-wrap items-center gap-1">
                  <span :class="['inline-block rounded-full px-2 py-0.5 text-[10px] font-bold', categoryBadgeClass(skill.category)]">
                    {{ categoryLabel(skill.category) }}
                  </span>
                  <span v-if="skill.stack" class="inline-block rounded-full bg-panel px-2 py-0.5 text-[10px] text-secondary border border-divider">
                    {{ skill.stack }}
                  </span>
                </div>
              </div>
              <div class="flex shrink-0 items-center gap-0.5">
                <Button icon="pi pi-copy" text rounded size="small" :title="t('skill.copyInstructions')" @click="copyInstructions(skill)" />
                <Button icon="pi pi-pencil" text rounded size="small" :title="t('common.edit')" @click="openEditDialog(skill)" />
                <Button icon="pi pi-trash" text rounded size="small" severity="danger" :title="t('common.delete')" @click="confirmDelete(skill)" />
              </div>
            </div>
            <p class="line-clamp-2 text-xs text-muted">{{ skill.description || t("skill.noDescription") }}</p>
            <div v-if="skill.tags.length" class="flex flex-wrap gap-1">
              <span v-for="tag in skill.tags" :key="tag" class="rounded-full bg-panel px-2 py-0.5 text-[10px] text-secondary">{{ tag }}</span>
            </div>
          </div>
        </div>

        <!-- List view -->
        <div v-else class="flex flex-col divide-y divide-divider rounded-lg border border-divider bg-canvas">
          <div v-for="skill in ctrl.filteredSkills.value" :key="skill.id" class="flex items-center gap-3 px-4 py-3">
            <i :class="[skill.icon, 'shrink-0 text-lg text-brand']" />
            <div class="min-w-0 flex-1">
              <div class="flex items-center gap-2">
                <h3 class="section-title truncate">{{ skill.name }}</h3>
                <span :class="['shrink-0 rounded-full px-2 py-0.5 text-[10px] font-bold', categoryBadgeClass(skill.category)]">
                  {{ categoryLabel(skill.category) }}
                </span>
                <span v-if="skill.stack" class="shrink-0 rounded-full bg-panel px-2 py-0.5 text-[10px] text-secondary border border-divider">
                  {{ skill.stack }}
                </span>
              </div>
              <p class="truncate text-xs text-muted">{{ skill.description || t("skill.noDescription") }}</p>
            </div>
            <div v-if="skill.tags.length" class="hidden shrink-0 items-center gap-1 sm:flex">
              <span v-for="tag in skill.tags" :key="tag" class="rounded-full bg-panel px-2 py-0.5 text-[10px] text-secondary">{{ tag }}</span>
            </div>
            <div class="flex shrink-0 items-center gap-0.5">
              <Button icon="pi pi-copy" text rounded size="small" :title="t('skill.copyInstructions')" @click="copyInstructions(skill)" />
              <Button icon="pi pi-pencil" text rounded size="small" :title="t('common.edit')" @click="openEditDialog(skill)" />
              <Button icon="pi pi-trash" text rounded size="small" severity="danger" :title="t('common.delete')" @click="confirmDelete(skill)" />
            </div>
          </div>
        </div>
      </div>
    </div>

    <SkillFormDialog
      :visible="showDialog"
      :skill="editingSkill"
      @update:visible="showDialog = $event"
      @save="saveSkill"
    />

    <SkillDeleteDialog
      :visible="showDeleteDialog"
      :skill="deleteTarget"
      @update:visible="showDeleteDialog = $event"
      @confirm="executeDelete"
    />
  </section>
</template>
