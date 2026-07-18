import type { ConnectionState, ThreadStatus } from "../../types/codex";

const labels: Record<ConnectionState | ThreadStatus, string> = {
  notRunning: "Not running",
  runningNotFocused: "Running",
  connected: "Connected",
  codexModeNotDetected: "Codex mode not detected",
  permissionRequired: "Permission required",
  degraded: "Degraded",
  error: "Error",
  working: "Working",
  thinking: "Thinking",
  waitingForUser: "Waiting for you",
  waitingForApproval: "Approval needed",
  completed: "Completed",
  failed: "Failed",
  idle: "Idle",
  unknown: "Unknown",
};

export function StatusBadge({ status }: { status: ConnectionState | ThreadStatus }) {
  return (
    <span className={`status-badge status-${status}`}>
      <span className="status-dot" aria-hidden="true" />
      {labels[status]}
    </span>
  );
}
