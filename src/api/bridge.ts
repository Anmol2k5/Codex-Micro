import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow, Window } from "@tauri-apps/api/window";
import type {
  ActionResult,
  ActionTarget,
  CapabilitySet,
  CodexAction,
  ConnectionState,
  ThreadSummary,
} from "../types/codex";

export interface MicroDeckBridge {
  getConnectionState(): Promise<ConnectionState>;
  getCapabilities(): Promise<CapabilitySet>;
  listThreads(): Promise<ThreadSummary[]>;
  getActiveThread(): Promise<ThreadSummary | null>;
  getReasoningOptions(): Promise<string[]>;
  executeAction(action: CodexAction, target: ActionTarget): Promise<ActionResult>;
  setAlwaysOnTop(value: boolean): Promise<void>;
  showWindow(label: "main" | "controller"): Promise<void>;
  hideCurrentWindow(): Promise<void>;
}

let browserThreads: ThreadSummary[] = [
  {
    id: "thread-1",
    title: "Implement auth flow",
    project: "MicroDeck",
    status: "working",
    isActive: true,
  },
  {
    id: "thread-2",
    title: "Review Windows adapter",
    project: "MicroDeck",
    status: "waitingForApproval",
    isActive: false,
  },
  {
    id: "thread-3",
    title: "Polish controller UI",
    project: "MicroDeck",
    status: "completed",
    isActive: false,
  },
  {
    id: "thread-4",
    title: "Write selector tests",
    project: "MicroDeck",
    status: "idle",
    isActive: false,
  },
  {
    id: "thread-5",
    title: "Prepare Windows installer",
    project: "MicroDeck",
    status: "thinking",
    isActive: false,
  },
];

const browserCapabilities: CapabilitySet = {
  canFocusApp: true,
  canListThreads: true,
  canSelectThread: true,
  canCreateThread: true,
  canReviewChanges: true,
  canApprove: true,
  canReject: true,
  canDiscardChanges: false,
  canSubmitPrompt: true,
  canStartSystemDictation: false,
  canReadReasoningOptions: true,
  canSetReasoningLevel: true,
};

function isTauriRuntime(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

function mockResult(
  action: CodexAction,
  target: ActionTarget,
  outcome: ActionResult["outcome"],
  diagnosticCode: string,
  userMessage: string,
): ActionResult {
  return { action, target, outcome, diagnosticCode, userMessage, elapsedMs: 35 };
}

const tauriBridge: MicroDeckBridge = {
  getConnectionState: () => invoke("get_connection_state"),
  getCapabilities: () => invoke("get_capabilities"),
  listThreads: () => invoke("list_threads"),
  getActiveThread: () => invoke("get_active_thread"),
  getReasoningOptions: () => invoke("get_reasoning_options"),
  executeAction: (action, target) =>
    invoke("execute_action", { request: { action, target } }),
  async setAlwaysOnTop(value) {
    await getCurrentWindow().setAlwaysOnTop(value);
  },
  async showWindow(label) {
    const target = await Window.getByLabel(label);
    if (target) {
      await target.show();
      await target.setFocus();
    }
  },
  async hideCurrentWindow() {
    await getCurrentWindow().hide();
  },
};

const browserBridge: MicroDeckBridge = {
  async getConnectionState() {
    return "connected";
  },
  async getCapabilities() {
    return browserCapabilities;
  },
  async listThreads() {
    return structuredClone(browserThreads);
  },
  async getActiveThread() {
    return structuredClone(browserThreads.find((thread) => thread.isActive) ?? null);
  },
  async getReasoningOptions() {
    return ["Low", "Medium", "High"];
  },
  async executeAction(action, target) {
    await new Promise((resolve) => window.setTimeout(resolve, 80));

    if (action.type === "selectThread") {
      const exists = browserThreads.some((thread) => thread.id === action.payload.threadId);
      if (!exists) {
        return mockResult(
          action,
          target,
          "targetNotFound",
          "MOCK_THREAD_NOT_FOUND",
          "The selected demo thread no longer exists.",
        );
      }
      browserThreads = browserThreads.map((thread) => ({
        ...thread,
        isActive: thread.id === action.payload.threadId,
      }));
    }

    if (action.type === "newThread") {
      browserThreads = browserThreads.map((thread) => ({ ...thread, isActive: false }));
      browserThreads.push({
        id: `thread-${browserThreads.length + 1}`,
        title: "New Codex task",
        project: "MicroDeck",
        status: "idle",
        isActive: true,
      });
    }

    if (action.type === "discardChanges" || action.type === "startSystemDictation") {
      return mockResult(
        action,
        target,
        "unsupported",
        "MOCK_ACTION_UNSUPPORTED",
        "This action is intentionally disabled in browser demo mode.",
      );
    }

    return mockResult(
      action,
      target,
      "succeeded",
      "MOCK_ACTION_SUCCEEDED",
      `Demo action ${action.type} completed.`,
    );
  },
  async setAlwaysOnTop() {},
  async showWindow() {},
  async hideCurrentWindow() {},
};

export const bridge: MicroDeckBridge = isTauriRuntime() ? tauriBridge : browserBridge;
