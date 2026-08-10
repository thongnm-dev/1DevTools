<script setup lang="ts">
import { ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import Checkbox from "primevue/checkbox";
import Dialog from "primevue/dialog";
import InputText from "primevue/inputtext";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { GitApi } from "../composables/useGit";

const { t } = useI18n();

const props = defineProps<{
  git: GitApi;
  target: { hash: string; label: string };
}>();
const visible = defineModel<boolean>("visible", { default: false });

const tagName = ref("");
const tagMessage = ref("");
const tagAnnotated = ref(true);
const tagPush = ref(false);

watch(visible, (v) => {
  if (v) {
    tagName.value = "";
    tagMessage.value = "";
    tagAnnotated.value = true;
    tagPush.value = false;
    props.git.loadTags();
  }
});

async function doCreateTag() {
  const ok = await props.git.createTag(
    tagName.value,
    props.target.hash,
    tagMessage.value,
    tagAnnotated.value,
    tagPush.value,
  );
  if (ok) {
    tagName.value = "";
    tagMessage.value = "";
  }
}
</script>

<template>
  <Dialog v-model:visible="visible" modal :header="t('git.dialogs.tag.title')" :style="{ width: '520px' }">
    <div class="flex flex-col gap-3">
      <p class="text-xs text-muted">{{ t('git.dialogs.tag.createAt') }} <strong class="text-ink">{{ target.label }}</strong></p>
      <div>
        <label class="mb-1 block text-xs font-bold text-muted">{{ t('git.dialogs.tag.nameLabel') }}</label>
        <InputText v-model="tagName" placeholder="v1.0.0" class="w-full" @keydown.enter="doCreateTag" />
      </div>
      <div class="flex items-center gap-2">
        <Checkbox v-model="tagAnnotated" binary input-id="tag-annotated" />
        <label for="tag-annotated" class="text-sm text-ink">{{ t('git.dialogs.tag.annotatedLabel') }}</label>
      </div>
      <div v-if="tagAnnotated">
        <label class="mb-1 block text-xs font-bold text-muted">{{ t('git.dialogs.tag.messageLabel') }}</label>
        <InputText v-model="tagMessage" :placeholder="t('git.dialogs.tag.messagePlaceholder')" class="w-full" />
      </div>
      <div class="flex items-center gap-2">
        <Checkbox v-model="tagPush" binary input-id="tag-push" />
        <label for="tag-push" class="text-sm text-ink">{{ t('git.dialogs.tag.pushLabel') }}</label>
      </div>
      <div v-if="git.tags.value.length" class="mt-1">
        <p class="mb-1 text-xs font-bold uppercase tracking-wide text-muted">{{ t('git.dialogs.tag.existingTags') }}</p>
        <div class="max-h-40 overflow-y-auto rounded-md border border-divider">
          <div
            v-for="tg in git.tags.value"
            :key="tg.name"
            class="flex items-center gap-2 border-b border-divider-light px-2.5 py-1.5 last:border-0"
          >
            <i class="pi pi-tag shrink-0 text-xs text-brand" />
            <span class="min-w-0 flex-1 truncate text-sm text-ink" :title="tg.subject">{{ tg.name }}</span>
            <span class="shrink-0 font-mono text-[11px] text-muted">{{ tg.target }}</span>
            <button
              class="shrink-0 rounded p-1 text-muted transition-colors hover:text-red-600"
              :title="t('git.dialogs.tag.deleteTag')"
              @click="git.deleteTag(tg.name, false)"
            >
              <i class="pi pi-trash text-xs" />
            </button>
          </div>
        </div>
      </div>
    </div>
    <template #footer>
      <DialogFooter
        :cancel-label="t('git.dialogs.tag.close')"
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="t('git.dialogs.tag.confirm')"
        confirm-icon="pi pi-tag"
        :confirm-disabled="!tagName.trim() || !!git.busyMessage.value"
        @cancel="visible = false"
        @confirm="doCreateTag"
      />
    </template>
  </Dialog>
</template>
