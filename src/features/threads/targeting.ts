import type { ActionTarget, ThreadSummary } from "../../types/codex";

export interface TargetingInput {
  followActive: boolean;
  selectedThreadId: string | null;
  threads: ThreadSummary[];
}

export function deriveActionTarget({
  followActive,
  selectedThreadId,
  threads,
}: TargetingInput): ActionTarget {
  const selectedExists = selectedThreadId
    ? threads.some((thread) => thread.id === selectedThreadId)
    : false;

  if (!followActive && selectedThreadId && selectedExists) {
    return { type: "selectedThread", threadId: selectedThreadId };
  }

  return { type: "activeThread" };
}

export function actionTargetLabel(
  target: ActionTarget,
  threads: ThreadSummary[],
): string {
  if (target.type === "activeThread") {
    const active = threads.find((thread) => thread.isActive);
    return active ? `Active: ${active.title}` : "Active thread";
  }

  const selected = threads.find((thread) => thread.id === target.threadId);
  return selected ? `Selected: ${selected.title}` : "Selected thread unavailable";
}
