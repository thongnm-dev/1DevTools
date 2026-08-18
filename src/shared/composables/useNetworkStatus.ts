import { readonly, ref } from "vue";
import { canUseTauriRuntime } from "@/tauri/commands/_base";
import { checkInternetConnection } from "@/tauri/commands/system";

const POLL_INTERVAL_MS = 15000;

const isOnline = ref(true);
const isChecking = ref(false);
const hasConnectedOnce = ref(false);

// Consecutive probe failures required before declaring offline.
// Prevents a single transient timeout from flashing the offline banner.
let consecutiveFailures = 0;
const OFFLINE_THRESHOLD = 2;

let pollTimer: number | undefined;
let started = false;

async function runProbe(): Promise<boolean> {
  if (canUseTauriRuntime()) {
    try {
      return await checkInternetConnection();
    } catch {
      return navigator.onLine;
    }
  }
  return navigator.onLine;
}

async function check(): Promise<boolean> {
  isChecking.value = true;
  try {
    const online = await runProbe();

    if (online) {
      consecutiveFailures = 0;
      isOnline.value = true;
      hasConnectedOnce.value = true;
    } else {
      consecutiveFailures++;
      if (!hasConnectedOnce.value || consecutiveFailures >= OFFLINE_THRESHOLD) {
        isOnline.value = false;
      }
    }

    return isOnline.value;
  } finally {
    isChecking.value = false;
  }
}

function handleBrowserOffline() {
  // The OS/interface reports the link is down — reflect it immediately.
  isOnline.value = false;
}

function handleBrowserOnline() {
  // The link is back, but confirm real reachability before clearing the error.
  void check();
}

/**
 * Starts connectivity monitoring exactly once: an initial probe, browser
 * online/offline listeners for instant reaction, and periodic polling to catch
 * silent drops. Returns the promise of the initial check.
 */
function start(): Promise<boolean> {
  if (started) {
    return Promise.resolve(isOnline.value);
  }
  started = true;

  window.addEventListener("online", handleBrowserOnline);
  window.addEventListener("offline", handleBrowserOffline);

  pollTimer = window.setInterval(() => void check(), POLL_INTERVAL_MS);

  return check();
}

export function useNetworkStatus() {
  return {
    isOnline: readonly(isOnline),
    isChecking: readonly(isChecking),
    hasConnectedOnce: readonly(hasConnectedOnce),
    start,
    retry: check,
  };
}
