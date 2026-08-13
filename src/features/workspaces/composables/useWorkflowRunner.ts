import { useI18n } from "vue-i18n";

import { useTerminal } from "@/features/terminal/composables/useTerminal";
import { useSkill } from "./useSkill";
import { usePrompt } from "./usePrompt";
import { aiUsageSetActive } from "@/tauri/commands/ai-usage";
import { friendlyError } from "@/tauri/commands/_base";
import type { Workflow, WorkflowStep } from "@/models/workflow";
import type { Workspace } from "@/models/workspace";
import { useToast } from "@/shared/composables/useToast";

/**
 * "Chạy" 1 Workflow trong ngữ cảnh 1 workspace cụ thể.
 *
 * Workflow là thư viện global (không gắn workspace cố định), nên workspace
 * đích luôn được chọn tại thời điểm chạy — xem `data-model.md` mục 3.
 *
 * Mỗi step resolve thành 1 hành động đơn giản, không có orchestration engine
 * thật (không chờ step trước hoàn thành mới chạy step sau — giống cách
 * Runner ở Git Desktop đang hoạt động: chỉ mở terminal, không theo dõi tiến
 * trình):
 * - `runner`/`terminal`: mở 1 tab terminal tại thư mục workspace, gõ sẵn lệnh.
 * - `skill`/`prompt`: copy nội dung (instructions/body) vào clipboard rồi mở
 *   1 tab terminal trống — người dùng tự paste vào agent. Biến `{{var}}` của
 *   prompt KHÔNG được điền tự động ở bước này (cần điền tay).
 * - `custom`: chỉ là note, không tự chạy gì.
 * - Nếu step có gắn `ai_account_id`, chuyển active account sang đó trước khi
 *   mở terminal cho step.
 * - Step có `is_latest_step = true` là bước cuối cùng được chạy — các step
 *   sau đó (nếu có) bị bỏ qua.
 */
export function useWorkflowRunner() {
  const { t } = useI18n();
  const toast = useToast();
  const term = useTerminal();
  const skillCtrl = useSkill();
  const promptCtrl = usePrompt();

  async function runStep(step: WorkflowStep, workspace: Workspace) {
    if (step.ai_account_id !== null) {
      try {
        await aiUsageSetActive(step.ai_account_id);
      } catch (e) {
        toast.error(friendlyError(e));
      }
    }

    switch (step.step_type) {
      case "runner":
      case "terminal": {
        term.addTab({
          title: `${step.name} · ${workspace.name}`,
          startDir: workspace.project_path,
          autoCommand: step.runner_command ?? undefined,
        });
        break;
      }
      case "skill": {
        const skill = skillCtrl.skills.value.find((s) => s.id === step.skill_id);
        if (!skill) {
          toast.error(t("workflow.run.skillNotFound", { name: step.name }));
          break;
        }
        await navigator.clipboard.writeText(skill.instructions);
        toast.success(t("workflow.run.skillCopied", { name: skill.name }));
        term.addTab({ title: `${step.name} · ${workspace.name}`, startDir: workspace.project_path });
        break;
      }
      case "prompt": {
        const prompt = promptCtrl.prompts.value.find((p) => p.id === step.prompt_id);
        if (!prompt) {
          toast.error(t("workflow.run.promptNotFound", { name: step.name }));
          break;
        }
        await navigator.clipboard.writeText(prompt.body);
        toast.success(t("workflow.run.promptCopied", { title: prompt.title }));
        term.addTab({ title: `${step.name} · ${workspace.name}`, startDir: workspace.project_path });
        break;
      }
      case "custom":
        toast.info(t("workflow.run.customNote", { name: step.name }));
        break;
    }
  }

  async function runWorkflow(workflow: Workflow, workspace: Workspace) {
    if (workflow.steps.length === 0) {
      toast.error(t("workflow.run.noSteps"));
      return;
    }
    toast.success(t("workflow.run.started", { name: workflow.name, workspace: workspace.name }));
    for (const step of workflow.steps) {
      await runStep(step, workspace);
      if (step.is_latest_step) break;
    }
  }

  return { runWorkflow };
}
