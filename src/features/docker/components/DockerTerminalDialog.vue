<script setup lang="ts">
import { onBeforeUnmount, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import Dialog from "primevue/dialog";
import Select from "primevue/select";
import Button from "primevue/button";
import { Terminal } from "@xterm/xterm";
import { FitAddon } from "@xterm/addon-fit";
import "@xterm/xterm/css/xterm.css";
import { terminalKill, terminalResize, terminalSpawn, terminalWrite } from "@/tauri/commands/terminal";
import { friendlyError } from "@/tauri/commands/_base";
import { useToast } from "@/shared/composables/useToast";

/**
 * Panel exec/logs độc lập với màn hình Terminal (`src/features/terminal`): tự
 * spawn/kill phiên PTY riêng của chính nó qua `terminal_spawn`/`terminal_kill`
 * (hạ tầng dùng chung, không đụng tới `useTerminal()` singleton hay tab của trang
 * Terminal), gắn xterm ngay trong dialog này.
 */
const props = defineProps<{
  containerId: string;
  containerName: string;
  mode: "exec" | "logs";
}>();
const visible = defineModel<boolean>("visible", { default: false });

const { t } = useI18n();
const toast = useToast();

const SHELL_OPTIONS = ["sh", "bash", "ash"];
const shell = ref(SHELL_OPTIONS[0]);

const THEME = {
  background: "#0b0f19",
  foreground: "#e5e9f0",
  cursor: "#e5e9f0",
  selectionBackground: "#33415580",
};

const containerEl = ref<HTMLElement | null>(null);
let term: Terminal | null = null;
let fit: FitAddon | null = null;
let ro: ResizeObserver | null = null;
let sessionId: string | null = null;

function base64ToBytes(b64: string): Uint8Array {
  const bin = atob(b64);
  const bytes = new Uint8Array(bin.length);
  for (let i = 0; i < bin.length; i++) bytes[i] = bin.charCodeAt(i);
  return bytes;
}

function commandFor(): string {
  return props.mode === "exec"
    ? `docker exec -it ${props.containerId} ${shell.value}`
    : `docker logs -f --tail 300 ${props.containerId}`;
}

async function disconnect() {
  ro?.disconnect();
  ro = null;
  if (sessionId) {
    await terminalKill(sessionId).catch(() => undefined);
    sessionId = null;
  }
  term?.dispose();
  term = null;
  fit = null;
}

async function connect() {
  if (!containerEl.value) return;
  await disconnect();

  term = new Terminal({
    fontFamily: '"Cascadia Code", "JetBrains Mono", Menlo, Consolas, monospace',
    fontSize: 13,
    cursorBlink: true,
    scrollback: 5000,
    theme: THEME,
  });
  fit = new FitAddon();
  term.loadAddon(fit);
  term.open(containerEl.value);
  fit.fit();

  term.onData((data) => {
    if (sessionId) void terminalWrite(sessionId, data).catch(() => undefined);
  });
  term.onResize(({ rows, cols }) => {
    if (sessionId) void terminalResize(sessionId, rows, cols).catch(() => undefined);
  });

  ro = new ResizeObserver(() => {
    try {
      fit?.fit();
    } catch {
      // Container có thể đang ẩn (kích thước 0) — bỏ qua.
    }
  });
  ro.observe(containerEl.value);

  try {
    const id = await terminalSpawn(
      term.rows,
      term.cols,
      (dataBase64) => term?.write(base64ToBytes(dataBase64)),
      (code) => {
        term?.writeln(`\r\n\x1b[90m[process exited${code === null ? "" : ` with code ${code}`}]\x1b[0m`);
      },
    );
    sessionId = id;
    void terminalResize(id, term.rows, term.cols).catch(() => undefined);
    void terminalWrite(id, `${commandFor()}\r`).catch(() => undefined);
  } catch (e) {
    term.writeln(`\x1b[31m${friendlyError(e)}\x1b[0m`);
    toast.error(friendlyError(e));
  }
}

function reconnect() {
  void connect();
}

watch(visible, (v) => {
  if (v) {
    requestAnimationFrame(() => void connect());
  } else {
    void disconnect();
  }
});

onBeforeUnmount(() => {
  void disconnect();
});
</script>

<template>
  <Dialog
    v-model:visible="visible"
    modal
    :header="
      mode === 'exec'
        ? t('docker.terminal.execTitle', { name: containerName })
        : t('docker.terminal.logsTitle', { name: containerName })
    "
    :style="{ width: '820px' }"
    :dismissable-mask="false"
    maximizable
  >
    <div class="flex flex-col gap-2">
      <div v-if="mode === 'exec'" class="flex items-center gap-2 text-xs">
        <span class="text-muted">{{ t("docker.terminal.shellLabel") }}</span>
        <Select v-model="shell" :options="SHELL_OPTIONS" class="w-28" />
        <Button
          size="small"
          icon="pi pi-refresh"
          :label="t('docker.terminal.reconnect')"
          severity="secondary"
          outlined
          @click="reconnect"
        />
      </div>
      <div ref="containerEl" class="terminal-host w-full overflow-hidden rounded-md p-1" style="background: #0b0f19" />
    </div>
    <template #footer>
      <Button icon="pi pi-times" :label="t('common.close')" severity="danger" outlined @click="visible = false" />
    </template>
  </Dialog>
</template>

<style scoped>
.terminal-host {
  height: 440px;
}

/* Khi dialog ở chế độ full màn hình, cho khung terminal lấp đầy không gian. */
.p-dialog-maximized .terminal-host {
  height: calc(100vh - 200px);
}
</style>
