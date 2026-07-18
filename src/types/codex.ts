export type ThreadId = string;

export type ConnectionState =
  | "notRunning"
  | "runningNotFocused"
  | "connected"
  | "codexModeNotDetected"
  | "permissionRequired"
  | "degraded"
  | "error";

export type ThreadStatus =
  | "working"
  | "thinking"
  | "waitingForUser"
  | "waitingForApproval"
  | "completed"
  | "failed"
  | "idle"
  | "unknown";

export interface ThreadSummary {
  id: ThreadId;
  title: string;
  project?: string | null;
  status: ThreadStatus;
  isActive: boolean;
  updatedAtMs?: number | null;
}

export type CodexAction =
  | { type: "focusApp" }
  | { type: "newThread" }
  | { type: "reviewChanges" }
  | { type: "approve" }
  | { type: "reject" }
  | { type: "discardChanges" }
  | { type: "submitPrompt"; payload: { text: string } }
  | { type: "startSystemDictation" }
  | { type: "selectThread"; payload: { threadId: ThreadId } }
  | { type: "setReasoningLevel"; payload: { value: string } }
  | { type: "openShortcutHelp" };

export type ActionTarget =
  | { type: "activeThread" }
  | { type: "selectedThread"; threadId: ThreadId };

export type ActionOutcome =
  | "succeeded"
  | "unsupported"
  | "targetNotFound"
  | "permissionDenied"
  | "timedOut"
  | "ambiguous"
  | "appNotRunning"
  | "codexModeNotDetected"
  | "failed";

export interface ActionResult {
  action: CodexAction;
  target: ActionTarget;
  outcome: ActionOutcome;
  userMessage: string;
  diagnosticCode: string;
  elapsedMs: number;
}

export interface CapabilitySet {
  canFocusApp: boolean;
  canListThreads: boolean;
  canSelectThread: boolean;
  canCreateThread: boolean;
  canReviewChanges: boolean;
  canApprove: boolean;
  canReject: boolean;
  canDiscardChanges: boolean;
  canSubmitPrompt: boolean;
  canStartSystemDictation: boolean;
  canReadReasoningOptions: boolean;
  canSetReasoningLevel: boolean;
}
