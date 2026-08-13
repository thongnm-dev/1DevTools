<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import Dialog from "primevue/dialog";
import InputText from "primevue/inputtext";
import Select from "primevue/select";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { CommandKind, DevCommand } from "@/models/dev_runner";

const props = defineProps<{ editing: DevCommand | null }>();
const visible = defineModel<boolean>("visible", { default: false });
const emit = defineEmits<{ save: [cmd: DevCommand] }>();

const { t } = useI18n();

const label = ref("");
const command = ref("");
const kind = ref<CommandKind>("run");

const kindOptions = computed(() => [
  { label: t("git.runner.kind.run"), value: "run" as CommandKind },
  { label: t("git.runner.kind.test"), value: "test" as CommandKind },
  { label: t("git.runner.kind.build"), value: "build" as CommandKind },
]);

watch(visible, (v) => {
  if (!v) return;
  if (props.editing) {
    label.value = props.editing.label;
    command.value = props.editing.command;
    kind.value = props.editing.kind;
  } else {
    label.value = "";
    command.value = "";
    kind.value = "run";
  }
});

const canSave = computed(() => label.value.trim().length > 0 && command.value.trim().length > 0);

function doSave() {
  if (!canSave.value) return;
  const trimLabel = label.value.trim();
  const trimCmd = command.value.trim();
  emit("save", {
    id: props.editing?.id ?? `custom:${Date.now()}`,
    label: trimLabel,
    command: trimCmd,
    category: "custom",
    source: "custom",
    source_file: "",
    kind: kind.value,
  });
  visible.value = false;
}
</script>

<template>
  <Dialog
    v-model:visible="visible"
    modal
    :header="editing ? t('git.runner.editDialogTitle') : t('git.runner.addDialogTitle')"
    :style="{ width: '420px' }"
  >
    <div class="flex flex-col gap-3">
      <div>
        <label class="mb-1 block text-xs font-bold text-muted">{{ t("git.runner.labelField") }}</label>
        <InputText v-model="label" class="w-full" :placeholder="t('git.runner.labelPlaceholder')" />
      </div>
      <div>
        <label class="mb-1 block text-xs font-bold text-muted">{{ t("git.runner.commandField") }}</label>
        <InputText
          v-model="command"
          class="w-full font-mono"
          :placeholder="t('git.runner.commandPlaceholder')"
          @keydown.enter="doSave"
        />
      </div>
      <div>
        <label class="mb-1 block text-xs font-bold text-muted">{{ t("git.runner.kindField") }}</label>
        <Select v-model="kind" :options="kindOptions" optionLabel="label" optionValue="value" class="w-full" />
      </div>
    </div>
    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="t('common.save')"
        confirm-icon="pi pi-check"
        :confirm-disabled="!canSave"
        @cancel="visible = false"
        @confirm="doSave"
      />
    </template>
  </Dialog>
</template>
