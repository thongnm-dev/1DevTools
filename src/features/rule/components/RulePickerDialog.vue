<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import Dialog from "primevue/dialog";
import InputText from "primevue/inputtext";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { Rule } from "@/models/rule";

const props = defineProps<{
  visible: boolean;
  rules: Rule[];
  modelValue: number[];
}>();

const emit = defineEmits<{
  "update:visible": [value: boolean];
  "update:modelValue": [value: number[]];
}>();

const { t } = useI18n();

const search = ref("");
const staged = ref<number[]>([]);

watch(
  () => props.visible,
  (v) => {
    if (!v) return;
    search.value = "";
    staged.value = [...props.modelValue];
  },
);

const filtered = computed(() => {
  const q = search.value.trim().toLowerCase();
  if (!q) return props.rules;
  return props.rules.filter(
    (r) => r.name.toLowerCase().includes(q) || r.description.toLowerCase().includes(q) || r.tags.some((tag) => tag.toLowerCase().includes(q)),
  );
});

function toggle(id: number) {
  const idx = staged.value.indexOf(id);
  if (idx === -1) staged.value.push(id);
  else staged.value.splice(idx, 1);
}

function confirm() {
  emit("update:modelValue", staged.value);
  emit("update:visible", false);
}
</script>

<template>
  <Dialog
    :visible="visible"
    class="w-full max-w-2xl rounded-lg bg-panel shadow-xl"
    :closable="true"
    modal
    @update:visible="emit('update:visible', $event)"
  >
    <template #header>
      <h3 class="section-title">{{ t("rule.picker.title") }}</h3>
    </template>

    <div class="space-y-3">
      <span class="flex items-center gap-2 rounded-md border border-divider bg-canvas px-3">
        <i class="pi pi-search text-xs text-muted" />
        <InputText
          v-model="search"
          class="embedded-input w-full border-0 !bg-transparent !py-2 !text-sm"
          :placeholder="t('rule.picker.searchPlaceholder')"
          autofocus
        />
      </span>

      <p class="text-xs text-muted">{{ t("rule.picker.selectedCount", { count: staged.length }) }}</p>

      <div class="max-h-[420px] overflow-auto rounded-md border border-divider bg-canvas p-2">
        <div v-if="rules.length === 0" class="flex flex-col items-center gap-2 py-10 text-center">
          <i class="pi pi-file-edit text-3xl text-muted/60" />
          <p class="text-sm text-muted">{{ t("rule.picker.noRulesYet") }}</p>
        </div>
        <div v-else-if="filtered.length === 0" class="py-10 text-center text-sm text-muted">
          {{ t("rule.picker.noMatch") }}
        </div>
        <div v-else class="flex flex-col gap-1">
          <button
            v-for="rule in filtered"
            :key="rule.id"
            type="button"
            :class="[
              'flex items-start gap-2 rounded-md border px-3 py-2 text-left transition-colors',
              staged.includes(rule.id)
                ? 'border-brand bg-brand/10'
                : 'border-transparent hover:border-divider hover:bg-panel',
            ]"
            @click="toggle(rule.id)"
          >
            <i :class="['pi text-sm mt-0.5', staged.includes(rule.id) ? 'pi-check-square text-brand' : 'pi-stop text-muted']" />
            <div class="min-w-0 flex-1">
              <div class="flex flex-wrap items-center gap-1.5">
                <span class="text-sm font-medium text-ink">{{ rule.name }}</span>
                <span v-for="tag in rule.tags" :key="tag" class="rounded-full bg-panel px-1.5 py-0.5 text-[10px] text-secondary">{{ tag }}</span>
              </div>
              <p v-if="rule.description" class="mt-0.5 truncate text-xs text-muted">{{ rule.description }}</p>
            </div>
          </button>
        </div>
      </div>
    </div>

    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="t('rule.picker.attach')"
        confirm-icon="pi pi-check"
        @cancel="emit('update:visible', false)"
        @confirm="confirm"
      />
    </template>
  </Dialog>
</template>
