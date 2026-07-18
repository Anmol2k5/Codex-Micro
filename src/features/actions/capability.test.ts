import { describe, expect, it } from "vitest";
import { isActionAvailable } from "./capability";
import type { CapabilitySet } from "../../types/codex";

const capabilities: CapabilitySet = {
  canFocusApp: true,
  canListThreads: false,
  canSelectThread: false,
  canCreateThread: false,
  canReviewChanges: true,
  canApprove: false,
  canReject: false,
  canDiscardChanges: false,
  canSubmitPrompt: false,
  canStartSystemDictation: false,
  canReadReasoningOptions: false,
  canSetReasoningLevel: false,
};

describe("isActionAvailable", () => {
  it("maps focus and review actions to their capabilities", () => {
    expect(isActionAvailable({ type: "focusApp" }, capabilities)).toBe(true);
    expect(isActionAvailable({ type: "reviewChanges" }, capabilities)).toBe(true);
  });

  it("maps unsupported actions to false", () => {
    expect(isActionAvailable({ type: "approve" }, capabilities)).toBe(false);
    expect(isActionAvailable({ type: "newThread" }, capabilities)).toBe(false);
  });

  it("requires thread selection capability for select-thread", () => {
    expect(
      isActionAvailable(
        { type: "selectThread", payload: { threadId: "thread-1" } },
        capabilities,
      ),
    ).toBe(false);
  });
});
