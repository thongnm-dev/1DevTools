<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { open } from "@tauri-apps/plugin-dialog";
import Dialog from "primevue/dialog";
import InputText from "primevue/inputtext";
import InputGroup from "primevue/inputgroup";
import Button from "primevue/button";
import Select from "primevue/select";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { DockerApi } from "../composables/useDocker";
import type { DockerProject, DockerProjectKind } from "@/models/docker";

const props = defineProps<{
  docker: DockerApi;
  editing: DockerProject | null;
}>();
const visible = defineModel<boolean>("visible", { default: false });

const { t } = useI18n();

const KIND_OPTIONS = computed<{ label: string; value: DockerProjectKind }[]>(() => [
  { label: t("docker.projectDialog.kindDockerfile"), value: "dockerfile" },
  { label: t("docker.projectDialog.kindCompose"), value: "compose" },
]);

const name = ref("");
const kind = ref<DockerProjectKind>("dockerfile");
const contextPath = ref("");
const dockerfilePath = ref("");
const imageTag = ref("");
const composeFile = ref("");
const saving = ref(false);

watch(visible, (v) => {
  if (!v) return;
  const editing = props.editing;
  name.value = editing?.name ?? "";
  kind.value = editing?.kind ?? "dockerfile";
  contextPath.value = editing?.context_path ?? "";
  dockerfilePath.value = editing?.dockerfile_path ?? "";
  imageTag.value = editing?.image_tag ?? "";
  composeFile.value = editing?.compose_file ?? "";
});

async function pickContextDir() {
  const picked = await open({ directory: true, title: t("docker.projectDialog.pickContextTitle") });
  if (picked && typeof picked === "string") contextPath.value = picked;
}

async function pickComposeFile() {
  const picked = await open({
    directory: false,
    title: t("docker.projectDialog.pickComposeTitle"),
    filters: [{ name: "docker-compose", extensions: ["yml", "yaml"] }],
  });
  if (picked && typeof picked === "string") composeFile.value = picked;
}

const canSave = computed(() => {
  if (!name.value.trim()) return false;
  return kind.value === "dockerfile" ? !!contextPath.value.trim() : !!composeFile.value.trim();
});

async function doSave() {
  if (!canSave.value || saving.value) return;
  saving.value = true;
  try {
    const result = props.editing
      ? await props.docker.updateProject(
          props.editing.id,
          name.value.trim(),
          kind.value,
          contextPath.value.trim(),
          dockerfilePath.value.trim(),
          imageTag.value.trim(),
          composeFile.value.trim(),
        )
      : await props.docker.addProject(
          name.value.trim(),
          kind.value,
          contextPath.value.trim(),
          dockerfilePath.value.trim(),
          imageTag.value.trim(),
          composeFile.value.trim(),
        );
    if (result) visible.value = false;
  } finally {
    saving.value = false;
  }
}
</script>

<template>
  <Dialog
    v-model:visible="visible"
    modal
    :header="editing ? t('docker.projectDialog.editTitle') : t('docker.projectDialog.addTitle')"
    :style="{ width: '480px' }"
  >
    <div class="flex flex-col gap-3">
      <div>
        <label class="mb-1 block text-xs font-bold text-muted">{{ t("docker.projectDialog.nameLabel") }}</label>
        <InputText v-model="name" class="w-full" :placeholder="t('docker.projectDialog.namePlaceholder')" />
      </div>
      <div>
        <label class="mb-1 block text-xs font-bold text-muted">{{ t("docker.projectDialog.kindLabel") }}</label>
        <Select v-model="kind" :options="KIND_OPTIONS" option-label="label" option-value="value" class="w-full" />
      </div>

      <template v-if="kind === 'dockerfile'">
        <div>
          <label class="mb-1 block text-xs font-bold text-muted">{{ t("docker.projectDialog.contextLabel") }}</label>
          <InputGroup class="h-8">
            <InputText :model-value="contextPath" readonly :placeholder="t('docker.projectDialog.contextPlaceholder')" />
            <Button icon="pi pi-folder-open" severity="secondary" outlined @click="pickContextDir" />
          </InputGroup>
        </div>
        <div>
          <label class="mb-1 block text-xs font-bold text-muted">{{ t("docker.projectDialog.dockerfileLabel") }}</label>
          <InputText v-model="dockerfilePath" class="w-full" placeholder="Dockerfile" />
        </div>
        <div>
          <label class="mb-1 block text-xs font-bold text-muted">{{ t("docker.projectDialog.tagLabel") }}</label>
          <InputText v-model="imageTag" class="w-full" placeholder="myapp:latest" />
        </div>
      </template>

      <template v-else>
        <div>
          <label class="mb-1 block text-xs font-bold text-muted">{{ t("docker.projectDialog.composeLabel") }}</label>
          <InputGroup class="h-8">
            <InputText :model-value="composeFile" readonly :placeholder="t('docker.projectDialog.composePlaceholder')" />
            <Button icon="pi pi-folder-open" severity="secondary" outlined @click="pickComposeFile" />
          </InputGroup>
        </div>
      </template>
    </div>
    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="t('common.save')"
        confirm-icon="pi pi-save"
        :busy="saving"
        :confirm-disabled="!canSave"
        @cancel="visible = false"
        @confirm="doSave"
      />
    </template>
  </Dialog>
</template>
