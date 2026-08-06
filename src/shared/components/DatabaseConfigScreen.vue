<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import Button from "primevue/button";
import InputText from "primevue/inputtext";
import Password from "primevue/password";
import Dialog from "primevue/dialog";
import InputNumber from "primevue/inputnumber";
import { useRouter } from "vue-router";
import { friendlyError } from "@/tauri/commands/_base";
import { getDatabaseConfig, testDatabaseConfig } from "@/tauri/commands/database-config";
import type { DatabaseStatus, SaveDatabaseConfigRequest } from "@/models/database-config";
import { useDatabaseStatus } from "@/shared/composables/useDatabaseStatus";
import { useAuthStore } from "@/app/stores/auth";
import { loginRoute } from "@/app/router/routes";

const { t } = useI18n();
const router = useRouter();
const auth = useAuthStore();
const database = useDatabaseStatus();

const form = ref<SaveDatabaseConfigRequest>({
  host: "localhost",
  port: 5432,
  dbname: "",
  user: "postgres",
  password: "",
});

const isSaving = ref(false);
const isTesting = ref(false);

// Confirm dialog shown after test/save. `proceed` marks a successful save whose
// status should be applied (entering the app) once the user acknowledges.
type ResultDialog = { type: "success" | "error"; title: string; text: string; proceed: boolean };
const dialog = ref<ResultDialog | null>(null);
let pendingStatus: DatabaseStatus | null = null;

onMounted(async () => {
  // Prefill with the existing config when the database is configured but the
  // connection currently fails (so the user can fix a wrong field).
  try {
    const existing = await getDatabaseConfig();
    if (existing) {
      form.value = {
        host: existing.host,
        port: existing.port,
        dbname: existing.dbname,
        user: existing.user,
        password: existing.password,
      };
    }
  } catch {
    // Ignore — keep defaults.
  }
});

function showError(text: string) {
  pendingStatus = null;
  dialog.value = { type: "error", title: t("databaseConfig.errorTitle"), text, proceed: false };
}

/** Validate the form and build a request, or show an error dialog and return null. */
function buildRequest(): SaveDatabaseConfigRequest | null {
  if (!form.value.host.trim()) {
    showError(t("databaseConfig.hostRequired"));
    return null;
  }
  if (!form.value.dbname.trim()) {
    showError(t("databaseConfig.dbNameRequired"));
    return null;
  }
  if (!form.value.port || form.value.port <= 0) {
    showError(t("databaseConfig.portInvalid"));
    return null;
  }
  return {
    host: form.value.host.trim(),
    port: Number(form.value.port),
    dbname: form.value.dbname.trim(),
    user: form.value.user.trim(),
    password: form.value.password,
  };
}

/** "Kiểm tra kết nối" — test the connection without writing config.ini. */
async function runTest() {
  if (isTesting.value || isSaving.value) return;
  const request = buildRequest();
  if (!request) return;
  isTesting.value = true;
  try {
    await testDatabaseConfig(request);
    pendingStatus = null;
    dialog.value = { type: "success", title: t("databaseConfig.successTitle"), text: t("databaseConfig.successText"), proceed: false };
  } catch (e) {
    showError(friendlyError(e));
  } finally {
    isTesting.value = false;
  }
}

/** "Lưu cấu hình" — test, persist config.ini and initialize the database. */
async function submit() {
  if (isTesting.value || isSaving.value) return;
  const request = buildRequest();
  if (!request) return;
  isSaving.value = true;
  try {
    const status = await database.save(request);
    pendingStatus = status;
    dialog.value = {
      type: "success",
      title: t("databaseConfig.saveSuccessTitle"),
      text: t("databaseConfig.saveSuccessText"),
      proceed: true,
    };
  } catch (e) {
    showError(friendlyError(e));
  } finally {
    isSaving.value = false;
  }
}

/** Acknowledge the dialog; enter the app if it was a successful save. */
function acknowledge() {
  const proceed = dialog.value?.proceed === true && pendingStatus !== null;
  dialog.value = null;
  if (proceed && pendingStatus) {
    const status = pendingStatus;
    pendingStatus = null;
    auth.logout();
    database.applyStatus(status);
    router.push(loginRoute.path);
  }
}
</script>

<template>
  <main
    class="force-light flex h-screen min-h-[640px] min-w-[900px] items-center justify-center overflow-hidden bg-canvas text-ink"
    data-theme="light">
    <section class="flex w-full max-w-md flex-col gap-6 rounded-xl border border-divider bg-panel p-8 shadow-card"
      :aria-label="t('databaseConfig.ariaLabel')">
      <div class="flex flex-col items-center gap-3 text-center">
        <span aria-hidden="true" class="flex h-16 w-16 items-center justify-center rounded-full bg-brand/10 text-brand">
          <i class="pi pi-database text-2xl" />
        </span>
        <div class="flex flex-col gap-1">
          <h1 class="text-lg font-bold text-ink">{{ t('databaseConfig.title') }}</h1>
          <p class="text-sm text-secondary">
            {{
              database.isConfigured.value
                ? t('databaseConfig.descriptionError')
                : t('databaseConfig.descriptionEmpty')
            }}
          </p>
        </div>
      </div>

      <p v-if="database.statusMessage.value && database.isConfigured.value"
        class="rounded-md border border-amber-500/20 bg-amber-500/10 px-3 py-2 text-xs text-amber-500">
        {{ database.statusMessage.value }}
      </p>

      <form class="flex flex-col gap-3" @submit.prevent="submit">
        <div class="grid grid-cols-[minmax(0,1fr)_120px] gap-3">
          <label class="block">
            <span class="text-xs font-bold text-muted">{{ t('databaseConfig.host') }} <span class="text-red-500">*</span></span>
            <InputText v-model="form.host" class="mt-1 w-full" :placeholder="t('databaseConfig.hostPlaceholder')" autocomplete="off" />
          </label>
          <label class="block">
            <span class="text-xs font-bold text-muted">{{ t('databaseConfig.port') }} <span class="text-red-500">*</span></span>
            <InputNumber v-model="form.port" class="mt-1 w-full" input-class="w-full" :min="1" :max="65535"
              :useGrouping="false" :placeholder="t('databaseConfig.portPlaceholder')" />
          </label>
        </div>

        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t('databaseConfig.dbName') }} <span class="text-red-500">*</span></span>
          <InputText v-model="form.dbname" class="mt-1 w-full" :placeholder="t('databaseConfig.dbNamePlaceholder')" autocomplete="off" />
        </label>

        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t('databaseConfig.user') }}</span>
          <InputText v-model="form.user" class="mt-1 w-full" :placeholder="t('databaseConfig.userPlaceholder')" autocomplete="off" />
        </label>

        <label class="block">
          <span class="text-xs font-bold text-muted">{{ t('databaseConfig.password') }}</span>
          <Password v-model="form.password" class="mt-1 w-full" input-class="w-full" :placeholder="t('databaseConfig.passwordPlaceholder')"
            autocomplete="off" :feedback="false" toggle-mask />
        </label>

        <div class="mt-2 grid grid-cols-2 gap-3">
          <Button :icon="isTesting ? 'pi pi-spinner pi-spin' : undefined"
            :label="isTesting ? t('databaseConfig.testing') : t('databaseConfig.testConnection')" severity="secondary" outlined
            :disabled="isTesting || isSaving" @click="runTest" />

          <Button :icon="isSaving ? 'pi pi-spinner pi-spin' : undefined"
            :label="isSaving ? t('databaseConfig.saving') : t('databaseConfig.save')" type="submit" :disabled="isTesting || isSaving" />
        </div>

        <Button v-if="database.wantsReconfigure.value" icon="pi pi-arrow-left" :label="t('databaseConfig.back')" text size="small"
          class="mt-1 self-center" :disabled="isTesting || isSaving" @click="database.cancelReconfigure()" />
      </form>
    </section>

    <!-- Result confirm dialog -->
    <Dialog :visible="!!dialog" class="w-full max-w-sm" modal :closable="false" :draggable="false"
      @update:visible="(v: boolean) => { if (!v) acknowledge(); }">
      <template #header>
        <div class="flex items-center gap-2">
          <span :class="[
            'flex h-8 w-8 items-center justify-center rounded-full',
            dialog?.type === 'success' ? 'bg-brand/10 text-brand' : 'bg-red-500/10 text-red-500',
          ]">
            <i :class="dialog?.type === 'success' ? 'pi pi-check' : 'pi pi-times'" />
          </span>
          <h3 class="font-bold text-ink">{{ dialog?.title }}</h3>
        </div>
      </template>

      <p class="text-sm text-secondary">{{ dialog?.text }}</p>

      <template #footer>
        <Button :label="t('common.ok')" @click="acknowledge" />
      </template>
    </Dialog>
  </main>
</template>
