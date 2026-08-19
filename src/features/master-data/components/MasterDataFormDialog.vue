<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import Dialog from "primevue/dialog";
import InputText from "primevue/inputtext";
import InputNumber from "primevue/inputnumber";
import Textarea from "primevue/textarea";
import Button from "primevue/button";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import IconPickerDialog from "@/shared/components/IconPickerDialog.vue";
import type { MasterDataApi } from "../composables/useMasterData";

const props = defineProps<{ ctrl: MasterDataApi }>();
const visible = defineModel<boolean>("visible", { default: false });

const { t } = useI18n();

const showIconPicker = ref(false);

async function saveAndClose() {
  if (await props.ctrl.saveDraft()) {
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
        <h3 class="section-title">
          {{ ctrl.isCreating.value ? t("masterData.dialog.registerTitle") : t("masterData.dialog.editTitle") }}
        </h3>
        <p class="mt-0.5 text-xs text-muted">{{ t("masterData.dialog.subtitle") }}</p>
      </div>
    </template>

    <div class="space-y-4">
      <label class="block">
        <span class="text-xs font-bold text-muted">{{ t("masterData.form.name") }} <span class="text-red-500">*</span></span>
        <InputText
          class="mt-1 w-full"
          :model-value="ctrl.draft.value.name"
          :placeholder="t('masterData.form.namePlaceholder')"
          autofocus
          @update:model-value="ctrl.updateDraft('name', ($event as string) ?? '')"
        />
      </label>

      <div class="grid gap-4 md:grid-cols-2">
        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("masterData.form.keygroup") }}</span>
          <InputText
            class="mt-1 w-full font-mono"
            :model-value="ctrl.draft.value.keygroup"
            :placeholder="t('masterData.form.keygroupPlaceholder')"
            @update:model-value="ctrl.updateDraft('keygroup', ($event as string) ?? '')"
          />
        </label>
        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t("masterData.form.displayOrder") }}</span>
          <InputNumber
            class="mt-1 w-full"
            :model-value="ctrl.draft.value.display_order"
            :min="0"
            show-buttons
            @update:model-value="ctrl.updateDraft('display_order', $event ?? 0)"
          />
        </label>
      </div>

      <label class="block">
        <span class="text-xs font-bold text-muted">{{ t("masterData.form.icon") }}</span>
        <div class="mt-1 flex items-center gap-2">
          <div class="flex flex-1 items-center gap-2 rounded-md border border-divider bg-panel px-3">
            <i :class="[ctrl.draft.value.icon || 'pi pi-tag', 'text-brand']" />
            <InputText
              :model-value="ctrl.draft.value.icon"
              class="embedded-input w-full border-0 !bg-transparent !p-0 !py-2 !text-sm"
              placeholder="pi pi-tag"
              @update:model-value="ctrl.updateDraft('icon', ($event as string) ?? '')"
            />
          </div>
          <Button
            icon="pi pi-th-large"
            severity="secondary"
            outlined
            :title="t('masterData.form.browseIcons')"
            @click="showIconPicker = true"
          />
        </div>
      </label>

      <label class="block">
        <span class="text-xs font-bold text-muted">{{ t("masterData.form.description") }}</span>
        <Textarea
          :model-value="ctrl.draft.value.description"
          :rows="3"
          class="mt-1 w-full !text-xs"
          :placeholder="t('masterData.form.descriptionPlaceholder')"
          @update:model-value="ctrl.updateDraft('description', ($event as string) ?? '')"
        />
      </label>
    </div>

    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="ctrl.isCreating.value ? t('masterData.actions.register') : t('masterData.actions.save')"
        :confirm-icon="ctrl.isCreating.value ? 'pi pi-plus' : 'pi pi-save'"
        :confirm-disabled="!ctrl.draft.value.name.trim()"
        @cancel="visible = false"
        @confirm="saveAndClose"
      />
    </template>
  </Dialog>

  <IconPickerDialog
    :visible="showIconPicker"
    :selected="ctrl.draft.value.icon"
    @update:visible="showIconPicker = $event"
    @select="(picked: string) => ctrl.updateDraft('icon', 'pi ' + picked)"
  />
</template>
