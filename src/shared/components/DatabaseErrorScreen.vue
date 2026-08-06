<script setup lang="ts">
import { useI18n } from "vue-i18n";
import Button from "primevue/button";
import { useDatabaseStatus } from "@/shared/composables/useDatabaseStatus";

const database = useDatabaseStatus();
const { t } = useI18n();
</script>

<template>
  <main
    class="flex h-screen min-h-[640px] min-w-[900px] items-center justify-center overflow-hidden bg-canvas text-ink">
    <section aria-live="assertive" :aria-label="t('databaseError.ariaLabel')"
      class="flex max-w-md select-none flex-col items-center gap-6 px-8 text-center" role="alert">
      <span aria-hidden="true"
        class="flex h-20 w-20 items-center justify-center rounded-full bg-red-50 text-red-600 dark:bg-red-950 dark:text-red-400">
        <i class="pi pi-database text-3xl" />
      </span>

      <div class="flex flex-col gap-2">
        <h1 class="text-lg font-semibold text-ink">{{ t('databaseError.title') }}</h1>
        <p class="text-sm text-secondary">
          {{ t('databaseError.description') }}
        </p>
        <p v-if="database.statusMessage.value"
          class="mt-1 break-words rounded-md border border-red-200 bg-red-50 px-3 py-2 text-xs text-red-700">
          {{ database.statusMessage.value }}
        </p>
      </div>

      <div class="flex items-center gap-2">
        <Button icon="pi pi-cog" :label="t('databaseError.reconfigure')" severity="secondary" outlined
          :disabled="database.isChecking.value" @click="database.requestReconfigure()" />

        <Button :icon="database.isChecking.value ? 'pi pi-spinner pi-spin' : undefined"
          :label="database.isChecking.value ? t('common.checking') : t('common.retry')" :disabled="database.isChecking.value"
          @click="database.check()" />
      </div>
    </section>
  </main>
</template>
