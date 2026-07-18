import type { CapabilitySet, CodexAction } from "../../types/codex";

export function isActionAvailable(action: CodexAction, capabilities: CapabilitySet): boolean {
  switch (action.type) {
    case "focusApp":
      return capabilities.canFocusApp;
    case "newThread":
      return capabilities.canCreateThread;
    case "reviewChanges":
      return capabilities.canReviewChanges;
    case "approve":
      return capabilities.canApprove;
    case "reject":
      return capabilities.canReject;
    case "discardChanges":
      return capabilities.canDiscardChanges;
    case "submitPrompt":
      return capabilities.canSubmitPrompt;
    case "startSystemDictation":
      return capabilities.canStartSystemDictation;
    case "selectThread":
      return capabilities.canSelectThread;
    case "setReasoningLevel":
      return capabilities.canSetReasoningLevel;
    case "openShortcutHelp":
      return true;
  }
}
