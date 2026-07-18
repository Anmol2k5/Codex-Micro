import type { ChangeEvent, CSSProperties } from "react";
import {
  AppWindow,
  Check,
  ChevronLeft,
  ChevronRight,
  CircleX,
  FileDiff,
  Focus,
  Mic,
  Minus,
  Plus,
  RotateCcw,
  Settings2,
  Sparkles,
} from "lucide-react";
import { bridge } from "../../api/bridge";
import { pageCount, pageThreads } from "../../features/threads/paging";
import { actionTargetLabel, deriveActionTarget } from "../../features/threads/targeting";
import { useMicroDeckStore } from "../../state/useMicroDeckStore";
import type { CodexAction } from "../../types/codex";
import { StatusBadge } from "../common/StatusBadge";
import { ActionKey } from "./ActionKey";

export function FloatingController() {
  const {
    connectionState,
    capabilities,
    threads,
    selectedThreadId,
    followActive,
    reasoningLevel,
    reasoningOptions,
    controllerPage,
    settings,
    busyAction,
    setSelectedThread,
    setFollowActive,
    setControllerPage,
    setReasoningLevel,
    executeAction,
  } = useMicroDeckStore();

  const visibleThreads = pageThreads(threads, controllerPage, 4);
  const totalPages = pageCount(threads.length, 4);
  const target = deriveActionTarget({ followActive, selectedThreadId, threads });
  const targetLabel = actionTargetLabel(target, threads);
  const run = (action: CodexAction) => void executeAction(action);

  const unavailable = (available: boolean, name: string) =>
    available ? undefined : `${name} is not available in the detected Codex UI.`;

  const dialIndex = Math.max(0, reasoningOptions.indexOf(reasoningLevel));
  const shellStyle: CSSProperties = {
    opacity: settings.controllerOpacity,
    transform: `scale(${settings.controllerScale})`,
  };

  return (
    <section
      className="microdeck-shell"
      style={shellStyle}
      aria-label="MicroDeck floating controller"
    >
      <header className="microdeck-header" data-tauri-drag-region>
        <div data-tauri-drag-region>
          <p className="eyebrow" data-tauri-drag-region>MICRODECK</p>
          <StatusBadge status={connectionState} />
        </div>
        <div className="header-actions">
          <button
            className="icon-button"
            type="button"
            aria-label="Open dashboard settings"
            title="Open settings in the MicroDeck dashboard"
            onClick={() => void bridge.showWindow("main")}
          >
            <Settings2 size={18} />
          </button>
          <button
            className="icon-button"
            type="button"
            aria-label="Hide controller"
            title="Hide floating controller"
            onClick={() => void bridge.hideCurrentWindow()}
          >
            <Minus size={18} />
          </button>
        </div>
      </header>

      <div className="thread-pager-row">
        <span>Threads</span>
        <div className="thread-pager-controls">
          <button
            type="button"
            aria-label="Previous thread page"
            onClick={() => setControllerPage(Math.max(0, controllerPage - 1))}
            disabled={controllerPage <= 0}
          >
            <ChevronLeft size={14} />
          </button>
          <span>{Math.min(controllerPage + 1, totalPages)}/{totalPages}</span>
          <button
            type="button"
            aria-label="Next thread page"
            onClick={() => setControllerPage(Math.min(totalPages - 1, controllerPage + 1))}
            disabled={controllerPage >= totalPages - 1}
          >
            <ChevronRight size={14} />
          </button>
        </div>
      </div>

      <div className="thread-strip" aria-label="Codex threads">
        {visibleThreads.length > 0 ? visibleThreads.map((thread, index) => {
          const selected = thread.id === selectedThreadId;
          const absoluteIndex = controllerPage * 4 + index + 1;
          return (
            <button
              key={thread.id}
              type="button"
              className={`thread-slot ${selected ? "thread-slot-selected" : ""}`}
              onClick={() => setSelectedThread(thread.id)}
              onDoubleClick={() =>
                run({ type: "selectThread", payload: { threadId: thread.id } })
              }
              title={`${thread.title}${thread.project ? ` — ${thread.project}` : ""}`}
            >
              <span className={`thread-led thread-${thread.status}`} aria-hidden="true" />
              <span className="thread-number">{absoluteIndex}</span>
              <span className="thread-title">{thread.title}</span>
              {thread.isActive ? <span className="active-mark" aria-label="Active thread">A</span> : null}
            </button>
          );
        }) : (
          <div className="empty-thread-strip">No accessible Codex threads detected yet.</div>
        )}
      </div>

      <div className="target-row">
        <span className="target-label" title={targetLabel}>{targetLabel}</span>
        <label className="toggle-label">
          <input
            type="checkbox"
            checked={followActive}
            onChange={(event: ChangeEvent<HTMLInputElement>) => setFollowActive(event.target.checked)}
          />
          <span>Follow active</span>
        </label>
      </div>

      <div className="controller-grid">
        <div className="action-grid">
          <ActionKey
            label="Focus Codex"
            icon={<Focus size={27} />}
            disabled={!capabilities.canFocusApp}
            busy={busyAction === "focusApp"}
            title={unavailable(capabilities.canFocusApp, "Focus Codex")}
            onClick={() => run({ type: "focusApp" })}
          />
          <ActionKey
            label="Review"
            icon={<FileDiff size={27} />}
            disabled={!capabilities.canReviewChanges}
            busy={busyAction === "reviewChanges"}
            title={unavailable(capabilities.canReviewChanges, "Review")}
            onClick={() => run({ type: "reviewChanges" })}
          />
          <ActionKey
            label="Approve"
            icon={<Check size={29} />}
            tone="positive"
            disabled={!capabilities.canApprove}
            busy={busyAction === "approve"}
            title={unavailable(capabilities.canApprove, "Approve")}
            onClick={() => run({ type: "approve" })}
          />
          <ActionKey
            label="Reject"
            icon={<CircleX size={27} />}
            tone="danger"
            disabled={!capabilities.canReject}
            busy={busyAction === "reject"}
            title={unavailable(capabilities.canReject, "Reject")}
            onClick={() => run({ type: "reject" })}
          />
          <ActionKey
            label="New thread"
            icon={<Plus size={29} />}
            disabled={!capabilities.canCreateThread}
            busy={busyAction === "newThread"}
            title={unavailable(capabilities.canCreateThread, "New thread")}
            onClick={() => run({ type: "newThread" })}
          />
          <ActionKey
            label="Switch"
            icon={<AppWindow size={26} />}
            disabled={!capabilities.canSelectThread || !selectedThreadId}
            busy={busyAction === "selectThread"}
            title={unavailable(capabilities.canSelectThread, "Thread switching")}
            onClick={() => {
              if (selectedThreadId) run({ type: "selectThread", payload: { threadId: selectedThreadId } });
            }}
          />
          <ActionKey
            label="Voice"
            icon={<Mic size={26} />}
            disabled={!capabilities.canStartSystemDictation}
            busy={busyAction === "startSystemDictation"}
            title={unavailable(capabilities.canStartSystemDictation, "Voice dictation")}
            onClick={() => run({ type: "startSystemDictation" })}
          />
          <ActionKey
            label="Discard"
            icon={<RotateCcw size={26} />}
            disabled={!capabilities.canDiscardChanges}
            busy={busyAction === "discardChanges"}
            title={unavailable(capabilities.canDiscardChanges, "Discard changes")}
            onClick={() => {
              const shouldRun = !settings.confirmDiscardChanges || window.confirm(
                "Discard changes in the targeted Codex thread? This may permanently remove work.",
              );
              if (shouldRun) run({ type: "discardChanges" });
            }}
          />
        </div>

        <div className="reasoning-panel">
          <div className="dial-wrap">
            <button
              type="button"
              className="dial-step"
              aria-label="Previous reasoning level"
              onClick={() => {
                const next = reasoningOptions[Math.max(0, dialIndex - 1)];
                if (next) void setReasoningLevel(next);
              }}
              disabled={!capabilities.canSetReasoningLevel || dialIndex <= 0}
            >
              <ChevronLeft size={18} />
            </button>
            <div className="reasoning-dial" aria-label={`Reasoning ${reasoningLevel}`}>
              <Sparkles size={24} />
              <span>{reasoningOptions.length > 0 ? reasoningLevel : "N/A"}</span>
            </div>
            <button
              type="button"
              className="dial-step"
              aria-label="Next reasoning level"
              onClick={() => {
                const next = reasoningOptions[Math.min(reasoningOptions.length - 1, dialIndex + 1)];
                if (next) void setReasoningLevel(next);
              }}
              disabled={
                !capabilities.canSetReasoningLevel ||
                reasoningOptions.length === 0 ||
                dialIndex >= reasoningOptions.length - 1
              }
            >
              <ChevronRight size={18} />
            </button>
          </div>
          <span className="reasoning-caption">Reasoning</span>
        </div>
      </div>

      <footer className="microdeck-footer">
        <span>Unofficial companion for Codex</span>
        <span className="footer-dot">•</span>
        <span>Local-first</span>
      </footer>
    </section>
  );
}
