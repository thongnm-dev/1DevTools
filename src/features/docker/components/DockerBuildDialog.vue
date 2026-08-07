<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { open } from "@tauri-apps/plugin-dialog";
import Dialog from "primevue/dialog";
import InputText from "primevue/inputtext";
import InputGroup from "primevue/inputgroup";
import Button from "primevue/button";
import Checkbox from "primevue/checkbox";
import DialogFooter from "@/shared/components/DialogFooter.vue";

export interface AdHocBuildPayload {
  contextPath: string;
  dockerfilePath: string;
  tag: string;
  saveName: string | null;
  noCache: boolean;
}

const visible = defineModel<boolean>("visible", { default: false });
const emit = defineEmits<{ build: [payload: AdHocBuildPayload] }>();

const { t } = useI18n();

const contextPath = ref("");
const dockerfilePath = ref("");
const tag = ref("");
const saveAsProject = ref(false);
const projectName = ref("");
const noCache = ref(false);

watch(visible, (v) => {
  if (!v) return;
  contextPath.value = "";
  dockerfilePath.value = "";
  tag.value = "";
  saveAsProject.value = false;
  projectName.value = "";
  noCache.value = false;
});

async function pickContextDir() {
  const picked = await open({ directory: true, title: t("docker.buildDialog.pickContextTitle") });
  if (picked && typeof picked === "string") contextPath.value = picked;
}

const canBuild = computed(() => {
  if (!contextPath.value.trim()) return false;
  if (saveAsProject.value && !projectName.value.trim()) return false;
  return true;
});

function doBuild() {
  if (!canBuild.value) return;
  emit("build", {
    contextPath: contextPath.value.trim(),
    dockerfilePath: dockerfilePath.value.trim(),
    tag: tag.value.trim(),
    saveName: saveAsProject.value ? projectName.value.trim() : null,
    noCache: noCache.value,
  });
  visible.value = false;
}
</script>

<template>
  <Dialog v-model:visible="visible" modal :header="t('docker.buildDialog.title')" :style="{ width: '460px' }">
    <div class="flex flex-col gap-3">
      <div>
        <label class="mb-1 block text-xs font-bold text-muted">{{ t("docker.buildDialog.contextLabel") }}</label>
        <InputGroup class="h-8">
          <InputText :model-value="contextPath" readonly :placeholder="t('docker.buildDialog.contextPlaceholder')" />
          <Button icon="pi pi-folder-open" severity="secondary" outlined @click="pickContextDir" />
        </InputGroup>
      </div>
      <div>
        <label class="mb-1 block text-xs font-bold text-muted">{{ t("docker.buildDialog.dockerfileLabel") }}</label>
        <InputText v-model="dockerfilePath" class="w-full" placeholder="Dockerfile" />
      </div>
      <div>
        <label class="mb-1 block text-xs font-bold text-muted">{{ t("docker.buildDialog.tagLabel") }}</label>
        <InputText v-model="tag" class="w-full" placeholder="myapp:latest" />
      </div>
      <div class="flex items-center gap-2">
        <Checkbox v-model="noCache" binary input-id="docker-no-cache" />
        <label for="docker-no-cache" class="text-xs text-secondary">{{ t("docker.buildDialog.cleanBuild") }}</label>
      </div>
      <div class="flex items-center gap-2">
        <Checkbox v-model="saveAsProject" binary input-id="docker-save-as-project" />
        <label for="docker-save-as-project" class="text-xs text-secondary">{{ t("docker.buildDialog.saveAsProject") }}</label>
      </div>
      <InputText
        v-if="saveAsProject"
        v-model="projectName"
        class="w-full"
        :placeholder="t('docker.projectDialog.namePlaceholder')"
      />
    </div>
    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="t('docker.buildDialog.confirm')"
        confirm-icon="pi pi-hammer"
        :confirm-disabled="!canBuild"
        @cancel="visible = false"
        @confirm="doBuild"
      />
    </template>
  </Dialog>
</template>
