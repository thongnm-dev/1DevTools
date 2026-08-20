<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import Button from "primevue/button";
import InputText from "primevue/inputtext";
import Fieldset from "primevue/fieldset";
import RuleFormDialog from "./RuleFormDialog.vue";
import RuleDeleteDialog from "./RuleDeleteDialog.vue";
import MarkdownPreviewDialog from "@/shared/components/MarkdownPreviewDialog.vue";
import { useRule } from "../composables/useRule";
import { useToast } from "@/shared/composables/useToast";
import { friendlyError } from "@/tauri/commands/_base";
import { ruleExport } from "@/tauri/commands/rule";
import type { Rule, RuleRequest } from "@/models/rule";

type ViewMode = "grid" | "list";
const viewMode = ref<ViewMode>("grid");

const { t } = useI18n();
const toast = useToast();
const ctrl = useRule();

// --- Add/Edit dialog ---
const showDialog = ref(false);
const editingRule = ref<Rule | null>(null);

function openCreateDialog() {
  editingRule.value = null;
  showDialog.value = true;
}

function openEditDialog(rule: Rule) {
  editingRule.value = rule;
  showDialog.value = true;
}

async function saveRule(request: RuleRequest) {
  if (editingRule.value !== null) {
    await ctrl.updateRule(editingRule.value.id, request);
  } else {
    await ctrl.createRule(request);
  }
  showDialog.value = false;
}

async function copyContent(rule: Rule) {
  try {
    await navigator.clipboard.writeText(rule.content);
    toast.success(t("rule.toast.copied"));
  } catch (e) {
    toast.error(friendlyError(e));
  }
}

// --- Export to markdown + preview ---
const showMarkdownPreview = ref(false);
const markdownPreviewPath = ref("");

async function previewRule(rule: Rule) {
  try {
    markdownPreviewPath.value = await ruleExport(rule.id);
    showMarkdownPreview.value = true;
  } catch (e) {
    toast.error(friendlyError(e));
  }
}

// --- Delete confirmation ---
const showDeleteDialog = ref(false);
const deleteTarget = ref<Rule | null>(null);

function confirmDelete(rule: Rule) {
  deleteTarget.value = rule;
  showDeleteDialog.value = true;
}

async function executeDelete() {
  if (!deleteTarget.value) return;
  await ctrl.deleteRule(deleteTarget.value.id);
  showDeleteDialog.value = false;
  deleteTarget.value = null;
}
</script>

<template>
  <section class="flex min-h-0 flex-1 flex-col gap-4 overflow-hidden">

    <!-- Action bar -->
    <section class="flex items-center justify-end rounded-lg border border-divider bg-panel p-4 shadow-sm">
      <Button icon="pi pi-plus" :label="t('rule.newRule')" size="small" @click="openCreateDialog" />
    </section>

    <!-- Search fieldset -->
    <Fieldset class="rounded-lg border border-divider bg-panel p-4 shadow-md fieldset-nested" :legend="t('common.searchLegend')" toggleable>
      <div class="grid gap-3">
        <label>
          <span class="text-xs font-bold text-muted">{{ t("rule.search.keyword") }}</span>
          <InputText
            class="mt-1 w-full"
            :placeholder="t('rule.search.keywordPlaceholder')"
            :model-value="ctrl.filters.value.keyword"
            @update:model-value="ctrl.filters.value = { ...ctrl.filters.value, keyword: ($event as string) ?? '' }"
            @keyup.enter="ctrl.search()"
          />
        </label>
        <div class="flex items-center justify-end gap-2">
          <Button icon="pi pi-refresh" :label="t('common.reset')" severity="secondary" outlined size="small" @click="ctrl.resetFilters()" />
          <Button icon="pi pi-search" :label="t('common.search')" size="small" @click="ctrl.search()" />
        </div>
      </div>
    </Fieldset>

    <!-- Rules panel -->
    <div class="flex min-h-0 flex-1 flex-col overflow-hidden rounded-lg border border-divider bg-panel shadow-sm">

      <!-- Panel header -->
      <div class="flex shrink-0 items-center gap-2 border-b border-divider px-3 py-2">
        <span class="flex-1 text-xs font-semibold text-default">
          {{ t('rule.title') }}
          <span class="ml-1 text-muted">{{ t('rule.table.count', { count: ctrl.filteredRules.value.length }) }}</span>
        </span>

        <!-- Sort controls -->
        <div class="flex items-center rounded-md border border-divider overflow-hidden">
          <select
            v-model="ctrl.sortBy.value"
            class="border-0 bg-panel px-2 py-1 text-xs text-default focus:outline-none"
          >
            <option value="name">{{ t('rule.sortName') }}</option>
            <option value="created_at">{{ t('rule.sortDate') }}</option>
          </select>
          <button
            class="border-l border-divider bg-panel px-2 py-1 text-muted transition-colors hover:text-default"
            :title="ctrl.sortDir.value === 'asc' ? t('rule.sortAsc') : t('rule.sortDesc')"
            @click="ctrl.sortDir.value = ctrl.sortDir.value === 'asc' ? 'desc' : 'asc'"
          >
            <i :class="['pi text-xs', ctrl.sortDir.value === 'asc' ? 'pi-sort-amount-up' : 'pi-sort-amount-down']" />
          </button>
        </div>

        <!-- View toggle -->
        <div class="flex rounded-md border border-divider overflow-hidden">
          <button
            :class="['px-2 py-1 text-xs transition-colors', viewMode === 'grid' ? 'bg-brand text-white' : 'bg-panel text-muted hover:text-default']"
            :title="t('rule.viewGrid')"
            @click="viewMode = 'grid'"
          >
            <i class="pi pi-th-large text-xs" />
          </button>
          <button
            :class="['px-2 py-1 text-xs transition-colors border-l border-divider', viewMode === 'list' ? 'bg-brand text-white' : 'bg-panel text-muted hover:text-default']"
            :title="t('rule.viewList')"
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
          v-else-if="ctrl.filteredRules.value.length === 0"
          class="flex h-full flex-col items-center justify-center gap-3 rounded-lg border border-dashed border-divider bg-canvas/50 p-12 text-center"
        >
          <i class="pi pi-file-edit text-4xl text-muted/60" />
          <p class="text-sm text-muted">{{ t("rule.emptyState") }}</p>
          <Button icon="pi pi-plus" :label="t('rule.newRule')" @click="openCreateDialog" />
        </div>

        <!-- Grid view -->
        <div v-else-if="viewMode === 'grid'" class="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3">
          <div v-for="rule in ctrl.filteredRules.value" :key="rule.id" class="flex flex-col gap-2 rounded-lg border border-divider bg-canvas p-4 shadow-sm">
            <div class="flex items-start gap-2">
              <i class="pi pi-file-edit text-xl text-brand" />
              <div class="min-w-0 flex-1">
                <h3 class="section-title truncate">{{ rule.name }}</h3>
              </div>
              <div class="flex shrink-0 items-center gap-0.5">
                <Button icon="pi pi-copy" text rounded size="small" :title="t('rule.copyContent')" @click="copyContent(rule)" />
                <Button icon="pi pi-eye" text rounded size="small" :title="t('rule.previewMarkdown')" @click="previewRule(rule)" />
                <Button icon="pi pi-pencil" text rounded size="small" :title="t('common.edit')" @click="openEditDialog(rule)" />
                <Button icon="pi pi-trash" text rounded size="small" severity="danger" :title="t('common.delete')" @click="confirmDelete(rule)" />
              </div>
            </div>
            <p class="line-clamp-2 text-xs text-muted">{{ rule.description || t("rule.noDescription") }}</p>
            <div v-if="rule.tags.length" class="flex flex-wrap gap-1">
              <span v-for="tag in rule.tags" :key="tag" class="rounded-full bg-panel px-2 py-0.5 text-[10px] text-secondary">{{ tag }}</span>
            </div>
          </div>
        </div>

        <!-- List view -->
        <div v-else class="flex flex-col divide-y divide-divider rounded-lg border border-divider bg-canvas">
          <div v-for="rule in ctrl.filteredRules.value" :key="rule.id" class="flex items-center gap-3 px-4 py-3">
            <i class="pi pi-file-edit shrink-0 text-lg text-brand" />
            <div class="min-w-0 flex-1">
              <h3 class="section-title truncate">{{ rule.name }}</h3>
              <p class="truncate text-xs text-muted">{{ rule.description || t("rule.noDescription") }}</p>
            </div>
            <div v-if="rule.tags.length" class="hidden shrink-0 items-center gap-1 sm:flex">
              <span v-for="tag in rule.tags" :key="tag" class="rounded-full bg-panel px-2 py-0.5 text-[10px] text-secondary">{{ tag }}</span>
            </div>
            <div class="flex shrink-0 items-center gap-0.5">
              <Button icon="pi pi-copy" text rounded size="small" :title="t('rule.copyContent')" @click="copyContent(rule)" />
              <Button icon="pi pi-eye" text rounded size="small" :title="t('rule.previewMarkdown')" @click="previewRule(rule)" />
              <Button icon="pi pi-pencil" text rounded size="small" :title="t('common.edit')" @click="openEditDialog(rule)" />
              <Button icon="pi pi-trash" text rounded size="small" severity="danger" :title="t('common.delete')" @click="confirmDelete(rule)" />
            </div>
          </div>
        </div>
      </div>
    </div>

    <RuleFormDialog
      :visible="showDialog"
      :rule="editingRule"
      @update:visible="showDialog = $event"
      @save="saveRule"
    />

    <RuleDeleteDialog
      :visible="showDeleteDialog"
      :rule="deleteTarget"
      @update:visible="showDeleteDialog = $event"
      @confirm="executeDelete"
    />

    <MarkdownPreviewDialog
      v-model:visible="showMarkdownPreview"
      :file-path="markdownPreviewPath"
    />
  </section>
</template>
