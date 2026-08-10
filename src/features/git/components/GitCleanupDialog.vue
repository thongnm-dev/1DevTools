<script setup lang="ts">
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import Checkbox from "primevue/checkbox";
import Dialog from "primevue/dialog";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { GitApi } from "../composables/useGit";

const { t } = useI18n();

const props = defineProps<{ git: GitApi }>();
const visible = defineModel<boolean>("visible", { default: false });

const cleanupList = ref<string[]>([]);
const cleanupSelected = ref<Set<string>>(new Set());
const cleanupScanning = ref(false);

watch(visible, async (v) => {
  if (!v) return;
  cleanupScanning.value = true;
  cleanupList.value = [];
  cleanupSelected.value = new Set();
  cleanupList.value = await props.git.cleanupScan();
  cleanupSelected.value = new Set(cleanupList.value);
  cleanupScanning.value = false;
});

function toggleCleanup(name: string) {
  const s = new Set(cleanupSelected.value);
  if (s.has(name)) s.delete(name);
  else s.add(name);
  cleanupSelected.value = s;
}

async function doCleanup() {
  await props.git.cleanupDelete([...cleanupSelected.value]);
  visible.value = false;
}
</script>

<template>
  <Dialog v-model:visible="visible" modal :header="t('git.dialogs.cleanup.title')" :style="{ width: '520px' }">
    <div class="flex flex-col gap-2">
      <p class="text-xs text-muted">
        {{ t('git.dialogs.cleanup.description1') }} <strong>fetch --prune</strong>{{ t('git.dialogs.cleanup.description2') }}
      </p>
      <div v-if="cleanupScanning" class="p-6 text-center text-sm text-muted">
        <i class="pi pi-spinner pi-spin mr-1.5" /> {{ t('git.dialogs.cleanup.scanning') }}
      </div>
      <div v-else-if="!cleanupList.length" class="p-6 text-center text-sm text-muted">
        {{ t('git.dialogs.cleanup.noBranches') }}
      </div>
      <div v-else class="max-h-64 overflow-y-auto rounded-md border border-divider">
        <label
          v-for="b in cleanupList"
          :key="b"
          class="flex cursor-pointer items-center gap-2 border-b border-divider-light px-2.5 py-1.5 last:border-0 hover:bg-canvas"
        >
          <Checkbox :model-value="cleanupSelected.has(b)" binary @change="toggleCleanup(b)" />
          <i class="pi pi-sitemap text-xs text-muted" />
          <span class="min-w-0 flex-1 truncate text-sm text-ink">{{ b }}</span>
          <span class="shrink-0 badge-danger">gone</span>
        </label>
      </div>
    </div>
    <template #footer>
      <DialogFooter
        :cancel-label="t('git.dialogs.cleanup.close')"
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="t('git.dialogs.cleanup.confirm', { count: cleanupSelected.size })"
        confirm-icon="pi pi-trash"
        confirm-severity="danger"
        :confirm-disabled="!cleanupSelected.size"
        @cancel="visible = false"
        @confirm="doCleanup"
      />
    </template>
  </Dialog>
</template>
