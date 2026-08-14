<script setup lang="ts">
import { useI18n } from "vue-i18n";
import Dialog from "primevue/dialog";
import InputText from "primevue/inputtext";
import Select from "primevue/select";
import Checkbox from "primevue/checkbox";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import { useToast } from "@/shared/composables/useToast";
import { TASK_CATEGORY_OPTIONS } from "@/models/task";
import type { TasksApi } from "../composables/useTasks";

const props = defineProps<{ ctrl: TasksApi }>();
const visible = defineModel<boolean>("visible", { default: false });

const { t } = useI18n();
const toast = useToast();

async function saveAndClose() {
  if (await props.ctrl.saveDraft()) {
    toast.success(props.ctrl.isCreating.value ? t("aiTasks.toast.created") : t("aiTasks.toast.updated"));
    visible.value = false;
  }
}
</script>

<template>
  <Dialog
    :visible="visible"
    class="w-full max-w-lg rounded-lg bg-panel shadow-xl"
    :closable="true"
    modal
    @update:visible="visible = $event"
  >
    <template #header>
      <div>
        <h3 class="section-title">{{ ctrl.isCreating.value ? t("aiTasks.dialog.addTitle") : t("aiTasks.dialog.editTitle") }}</h3>
        <p v-if="ctrl.draft.value && !ctrl.isCreating.value" class="mt-1 text-sm text-muted">
          {{ t("aiTasks.dialog.idLabel", { id: ctrl.draft.value.id }) }}
        </p>
      </div>
    </template>

    <div v-if="ctrl.draft.value" class="space-y-4">
      <label class="block">
        <span class="text-xs font-bold text-muted">{{ t("aiTasks.form.taskCd") }} <span class="text-red-500">*</span></span>
        <InputText
          class="mt-1 w-full"
          :model-value="ctrl.draft.value.task_cd"
          :placeholder="t('aiTasks.form.taskCdPlaceholder')"
          autofocus
          @update:model-value="(v) => (ctrl.draft.value!.task_cd = String(v))"
        />
      </label>

      <label class="block">
        <span class="text-xs font-bold text-muted">{{ t("aiTasks.form.taskName") }}</span>
        <InputText
          class="mt-1 w-full"
          :model-value="ctrl.draft.value.task_name"
          :placeholder="t('aiTasks.form.taskNamePlaceholder')"
          @update:model-value="(v) => (ctrl.draft.value!.task_name = String(v))"
        />
      </label>

      <label class="block">
        <span class="text-xs font-bold text-muted">{{ t("aiTasks.form.category") }}</span>
        <Select
          class="mt-1 w-full"
          :model-value="ctrl.draft.value.category_id"
          :options="TASK_CATEGORY_OPTIONS"
          option-label="label"
          option-value="value"
          @update:model-value="(v) => (ctrl.draft.value!.category_id = v)"
        />
      </label>

      <label v-if="!ctrl.isCreating.value" class="flex items-start gap-2">
        <Checkbox
          binary
          input-id="task-is-complete"
          class="mt-0.5"
          :model-value="ctrl.draft.value.is_complete"
          @update:model-value="(v) => (ctrl.draft.value!.is_complete = !!v)"
        />
        <span class="text-sm text-ink">{{ t("aiTasks.form.markComplete") }}</span>
      </label>
    </div>

    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="ctrl.isCreating.value ? t('aiTasks.actions.create') : t('aiTasks.actions.save')"
        :confirm-icon="ctrl.isCreating.value ? 'pi pi-plus' : 'pi pi-save'"
        @cancel="visible = false"
        @confirm="saveAndClose"
      />
    </template>
  </Dialog>
</template>
