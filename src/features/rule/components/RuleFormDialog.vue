<script setup lang="ts">
import { ref, watch, computed, onBeforeUnmount } from "vue";
import { useI18n } from "vue-i18n";
import { open } from "@tauri-apps/plugin-dialog";
import Button from "primevue/button";
import Dialog from "primevue/dialog";
import InputText from "primevue/inputtext";
import Textarea from "primevue/textarea";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import { explorerReadFile } from "@/tauri/commands/explorer";
import { friendlyError } from "@/tauri/commands/_base";
import { useToast } from "@/shared/composables/useToast";
import type { Rule, RuleRequest } from "@/models/rule";

const props = defineProps<{
  visible: boolean;
  rule: Rule | null;
}>();

const emit = defineEmits<{
  "update:visible": [value: boolean];
  save: [request: RuleRequest];
}>();

const { t } = useI18n();
const toast = useToast();

const name = ref("");
const description = ref("");
const content = ref("");
const tagsText = ref("");
const fullscreen = ref(false);
const importing = ref(false);

// Resize (giữ kích thước giữa các lần mở)
const dialogWidth = ref(900);
const dialogHeight = ref(600);
const MIN_WIDTH = 380;
const MIN_HEIGHT = 320;
let cleanupResize: (() => void) | null = null;

function startResize(event: MouseEvent) {
  event.preventDefault();
  const startX = event.clientX;
  const startY = event.clientY;
  const startWidth = dialogWidth.value;
  const startHeight = dialogHeight.value;
  function onMove(ev: MouseEvent) {
    dialogWidth.value = Math.max(MIN_WIDTH, startWidth + (ev.clientX - startX));
    dialogHeight.value = Math.max(MIN_HEIGHT, startHeight + (ev.clientY - startY));
  }
  function onUp() {
    document.removeEventListener("mousemove", onMove);
    document.removeEventListener("mouseup", onUp);
    document.body.style.userSelect = "";
    cleanupResize = null;
  }
  document.body.style.userSelect = "none";
  document.addEventListener("mousemove", onMove);
  document.addEventListener("mouseup", onUp);
  cleanupResize = onUp;
}

onBeforeUnmount(() => cleanupResize?.());

const dialogClass = computed(() =>
  fullscreen.value
    ? "w-screen h-screen !m-0 !max-w-none !rounded-none bg-panel"
    : "rounded-lg bg-panel shadow-xl",
);

const dialogStyle = computed(() =>
  fullscreen.value ? {} : { width: `${dialogWidth.value}px`, height: `${dialogHeight.value}px` },
);

watch(
  () => props.visible,
  (val) => {
    if (!val) {
      fullscreen.value = false;
      return;
    }
    if (props.rule) {
      name.value = props.rule.name;
      description.value = props.rule.description;
      content.value = props.rule.content;
      tagsText.value = props.rule.tags.join(", ");
    } else {
      name.value = "";
      description.value = "";
      content.value = "";
      tagsText.value = "";
    }
  },
);

/** Đính kèm nội dung từ 1 file `.md` có sẵn trên máy — thay cho việc gõ tay. */
async function importFromFile() {
  try {
    const picked = await open({
      multiple: false,
      filters: [{ name: "Markdown", extensions: ["md", "markdown"] }],
      title: t("rule.dialog.importTitle"),
    });
    if (!picked || typeof picked !== "string") return;
    importing.value = true;
    content.value = await explorerReadFile(picked);
    if (!name.value.trim()) {
      const fileName = picked.split(/[\\/]/).filter(Boolean).pop() ?? "";
      name.value = fileName.replace(/\.(md|markdown)$/i, "");
    }
  } catch (e) {
    toast.error(friendlyError(e));
  } finally {
    importing.value = false;
  }
}

function save() {
  const trimmedName = name.value.trim();
  if (!trimmedName) return;
  emit("save", {
    name: trimmedName,
    description: description.value.trim(),
    content: content.value,
    tags: tagsText.value.split(",").map((s) => s.trim()).filter(Boolean),
  });
}
</script>

<template>
  <Dialog
    :visible="visible"
    :class="dialogClass"
    :style="dialogStyle"
    :closable="true"
    modal
    @update:visible="emit('update:visible', $event)"
  >
    <template #header>
      <div class="flex flex-1 items-center justify-between">
        <h3 class="section-title">{{ rule ? t("rule.dialog.editTitle") : t("rule.dialog.newTitle") }}</h3>
        <Button
          :icon="fullscreen ? 'pi pi-window-minimize' : 'pi pi-window-maximize'"
          text
          rounded
          size="small"
          severity="secondary"
          :title="fullscreen ? t('rule.dialog.exitFullscreen') : t('rule.dialog.fullscreen')"
          @click="fullscreen = !fullscreen"
        />
      </div>
    </template>

    <div :class="fullscreen ? 'flex flex-col gap-4' : 'space-y-3'">
      <!-- Name -->
      <label class="block">
        <span class="text-xs font-bold text-muted">{{ t("rule.dialog.name") }} <span class="text-red-500">*</span></span>
        <InputText v-model="name" class="mt-1 w-full" :placeholder="t('rule.dialog.namePlaceholder')" autofocus />
      </label>

      <!-- Description -->
      <label class="block">
        <span class="text-xs font-bold text-muted">{{ t("rule.dialog.description") }}</span>
        <InputText v-model="description" class="mt-1 w-full" :placeholder="t('rule.dialog.descriptionPlaceholder')" />
      </label>

      <!-- Content -->
      <label :class="['block', fullscreen && 'flex flex-1 flex-col']">
        <div class="flex items-center justify-between">
          <span class="text-xs font-bold text-muted">{{ t("rule.dialog.content") }}</span>
          <Button
            icon="pi pi-upload"
            :label="t('rule.dialog.importFromFile')"
            text
            size="small"
            :loading="importing"
            @click="importFromFile"
          />
        </div>
        <Textarea
          v-model="content"
          :rows="fullscreen ? 20 : 12"
          :class="['mt-1 w-full !text-xs', fullscreen && 'flex-1 resize-none']"
          :placeholder="t('rule.dialog.contentPlaceholder')"
        />
      </label>

      <!-- Tags -->
      <label class="block">
        <span class="text-xs font-bold text-muted">{{ t("rule.dialog.tags") }}</span>
        <InputText v-model="tagsText" class="mt-1 w-full" :placeholder="t('rule.dialog.tagsPlaceholder')" />
      </label>
    </div>

    <div v-if="!fullscreen" class="dialog-resize-handle" :title="t('common.resize')" @mousedown="startResize" />

    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="rule ? t('common.save') : t('common.create')"
        :confirm-icon="rule ? 'pi pi-check' : 'pi pi-plus'"
        :confirm-disabled="!name.trim()"
        @cancel="emit('update:visible', false)"
        @confirm="save"
      />
    </template>
  </Dialog>
</template>
