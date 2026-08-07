<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import { useRouter } from "vue-router";
import Dialog from "primevue/dialog";
import InputText from "primevue/inputtext";
import Select from "primevue/select";
import DialogFooter from "@/shared/components/DialogFooter.vue";
import { useTerminal } from "@/features/terminal/composables/useTerminal";
import type { GitApi } from "../composables/useGit";

/** Giá trị đặc biệt cho lựa chọn "Khác" — hiện textbox để tự nhập tham số. */
const CUSTOM_PRESET = "__custom__";

/** CLI agent có thể khởi chạy trong terminal mới, kèm các tham số dòng lệnh thường dùng. */
const AGENT_OPTIONS = [
  {
    key: "claude",
    title: "Claude",
    command: "claude",
    icon: "pi pi-sparkles",
    presets: ["--dangerously-skip-permissions", "--resume", ""],
  },
  {
    key: "codex",
    title: "Codex",
    command: "codex",
    icon: "pi pi-microchip-ai",
    presets: ["--full-auto", "--dangerously-bypass-approvals-and-sandbox", ""],
  },
  {
    key: "copilot",
    title: "Copilot",
    command: "copilot",
    icon: "pi pi-github",
    presets: ["--allow-all-tools", ""],
  },
] as const;

const { t } = useI18n();
const router = useRouter();
const term = useTerminal();

const props = defineProps<{ git: GitApi }>();
const visible = defineModel<boolean>("visible", { default: false });

const selectedKey = ref<(typeof AGENT_OPTIONS)[number]["key"]>(AGENT_OPTIONS[0].key);
const selectedPreset = ref<string>(AGENT_OPTIONS[0].presets[0]);
const customArgs = ref("");

const currentAgent = computed(
  () => AGENT_OPTIONS.find((a) => a.key === selectedKey.value) ?? AGENT_OPTIONS[0],
);

const presetOptions = computed(() => [
  ...currentAgent.value.presets.map((p) => ({
    label: p || t("git.dialogs.agentTerminal.noArgs"),
    value: p,
  })),
  { label: t("git.dialogs.agentTerminal.customPreset"), value: CUSTOM_PRESET },
]);

const isCustomPreset = computed(() => selectedPreset.value === CUSTOM_PRESET);

const finalArgs = computed(() =>
  isCustomPreset.value ? customArgs.value.trim() : selectedPreset.value,
);

function selectAgent(agent: (typeof AGENT_OPTIONS)[number]) {
  selectedKey.value = agent.key;
  selectedPreset.value = agent.presets[0];
  customArgs.value = "";
}

watch(visible, (v) => {
  if (v) selectAgent(AGENT_OPTIONS[0]);
});

const repoPath = () => props.git.info.value?.path || props.git.activeRepo.value?.path || "";

function doLaunch() {
  const path = repoPath();
  if (!path) return;
  const agent = currentAgent.value;
  const args = finalArgs.value;
  const command = args ? `${agent.command} ${args}` : agent.command;
  const repoName = props.git.activeRepo.value?.name ?? path.split(/[\\/]/).filter(Boolean).pop() ?? agent.title;

  term.addTab({ title: `${agent.title} · ${repoName}`, startDir: path, autoCommand: command });
  visible.value = false;
  void router.push("/terminal");
}
</script>

<template>
  <Dialog v-model:visible="visible" modal :header="t('git.dialogs.agentTerminal.title')" :style="{ width: '460px' }">
    <div class="flex flex-col gap-3">
      <div>
        <label class="mb-1 block text-xs font-bold text-muted">{{ t('git.dialogs.agentTerminal.selectAgent') }}</label>
        <div class="grid grid-cols-3 gap-2">
          <button
            v-for="agent in AGENT_OPTIONS"
            :key="agent.key"
            type="button"
            class="flex flex-col items-center gap-1.5 rounded-md border px-2 py-2.5 text-xs font-medium transition-colors"
            :class="selectedKey === agent.key
              ? 'border-brand bg-brand/10 text-brand'
              : 'border-divider text-secondary hover:bg-canvas'"
            @click="selectAgent(agent)"
          >
            <i :class="agent.icon" class="text-base" />
            {{ agent.title }}
          </button>
        </div>
      </div>
      <div>
        <label class="mb-1 block text-xs font-bold text-muted">{{ t('git.dialogs.agentTerminal.argsLabel') }}</label>
        <Select
          v-model="selectedPreset"
          :options="presetOptions"
          option-label="label"
          option-value="value"
          class="w-full font-mono text-sm"
        />
        <InputText
          v-if="isCustomPreset"
          v-model="customArgs"
          :placeholder="t('git.dialogs.agentTerminal.argsPlaceholder')"
          class="mt-2 w-full font-mono text-sm"
          @keydown.enter="doLaunch"
        />
      </div>
      <p class="text-xs text-muted">{{ t('git.dialogs.agentTerminal.hint') }}</p>
    </div>
    <template #footer>
      <DialogFooter
        cancel-icon="pi pi-times"
        cancel-severity="danger"
        :confirm-label="t('git.dialogs.agentTerminal.confirm')"
        confirm-icon="pi pi-play"
        @cancel="visible = false"
        @confirm="doLaunch"
      />
    </template>
  </Dialog>
</template>
