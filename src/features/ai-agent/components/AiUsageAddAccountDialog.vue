<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { open } from "@tauri-apps/plugin-dialog";
import Button from "primevue/button";
import Dialog from "primevue/dialog";
import InputGroup from "primevue/inputgroup";
import InputText from "primevue/inputtext";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import type { AiUsageApi } from "../composables/useAiUsage";
import type { AiProvider } from "@/models/ai-usage";

const props = defineProps<{ ctrl: AiUsageApi }>();
const visible = defineModel<boolean>("visible", { default: false });
const emit = defineEmits<{ "login-terminal": [configDir: string] }>();

const { t } = useI18n();

const accountName = ref("");
const apiKey = ref("");
const provider = ref<AiProvider>("claude");
const showApiKey = ref(false);
/** Kiểu account đang thêm: subscription (tool tự capture token) hay API key. */
const accountKind = ref<"subscription" | "key">("subscription");
/** Với subscription: capture login hiện tại, hay đăng ký từ một config dir khác. */
const subMode = ref<"current" | "dir">("current");
const configDir = ref("");

watch(visible, (v) => {
  if (!v) return;
  accountName.value = "";
  apiKey.value = "";
  provider.value = "claude";
  showApiKey.value = false;
  accountKind.value = "subscription";
  subMode.value = "current";
  configDir.value = "";
  props.ctrl.capturePreview.value = null;
  props.ctrl.configDirPreview.value = null;
  void loadPreviewAndPrefill();
});

/** Đổi provider — Claude luôn là subscription, Codex chỉ API key. */
function selectProvider(next: AiProvider) {
  provider.value = next;
  accountKind.value = next === "claude" ? "subscription" : "key";
  if (next === "claude" && subMode.value === "current") void loadPreviewAndPrefill();
}

/** Nạp login hiện tại + prefill tên (chỉ khi tên còn trống). */
async function loadPreviewAndPrefill() {
  await props.ctrl.loadCapturePreview();
  const preview = props.ctrl.capturePreview.value;
  if (preview && !accountName.value.trim()) {
    accountName.value = preview.display_name || preview.email;
  }
}

/** Đổi giữa "capture login hiện tại" và "đăng ký config dir khác". */
function selectSubMode(mode: "current" | "dir") {
  subMode.value = mode;
  if (mode === "current") void loadPreviewAndPrefill();
}

/** Mở dialog chọn folder → điền vào config dir. */
async function browseConfigDir() {
  const selected = await open({ directory: true, title: t("aiUsage.dialog.selectConfigDirTitle") });
  if (typeof selected === "string") {
    configDir.value = selected;
    await onConfigDirInput();
  }
}

/** Xoá đường dẫn config dir đã chọn + reset preview đọc được. */
function clearConfigDir() {
  configDir.value = "";
  props.ctrl.configDirPreview.value = null;
}

/** Preview login tại config dir + prefill tên. */
async function onConfigDirInput() {
  const dir = configDir.value.trim();
  if (!dir) return;
  await props.ctrl.previewConfigDir(dir);
  const preview = props.ctrl.configDirPreview.value;
  if (preview && !accountName.value.trim()) {
    accountName.value = preview.display_name || preview.email;
  }
}

/** Yêu cầu trang cha mở dialog terminal để login tại config dir hiện tại. */
function openLoginForConfigDir() {
  const dir = configDir.value.trim();
  if (!dir) return;
  emit("login-terminal", dir);
}

/** Re-check login sau khi đã login xong trong terminal. */
async function recheckConfigDir() {
  await onConfigDirInput();
}

/** Form hợp lệ: subscription cần đọc được login (hiện tại cần token; config dir chỉ cần identity). */
const canSaveAccount = computed(() => {
  if (!accountName.value.trim()) return false;
  if (accountKind.value !== "subscription") return !!apiKey.value.trim();
  return subMode.value === "current" ? !!props.ctrl.capturePreview.value?.has_token : !!props.ctrl.configDirPreview.value;
});

async function saveAccount() {
  if (!canSaveAccount.value) return;
  if (accountKind.value === "subscription") {
    // Subscription: capture login hiện tại, hoặc đăng ký từ một config dir khác.
    const ok =
      subMode.value === "current"
        ? await props.ctrl.captureAdd(accountName.value.trim())
        : await props.ctrl.addConfigDir(configDir.value.trim(), accountName.value.trim());
    if (ok) visible.value = false;
    return;
  }
  const ok = await props.ctrl.addAccount({
    name: accountName.value.trim(),
    provider: provider.value,
    api_key: apiKey.value.trim(),
  });
  if (ok) visible.value = false;
}
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
      <h3 class="section-title">{{ t("aiUsage.addDialog.header") }}</h3>
    </template>

    <div class="space-y-4">
      <label class="block">
        <span class="text-xs font-bold text-muted">{{ t("aiUsage.addDialog.provider") }}</span>
        <div class="mt-1 flex gap-2">
          <Button
            label="Claude"
            size="small"
            :severity="provider === 'claude' ? undefined : 'secondary'"
            :outlined="provider !== 'claude'"
            @click="selectProvider('claude')"
          />
          <Button
            label="Codex"
            size="small"
            :severity="provider === 'codex' ? undefined : 'secondary'"
            :outlined="provider !== 'codex'"
            @click="selectProvider('codex')"
          />
        </div>
      </label>

      <label class="block">
        <span class="text-xs font-bold text-muted">{{ t("aiUsage.addDialog.accountName") }} <span class="text-red-500">*</span></span>
        <InputText v-model="accountName" class="mt-1 w-full" :placeholder="t('aiUsage.addDialog.accountNamePlaceholder')" autofocus />
      </label>

      <!-- Subscription: tool tự capture token của login Claude đang active -->
      <div v-if="accountKind === 'subscription'" class="block space-y-2">
        <!-- Nguồn login: capture hiện tại hay từ config dir khác (thêm acc thứ 2) -->
        <div class="flex gap-2">
          <Button
            :label="t('aiUsage.addDialog.currentLogin')"
            size="small"
            :severity="subMode === 'current' ? undefined : 'secondary'"
            :outlined="subMode !== 'current'"
            @click="selectSubMode('current')"
          />
          <Button
            :label="t('aiUsage.addDialog.otherConfigDir')"
            size="small"
            :severity="subMode === 'dir' ? undefined : 'secondary'"
            :outlined="subMode !== 'dir'"
            @click="selectSubMode('dir')"
          />
        </div>

        <!-- Config dir input (đăng ký account thứ 2 đã login ở dir riêng) -->
        <label v-if="subMode === 'dir'" class="block">
          <span class="text-xs font-bold text-muted">CLAUDE_CONFIG_DIR <span class="text-red-500">*</span></span>
          <InputGroup class="h-8">
            <InputText readonly placeholder="~/.claude-work" :model-value="configDir" />
            <Button
              icon="pi pi-folder-open"
              severity="secondary"
              outlined
              :title="t('aiUsage.addDialog.selectFolder')"
              @click="browseConfigDir"
            />
            <Button
              v-if="configDir"
              icon="pi pi-times"
              severity="danger"
              text
              :title="t('aiUsage.addDialog.clearPath')"
              @click="clearConfigDir"
            />
          </InputGroup>
          <span class="text-xs text-muted">
            {{ t("aiUsage.addDialog.loginOnceHint") }} <code class="rounded bg-canvas px-1">CLAUDE_CONFIG_DIR=&lt;dir&gt; claude /login</code>
          </span>
        </label>

        <!-- Preview login đọc được (từ login hiện tại hoặc config dir) -->
        <div
          v-if="subMode === 'current' ? ctrl.capturePreview.value?.has_token : ctrl.configDirPreview.value"
          class="rounded-lg border border-divider bg-canvas/50 p-3 text-xs"
        >
          <div class="flex items-center gap-2">
            <i class="pi pi-user text-muted" />
            <span class="truncate font-semibold text-ink">
              {{ (subMode === 'current' ? ctrl.capturePreview.value : ctrl.configDirPreview.value)?.email }}
            </span>
            <span
              v-if="(subMode === 'current' ? ctrl.capturePreview.value : ctrl.configDirPreview.value)?.subscription_type"
              class="shrink-0 badge-info"
            >
              {{ (subMode === 'current' ? ctrl.capturePreview.value : ctrl.configDirPreview.value)?.subscription_type }}
            </span>
          </div>
          <p
            v-if="(subMode === 'current' ? ctrl.capturePreview.value : ctrl.configDirPreview.value)?.token_expires_at"
            class="mt-1 text-xs text-muted"
          >
            <i class="pi pi-clock mr-1" />{{ t("aiUsage.addDialog.tokenExpires") }}
            {{ (subMode === 'current' ? ctrl.capturePreview.value : ctrl.configDirPreview.value)?.token_expires_at }}
          </p>
          <p class="mt-1.5 text-xs text-muted">
            {{ subMode === 'current' ? t("aiUsage.addDialog.tokenSavedHint") : t("aiUsage.addDialog.tokenKeychainHint") }}
          </p>
          <p
            v-if="subMode === 'dir' && ctrl.configDirPreview.value && !ctrl.configDirPreview.value.has_token"
            class="banner-warning mt-2"
          >
            <i class="pi pi-exclamation-triangle mr-1" />{{ t("aiUsage.addDialog.noTokenWarning") }}
            <code class="rounded bg-amber-100 px-1">CLAUDE_CONFIG_DIR=&lt;dir&gt; claude /login</code> {{ t("aiUsage.addDialog.noTokenWarningSuffix") }}
          </p>
        </div>
        <div v-else class="banner-warning">
          <p>
            {{ subMode === 'current' ? t("aiUsage.addDialog.noCurrentLogin") : t("aiUsage.addDialog.noConfigDirLogin") }}
          </p>
          <div v-if="subMode === 'dir' && configDir.trim()" class="mt-2 flex gap-2">
            <Button
              icon="pi pi-terminal"
              :label="t('aiUsage.addDialog.openTerminalToLogin')"
              size="small"
              severity="warn"
              @click="openLoginForConfigDir"
            />
            <Button
              icon="pi pi-refresh"
              :label="t('aiUsage.addDialog.recheck')"
              size="small"
              severity="secondary"
              @click="recheckConfigDir"
            />
          </div>
        </div>
      </div>

      <!-- API key -->
      <label v-else class="block">
        <span class="text-xs font-bold text-muted">{{ t("aiUsage.addDialog.apiKey") }} <span class="text-red-500">*</span></span>
        <div class="relative mt-1">
          <InputText
            v-model="apiKey"
            :type="showApiKey ? 'text' : 'password'"
            class="w-full pr-10"
            placeholder="sk-..."
            autocomplete="off"
          />
          <Button
            :icon="`pi ${showApiKey ? 'pi-eye-slash' : 'pi-eye'}`"
            text
            rounded
            size="small"
            class="absolute right-2 top-1/2 -translate-y-1/2"
            :title="showApiKey ? t('aiUsage.addDialog.hideApiKey') : t('aiUsage.addDialog.showApiKey')"
            @click="showApiKey = !showApiKey"
          />
        </div>
        <span class="text-xs text-muted">{{ t("aiUsage.addDialog.apiKeyHint") }}</span>
      </label>
    </div>

    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="ctrl.isSaving.value || ctrl.isCapturing.value ? t('aiUsage.addDialog.saving') : t('common.save')"
        :confirm-disabled="!canSaveAccount || ctrl.isSaving.value || ctrl.isCapturing.value"
        @cancel="visible = false"
        @confirm="saveAccount"
      />
    </template>
  </Dialog>
</template>
