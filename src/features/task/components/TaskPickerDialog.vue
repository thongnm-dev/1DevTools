<script setup lang="ts">
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import Button from "primevue/button";
import InputText from "primevue/inputtext";
import Dialog from "primevue/dialog";
import DataTable from "primevue/datatable";
import Column from "primevue/column";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import TaskDialog from "./TaskDialog.vue";
import { useTasks } from "../composables/useTasks";
import type { Task } from "@/models/task";

const props = withDefaults(defineProps<{ excludeIds?: number[] }>(), { excludeIds: () => [] });
const visible = defineModel<boolean>("visible", { default: false });
const emit = defineEmits<{ pick: [task: Task] }>();

const { t } = useI18n();
const pickerCtrl = useTasks();
const showNewTaskDialog = ref(false);

watch(visible, (v) => {
  if (!v) return;
  pickerCtrl.filters.value = { keyword: "", isComplete: false };
  void pickerCtrl.fetchTasks();
});

function pickTask(id: number) {
  const task = pickerCtrl.tasks.value.find((t) => t.id === id);
  if (task) emit("pick", task);
}

function openCreateTask() {
  pickerCtrl.startCreate();
  showNewTaskDialog.value = true;
}

async function afterCreateTask() {
  showNewTaskDialog.value = false;
  pickerCtrl.filters.value = { keyword: "", isComplete: false };
  await pickerCtrl.fetchTasks();
  const created = pickerCtrl.tasks.value[0];
  if (created) emit("pick", created);
}
</script>

<template>
  <Dialog
    :visible="visible"
    class="w-full max-w-2xl rounded-lg bg-panel shadow-xl"
    :closable="true"
    modal
    @update:visible="visible = $event"
  >
    <template #header>
      <h3 class="section-title">{{ t("aiCowork.picker.title") }}</h3>
    </template>
    <div class="space-y-3">
      <div class="flex gap-2">
        <InputText
          class="flex-1"
          :placeholder="t('aiCowork.picker.searchPlaceholder')"
          :model-value="pickerCtrl.filters.value.keyword"
          @update:model-value="pickerCtrl.filters.value.keyword = String($event)"
          @keyup.enter="pickerCtrl.search()"
        />
        <Button icon="pi pi-search" severity="secondary" @click="pickerCtrl.search()" />
        <Button icon="pi pi-plus" :label="t('aiCowork.picker.newTask')" @click="openCreateTask" />
      </div>
      <DataTable
        :value="pickerCtrl.filteredTasks.value.filter((task) => !props.excludeIds.includes(task.id))"
        scrollable
        scroll-height="360px"
        :table-style="{ minWidth: '480px' }"
      >
        <Column field="task_cd" :header="t('aiTasks.table.taskCd')" />
        <Column field="task_name" :header="t('aiTasks.table.taskName')" />
        <Column field="category_id" :header="t('aiTasks.table.category')" />
        <Column :header="t('aiCowork.picker.add')" body-class="text-center" :style="{ width: '80px' }">
          <template #body="{ data }">
            <Button icon="pi pi-plus" text rounded size="small" @click="pickTask(data.id)" />
          </template>
        </Column>
      </DataTable>
    </div>
    <template #footer>
      <DialogFooter hide-confirm :cancel-label="t('common.close')" @cancel="visible = false" />
    </template>
  </Dialog>

  <TaskDialog v-model:visible="showNewTaskDialog" :ctrl="pickerCtrl" @update:visible="(v) => !v && afterCreateTask()" />
</template>
