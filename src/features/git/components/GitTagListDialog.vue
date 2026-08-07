<script setup lang="ts">
import { watch } from "vue";
import { useI18n } from "vue-i18n";
import Button from "primevue/button";
import Column from "primevue/column";
import DataTable from "primevue/datatable";
import Dialog from "primevue/dialog";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { GitApi } from "../composables/useGit";
import { useDataTablePagination } from "@/shared/composables/useDataTablePagination";

const { t } = useI18n();
const props = defineProps<{ git: GitApi }>();
const { paginationCompact } = useDataTablePagination();
const visible = defineModel<boolean>("visible", { default: false });

const emit = defineEmits<{ "create-tag": [] }>();

watch(visible, (v) => {
  if (v) props.git.loadTags();
});
</script>

<template>
  <Dialog v-model:visible="visible" modal :header="t('git.dialogs.tagList.title')" :style="{ width: '1000px' }">
    <div v-if="!git.tags.value.length" class="p-6 text-center text-sm text-muted">
      {{ t('common.noData') }}
    </div>
    <DataTable
      v-else
      :value="git.tags.value"
      paginator
      :rows="paginationCompact.rows"
      :rows-per-page-options="paginationCompact.rowsPerPageOptions"
      :paginator-template="paginationCompact.paginatorTemplate"
      :current-page-report-template="paginationCompact.currentPageReportTemplate"
      scrollable
      scroll-height="550px"
      class="app-data-table"
    >
      <Column :header="t('git.dialogs.tagList.columnTag')" field="name" sortable>
        <template #body="{ data }">
          <div class="flex items-center gap-2">
            <i class="pi pi-tag shrink-0 text-xs text-brand" />
            <span class="font-medium text-ink">{{ data.name }}</span>
          </div>
        </template>
      </Column>
      <Column :header="t('git.dialogs.tagList.columnSha')" field="target" style="width: 100px">
        <template #body="{ data }">
          <span class="font-mono text-xs text-muted">{{ data.target }}</span>
        </template>
      </Column>
      <Column :header="t('git.dialogs.tagList.columnMessage')" field="subject">
        <template #body="{ data }">
          <span class="text-sm text-secondary">{{ data.subject || "—" }}</span>
        </template>
      </Column>
      <Column :header="t('git.dialogs.tagList.columnDate')" field="date" sortable style="width: 140px">
        <template #body="{ data }">
          <span class="text-xs text-muted">{{ data.date || "—" }}</span>
        </template>
      </Column>
      <Column header="" style="width: 90px">
        <template #body="{ data }">
          <div class="flex items-center gap-1">
            <Button
              size="small"
              text
              severity="secondary"
              :title="t('git.dialogs.tagList.copyTagName')"
              @click="git.copyText(data.name, t('git.dialogs.tagList.tagNameLabel'))"
            >
              <i class="pi pi-copy" />
            </Button>
            <Button
              size="small"
              text
              severity="danger"
              :title="t('git.dialogs.tagList.deleteTagLocal')"
              @click="git.deleteTag(data.name, false)"
            >
              <i class="pi pi-trash" />
            </Button>
          </div>
        </template>
      </Column>
    </DataTable>
    <template #footer>
      <DialogFooter
        :cancel-label="t('git.dialogs.tagList.close')"
        cancel-icon="pi pi-times"
        cancel-severity="warn"
        :confirm-label="t('git.dialogs.tagList.createTag')"
        confirm-icon="pi pi-plus"
        @cancel="visible = false"
        @confirm="emit('create-tag')"
      />
    </template>
  </Dialog>
</template>
