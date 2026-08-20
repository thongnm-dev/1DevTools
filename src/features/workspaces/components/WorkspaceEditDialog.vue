<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import Button from "primevue/button";
import Dialog from "primevue/dialog";
import InputGroup from "primevue/inputgroup";
import InputText from "primevue/inputtext";
import Select from "primevue/select";
import IconPickerDialog from "@/shared/components/IconPickerDialog.vue";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { WorkspaceApi } from "../composables/useWorkspace";
import type { GitRepo } from "@/models/git";
import type { Workspace } from "@/models/workspace";
import { DEFAULT_WORKSPACE_ICON } from "@/models/workspace";

const props = defineProps<{ workspaceCtrl: WorkspaceApi; editingWorkspace: Workspace | null }>();
const visible = defineModel<boolean>("visible", { default: false });

const { t } = useI18n();

const selectedRepoId = ref<number | null>(null);
const wsName = ref("");
const wsIcon = ref(DEFAULT_WORKSPACE_ICON);
const showIconPicker = ref(false);

const isEditing = computed(() => props.editingWorkspace !== null);

watch(visible, (v) => {
  if (!v) return;
  const ws = props.editingWorkspace;
  selectedRepoId.value = null;
  wsName.value = ws?.name ?? "";
  wsIcon.value = ws?.icon ?? DEFAULT_WORKSPACE_ICON;
});

const repoOptions = computed(() => props.workspaceCtrl.gitRepos.value.map((r) => ({ label: r.name, value: r.id })));

function onRepoSelected() {
  const repo = props.workspaceCtrl.gitRepos.value.find((r) => r.id === selectedRepoId.value);
  if (repo) wsName.value = repo.name;
}

async function browseForFolder() {
  const repo = await props.workspaceCtrl.pickFolder();
  if (!repo) return;
  selectedRepoId.value = repo.id;
  wsName.value = repo.name;
}

async function saveWorkspace() {
  const name = wsName.value.trim();
  if (!name) return;
  if (props.editingWorkspace !== null) {
    await props.workspaceCtrl.updateWorkspace(props.editingWorkspace.id, { name, icon: wsIcon.value });
  } else {
    const repo = props.workspaceCtrl.gitRepos.value.find((r: GitRepo) => r.id === selectedRepoId.value);
    if (!repo) return;
    await props.workspaceCtrl.createFromRepo(repo, name, wsIcon.value);
  }
  visible.value = false;
}

const selectPt = {
  root: { class: "!bg-panel !border-divider" },
  label: { class: "!flex !items-center !text-xs !py-1.5 !text-ink" },
  option: { class: "!text-xs" },
};
</script>

<template>
  <Dialog
    :visible="visible"
    class="w-full max-w-md rounded-lg bg-panel shadow-xl"
    :closable="true"
    modal
    @update:visible="visible = $event"
  >
    <template #header>
      <h3 class="section-title">{{ isEditing ? t("workspaces.dialog.editTitle") : t("workspaces.dialog.newTitle") }}</h3>
    </template>

    <div class="space-y-4">
      <div v-if="!isEditing" class="block">
        <span class="text-xs font-bold text-muted">{{ t("workspaces.dialog.project") }} <span class="text-red-500">*</span></span>
        <InputGroup class="mt-1">
          <Select
            v-model="selectedRepoId"
            :options="repoOptions"
            optionLabel="label"
            optionValue="value"
            :placeholder="t('workspaces.dialog.projectPlaceholder')"
            class="min-w-0 flex-1"
            :pt="selectPt"
            @change="onRepoSelected"
          />
          <Button icon="pi pi-folder-open" severity="secondary" outlined :title="t('workspaces.dialog.browse')" @click="browseForFolder" />
        </InputGroup>
      </div>

      <div class="flex items-end gap-3">
        <label class="block min-w-0 flex-1">
          <span class="text-xs font-bold text-muted">{{ t("workspaces.dialog.name") }} <span class="text-red-500">*</span></span>
          <InputText v-model="wsName" class="mt-1 w-full" :placeholder="t('workspaces.dialog.namePlaceholder')" autofocus />
        </label>
        <div class="block">
          <span class="text-xs font-bold text-muted">{{ t("workspaces.dialog.icon") }}</span>
          <div class="mt-1 flex items-center gap-2">
            <div class="flex h-10 items-center gap-2 rounded-md border border-divider bg-panel px-3">
              <i :class="[wsIcon, 'text-brand']" />
              <InputText
                v-model="wsIcon"
                class="embedded-input w-24 border-0 !bg-transparent !p-0 !text-sm"
                placeholder="pi pi-folder"
              />
            </div>
            <Button icon="pi pi-th-large" severity="secondary" outlined :title="t('workspaces.dialog.browseIcons')" @click="showIconPicker = true" />
          </div>
        </div>
      </div>

    </div>

    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="isEditing ? t('common.save') : t('common.create')"
        :confirm-icon="isEditing ? 'pi pi-check' : 'pi pi-plus'"
        :confirm-disabled="!wsName.trim() || (!isEditing && selectedRepoId === null)"
        @cancel="visible = false"
        @confirm="saveWorkspace"
      />
    </template>
  </Dialog>

  <IconPickerDialog
    :visible="showIconPicker"
    :selected="wsIcon"
    @update:visible="showIconPicker = $event"
    @select="(icon: string) => (wsIcon = 'pi ' + icon)"
  />
</template>
