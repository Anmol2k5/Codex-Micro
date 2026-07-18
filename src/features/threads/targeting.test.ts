import { describe, expect, it } from "vitest";
import { actionTargetLabel, deriveActionTarget } from "./targeting";
import type { ThreadSummary } from "../../types/codex";

const threads: ThreadSummary[] = [
  { id: "a", title: "Active task", status: "working", isActive: true },
  { id: "b", title: "Selected task", status: "idle", isActive: false },
];

describe("deriveActionTarget", () => {
  it("targets the active thread while follow-active is enabled", () => {
    expect(
      deriveActionTarget({ followActive: true, selectedThreadId: "b", threads }),
    ).toEqual({ type: "activeThread" });
  });

  it("targets the selected thread while follow-active is disabled", () => {
    expect(
      deriveActionTarget({ followActive: false, selectedThreadId: "b", threads }),
    ).toEqual({ type: "selectedThread", threadId: "b" });
  });

  it("falls back to active when the selected thread is stale", () => {
    expect(
      deriveActionTarget({ followActive: false, selectedThreadId: "missing", threads }),
    ).toEqual({ type: "activeThread" });
  });
});

describe("actionTargetLabel", () => {
  it("shows the current active thread title", () => {
    expect(actionTargetLabel({ type: "activeThread" }, threads)).toBe("Active: Active task");
  });

  it("shows the selected thread title", () => {
    expect(
      actionTargetLabel({ type: "selectedThread", threadId: "b" }, threads),
    ).toBe("Selected: Selected task");
  });
});
