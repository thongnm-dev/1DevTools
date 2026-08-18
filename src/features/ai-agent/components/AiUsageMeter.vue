<script setup lang="ts">
import { useI18n } from "vue-i18n";

withDefaults(
  defineProps<{
    label: string;
    /** % còn lại (0–100) từ backend — component tự đổi sang % đã dùng để bar tăng dần 0→100%. */
    remainingPercent: number;
    /** Thời điểm reset (`YYYY-MM-DD HH:MM:SS`). Bỏ trống (undefined) → không hiển thị dòng reset. */
    resetAt?: string;
    size?: "sm" | "md";
  }>(),
  { resetAt: undefined, size: "sm" },
);

const { t } = useI18n();

function usedPercent(remainingPercent: number): number {
  return Math.min(100, Math.max(0, 100 - remainingPercent));
}

function usageBarClass(usedPercentValue: number): string {
  if (usedPercentValue >= 90) return "bg-red-500";
  if (usedPercentValue >= 70) return "bg-amber-500";
  return "bg-brand";
}

/** Diễn giải reset_at (`YYYY-MM-DD HH:MM:SS`) thành chuỗi ngắn — cùng ngày chỉ hiện giờ
 * (vd "sẽ reset sau 2h 15m ・ 11:10"), khác ngày (thường gặp ở giới hạn tuần) hiện thêm
 * ngày dạng yyyy/MM/dd để không mơ hồ (vd "sẽ reset sau 3d ・ 2026/08/21 11:10"). */
function resetHint(resetAt: string): string {
  const raw = resetAt?.trim();
  if (!raw) return "—";
  const target = new Date(raw.replace(" ", "T"));
  if (Number.isNaN(target.getTime())) return raw;
  const diffMs = target.getTime() - Date.now();
  const sameDay = target.toDateString() === new Date().toDateString();
  const time = raw.slice(11, 16) || "";
  const clock = sameDay ? time : `${raw.slice(0, 4)}/${raw.slice(5, 7)}/${raw.slice(8, 10)} ${time}`;
  if (diffMs <= 0) return t("aiUsage.meter.resetSoon", { clock });
  const mins = Math.round(diffMs / 60000);
  const days = Math.floor(mins / 1440);
  const hours = Math.floor((mins % 1440) / 60);
  const rem = mins % 60;
  const parts: string[] = [];
  if (days > 0) parts.push(`${days}d`);
  if (hours > 0) parts.push(`${hours}h`);
  if (days === 0 && rem > 0) parts.push(`${rem}m`);
  const rel = parts.length ? parts.join(" ") : "<1m";
  return t("aiUsage.meter.resetIn", { rel, clock });
}
</script>

<template>
  <div>
    <div class="flex items-center justify-between" :class="size === 'md' ? 'text-xs' : 'text-[11px]'">
      <span class="min-w-0 truncate font-bold text-muted">{{ label }} <template v-if="resetAt !== undefined"> {{ resetHint(resetAt) }}</template></span>
      <span class="shrink-0 font-bold text-ink">{{ Math.round(usedPercent(remainingPercent)) }}%</span>
    </div>
    <div class="overflow-hidden rounded-full bg-canvas" :class="size === 'md' ? 'mt-1.5 h-2' : 'mt-1 h-1.5'">
      <div
        :class="['h-full rounded-full transition-all', usageBarClass(usedPercent(remainingPercent))]"
        :style="{ width: `${usedPercent(remainingPercent)}%` }"
      />
    </div>
  </div>
</template>
