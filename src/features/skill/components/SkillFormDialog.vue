<script setup lang="ts">
import { ref, watch, computed, onBeforeUnmount } from "vue";
import { useI18n } from "vue-i18n";
import Button from "primevue/button";
import Dialog from "primevue/dialog";
import InputText from "primevue/inputtext";
import Select from "primevue/select";
import Textarea from "primevue/textarea";
import IconPickerDialog from "@/shared/components/IconPickerDialog.vue";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { Skill, SkillCategory, SkillRequest } from "@/models/skill";
import { DEFAULT_SKILL_ICON, SKILL_CATEGORY_META } from "@/models/skill";

const props = defineProps<{
  visible: boolean;
  skill: Skill | null;
}>();

const emit = defineEmits<{
  "update:visible": [value: boolean];
  save: [request: SkillRequest];
}>();

const { t } = useI18n();

const categoryOptions = computed(() =>
  (Object.keys(SKILL_CATEGORY_META) as SkillCategory[]).map((value) => ({ label: t(`skill.category.${value}`), value })),
);

const name = ref("");
const description = ref("");
const icon = ref(DEFAULT_SKILL_ICON);
const category = ref<SkillCategory>("custom");
const instructions = ref("");
const tagsText = ref("");
const showIconPicker = ref(false);
const fullscreen = ref(false);

// Resize (giữ kích thước giữa các lần mở)
const dialogWidth = ref(512);
const dialogHeight = ref(540);
const MIN_WIDTH = 380;
const MIN_HEIGHT = 300;
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
    if (props.skill) {
      name.value = props.skill.name;
      description.value = props.skill.description;
      icon.value = props.skill.icon;
      category.value = props.skill.category;
      instructions.value = props.skill.instructions;
      tagsText.value = props.skill.tags.join(", ");
    } else {
      name.value = "";
      description.value = "";
      icon.value = DEFAULT_SKILL_ICON;
      category.value = "custom";
      instructions.value = "";
      tagsText.value = "";
    }
  },
);

function save() {
  const trimmedName = name.value.trim();
  if (!trimmedName) return;
  emit("save", {
    name: trimmedName,
    description: description.value.trim(),
    icon: icon.value,
    category: category.value,
    instructions: instructions.value,
    tags: tagsText.value.split(",").map((s) => s.trim()).filter(Boolean),
  });
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
    :class="dialogClass"
    :style="dialogStyle"
    :closable="true"
    modal
    @update:visible="emit('update:visible', $event)"
  >
    <template #header>
      <div class="flex flex-1 items-center justify-between">
        <h3 class="section-title">{{ skill ? t("skill.dialog.editTitle") : t("skill.dialog.newTitle") }}</h3>
        <Button
          :icon="fullscreen ? 'pi pi-window-minimize' : 'pi pi-window-maximize'"
          text
          rounded
          size="small"
          severity="secondary"
          :title="fullscreen ? t('skill.dialog.exitFullscreen') : t('skill.dialog.fullscreen')"
          @click="fullscreen = !fullscreen"
        />
      </div>
    </template>

    <div :class="fullscreen ? 'flex flex-col gap-4' : 'space-y-4'">
      <div class="flex items-end gap-3">
        <label class="block min-w-0 flex-1">
          <span class="text-xs font-bold text-muted">{{ t("skill.dialog.name") }} <span class="text-red-500">*</span></span>
          <InputText v-model="name" class="mt-1 w-full" :placeholder="t('skill.dialog.namePlaceholder')" autofocus />
        </label>
        <div class="block">
          <span class="text-xs font-bold text-muted">{{ t("skill.dialog.icon") }}</span>
          <div class="mt-1 flex items-center gap-2">
            <div class="flex h-10 items-center gap-2 rounded-md border border-divider bg-panel px-3">
              <i :class="[icon, 'text-brand']" />
              <InputText v-model="icon" class="embedded-input w-24 border-0 !bg-transparent !p-0 !text-sm" placeholder="pi pi-book" />
            </div>
            <Button icon="pi pi-th-large" severity="secondary" outlined :title="t('skill.dialog.browseIcons')" @click="showIconPicker = true" />
          </div>
        </div>
      </div>

      <label class="block">
        <span class="text-xs font-bold text-muted">{{ t("skill.dialog.category") }}</span>
        <Select v-model="category" :options="categoryOptions" optionLabel="label" optionValue="value" class="mt-1 w-full" :pt="selectPt" />
      </label>

      <label class="block">
        <span class="text-xs font-bold text-muted">{{ t("skill.dialog.description") }}</span>
        <InputText v-model="description" class="mt-1 w-full" :placeholder="t('skill.dialog.descriptionPlaceholder')" />
      </label>

      <label :class="['block', fullscreen && 'flex flex-1 flex-col']">
        <span class="text-xs font-bold text-muted">{{ t("skill.dialog.instructions") }}</span>
        <Textarea
          v-model="instructions"
          :rows="fullscreen ? 20 : 8"
          :class="['mt-1 w-full !text-xs', fullscreen && 'flex-1 resize-none']"
          :placeholder="t('skill.dialog.instructionsPlaceholder')"
        />
      </label>

      <label class="block">
        <span class="text-xs font-bold text-muted">{{ t("skill.dialog.tags") }}</span>
        <InputText v-model="tagsText" class="mt-1 w-full" :placeholder="t('skill.dialog.tagsPlaceholder')" />
      </label>
    </div>

    <div v-if="!fullscreen" class="dialog-resize-handle" :title="t('common.resize')" @mousedown="startResize" />

    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="skill ? t('common.save') : t('skill.dialog.create')"
        :confirm-icon="skill ? 'pi pi-check' : 'pi pi-plus'"
        :confirm-disabled="!name.trim()"
        @cancel="emit('update:visible', false)"
        @confirm="save"
      />
    </template>
  </Dialog>

  <IconPickerDialog
    :visible="showIconPicker"
    :selected="icon"
    @update:visible="showIconPicker = $event"
    @select="(picked: string) => (icon = 'pi ' + picked)"
  />
</template>
