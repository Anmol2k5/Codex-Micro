import { create } from "zustand";
import { bridge } from "../api/bridge";
import {
  DEFAULT_SETTINGS,
  clearSettings,
  loadSettings,
  saveSettings,
  type AppSettings,
} from "../features/settings/settings";
import { selectedPageIndex } from "../features/threads/paging";
import { deriveActionTarget } from "../features/threads/targeting";
import type {
  ActionResult,
  CapabilitySet,
  CodexAction,
  ConnectionState,
  ThreadSummary,
} from "../types/codex";

const initialSettings = loadSettings();

const emptyCapabilities: CapabilitySet = {
  canFocusApp: false,
  canListThreads: false,
  canSelectThread: false,
  canCreateThread: false,
  canReviewChanges: false,
  canApprove: false,
  canReject: false,
  canDiscardChanges: false,
  canSubmitPrompt: false,
  canStartSystemDictation: false,
  canReadReasoningOptions: false,
  canSetReasoningLevel: false,
};

export interface ActionHistoryEntry extends ActionResult {
  recordedAt: number;
}

interface MicroDeckState {
  connectionState: ConnectionState;
  capabilities: CapabilitySet;
  threads: ThreadSummary[];
  selectedThreadId: string | null;
  followActive: boolean;
  reasoningLevel: string;
  reasoningOptions: string[];
  lastActionResult: ActionResult | null;
  actionHistory: ActionHistoryEntry[];
  controllerPage: number;
  busyAction: CodexAction["type"] | null;
  error: string | null;
  settings: AppSettings;
  refresh: () => Promise<void>;
  setSelectedThread: (threadId: string) => void;
  setFollowActive: (value: boolean) => void;
  setControllerPage: (page: number) => void;
  setReasoningLevel: (value: string) => Promise<void>;
  executeAction: (action: CodexAction) => Promise<ActionResult | null>;
  updateSetting: <K extends keyof AppSettings>(key: K, value: AppSettings[K]) => Promise<void>;
  completeOnboarding: () => Promise<void>;
  resetLocalData: () => Promise<void>;
}

export const useMicroDeckStore = create<MicroDeckState>((set, get) => ({
  connectionState: "notRunning",
  capabilities: emptyCapabilities,
  threads: [],
  selectedThreadId: null,
  followActive: initialSettings.followActiveThread,
  reasoningLevel: "Medium",
  reasoningOptions: [],
  lastActionResult: null,
  actionHistory: [],
  controllerPage: 0,
  busyAction: null,
  error: null,
  settings: initialSettings,

  refresh: async () => {
    try {
      const [connectionState, capabilities, threads, reasoningOptions] = await Promise.all([
        bridge.getConnectionState(),
        bridge.getCapabilities(),
        bridge.listThreads(),
        bridge.getReasoningOptions(),
      ]);

      set((state) => {
        const selectedThreadId =
          state.selectedThreadId && threads.some((thread) => thread.id === state.selectedThreadId)
            ? state.selectedThreadId
            : threads.find((thread) => thread.isActive)?.id ?? threads[0]?.id ?? null;

        return {
          connectionState,
          capabilities,
          threads,
          reasoningOptions,
          selectedThreadId,
          controllerPage: selectedPageIndex(threads, selectedThreadId, 4),
          error: null,
        };
      });
    } catch (error) {
      set({
        connectionState: "error",
        error: error instanceof Error ? error.message : String(error),
      });
    }
  },

  setSelectedThread: (selectedThreadId) =>
    set((state) => ({
      selectedThreadId,
      controllerPage: selectedPageIndex(state.threads, selectedThreadId, 4),
    })),

  setFollowActive: (followActive) => {
    const settings = { ...get().settings, followActiveThread: followActive };
    saveSettings(settings);
    set({ followActive, settings });
  },

  setControllerPage: (controllerPage) => set({ controllerPage }),

  setReasoningLevel: async (value) => {
    if (!get().reasoningOptions.includes(value)) return;
    const result = await get().executeAction({
      type: "setReasoningLevel",
      payload: { value },
    });
    if (result?.outcome === "succeeded") set({ reasoningLevel: value });
  },

  executeAction: async (action) => {
    const state = get();
    const target = deriveActionTarget({
      followActive: state.followActive,
      selectedThreadId: state.selectedThreadId,
      threads: state.threads,
    });

    set({ busyAction: action.type, error: null });
    try {
      const result = await bridge.executeAction(action, target);
      set((current) => ({
        lastActionResult: result,
        busyAction: null,
        actionHistory: [
          ...current.actionHistory,
          { ...result, recordedAt: Date.now() },
        ].slice(-50),
      }));
      if (
        result.outcome === "succeeded" &&
        (action.type === "selectThread" || action.type === "newThread")
      ) {
        await get().refresh();
      }
      return result;
    } catch (error) {
      set({
        busyAction: null,
        error: error instanceof Error ? error.message : String(error),
      });
      return null;
    }
  },

  updateSetting: async (key, value) => {
    const settings = { ...get().settings, [key]: value };
    saveSettings(settings);
    set({ settings });
    if (key === "alwaysOnTop") {
      try {
        await bridge.setAlwaysOnTop(Boolean(value));
      } catch (error) {
        set({ error: error instanceof Error ? error.message : String(error) });
      }
    }
  },

  completeOnboarding: async () => {
    await get().updateSetting("onboardingComplete", true);
  },

  resetLocalData: async () => {
    clearSettings();
    saveSettings(DEFAULT_SETTINGS);
    set({
      settings: { ...DEFAULT_SETTINGS },
      followActive: DEFAULT_SETTINGS.followActiveThread,
      actionHistory: [],
      lastActionResult: null,
      controllerPage: 0,
    });
    try {
      await bridge.setAlwaysOnTop(DEFAULT_SETTINGS.alwaysOnTop);
    } catch {
      // Reset remains local even if window controls are unavailable in browser/test mode.
    }
  },
}));
