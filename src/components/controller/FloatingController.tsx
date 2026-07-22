import type { CSSProperties } from "react";
import {
  ArrowUp,
  Check,
  CheckCircle2,
  ChevronLeft,
  ChevronRight,
  Cloud,
  FileDiff,
  Focus,
  GitBranch,
  Mic,
  Minus,
  Plus,
  RotateCcw,
  Settings2,
  Sparkles,
  XCircle,
  Zap,
} from "lucide-react";
import { bridge } from "../../api/bridge";
import { pageCount, pageThreads } from "../../features/threads/paging";
import { actionTargetLabel, deriveActionTarget } from "../../features/threads/targeting";
import { useMicroDeckStore } from "../../state/useMicroDeckStore";
import type { CodexAction } from "../../types/codex";
import { StatusBadge } from "../common/StatusBadge";
import { HardwareKey } from "./HardwareKey";
import { HardwareKnob } from "./HardwareKnob";

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

  const shellStyle: CSSProperties = {
    opacity: settings.controllerOpacity,
    transform: `scale(${settings.controllerScale})`,
  };

  return (
    <section
      className="hardware-deck-shell"
      style={shellStyle}
      aria-label="CODEX MICRO hardware control deck"
    >
      {/* Top Outer Header */}
      <header className="deck-outer-header" data-tauri-drag-region>
        <div className="deck-brand" data-tauri-drag-region>
          <span className="brand-primary">CODEX</span>
          <span className="brand-secondary">MICRO</span>
        </div>
        <div className="deck-outer-actions">
          <span
            className={`status-dot status-dot-${connectionState}`}
            title={`Status: ${connectionState}`}
          />
          <button
            className="deck-icon-btn"
            type="button"
            aria-label="Open settings dashboard"
            title="Open settings in MicroDeck dashboard"
            onClick={() => void bridge.showWindow("main")}
          >
            <Settings2 size={19} />
          </button>
          <button
            className="deck-icon-btn"
            type="button"
            aria-label="Hide controller"
            title="Hide floating controller"
            onClick={() => void bridge.hideCurrentWindow()}
          >
            <Minus size={19} />
          </button>
        </div>
      </header>

      {/* Main Skeuomorphic Acrylic Hardware Enclosure */}
      <div className="acrylic-casing">
        {/* Corner Hex Screws */}
        <div className="corner-screw screw-top-left" />
        <div className="corner-screw screw-top-right" />
        <div className="corner-screw screw-bottom-left" />
        <div className="corner-screw screw-bottom-right" />

        {/* Plate Engravings */}
        <div className="engraving engraving-left">Work Louder | OpenAI 2026</div>
        <div className="engraving engraving-top">
          <ArrowUp size={14} />
        </div>
        <div className="engraving engraving-right">You can just build things</div>
        <div className="engraving engraving-bottom">Let's build</div>

        {/* Thread Pager Header Bar */}
        <div className="deck-thread-bar">
          <div className="thread-target-info" title={targetLabel}>
            <span className="target-dot" />
            <span className="target-text">{targetLabel}</span>
          </div>
          <div className="thread-pager-mini">
            <button
              type="button"
              onClick={() => setControllerPage(Math.max(0, controllerPage - 1))}
              disabled={controllerPage <= 0}
              aria-label="Previous page"
            >
              <ChevronLeft size={13} />
            </button>
            <span>{Math.min(controllerPage + 1, Math.max(1, totalPages))}/{Math.max(1, totalPages)}</span>
            <button
              type="button"
              onClick={() => setControllerPage(Math.min(totalPages - 1, controllerPage + 1))}
              disabled={controllerPage >= totalPages - 1}
              aria-label="Next page"
            >
              <ChevronRight size={13} />
            </button>
          </div>
        </div>

        {/* Hardware Control Matrix */}
        <div className="deck-grid">
          {/* Row 1 */}
          <div className="grid-cell">
            <HardwareKnob
              level={reasoningLevel}
              options={reasoningOptions.length > 0 ? reasoningOptions : ["Low", "Medium", "High", "Extra"]}
              onChange={(lvl) => void setReasoningLevel(lvl)}
              disabled={!capabilities.canSetReasoningLevel}
            />
          </div>

          <div className="grid-cell">
            <HardwareKey
              variant="translucent"
              icon={<Plus size={18} />}
              label="Thread 1"
              badge={visibleThreads[0] ? 1 : undefined}
              active={visibleThreads[0]?.id === selectedThreadId}
              disabled={!visibleThreads[0]}
              title={visibleThreads[0] ? visibleThreads[0].title : "Empty thread slot"}
              onClick={() => {
                if (visibleThreads[0]) setSelectedThread(visibleThreads[0].id);
              }}
            />
          </div>

          <div className="grid-cell">
            <HardwareKey
              variant="translucent"
              icon={<Plus size={18} />}
              label="Thread 2"
              badge={visibleThreads[1] ? 2 : undefined}
              active={visibleThreads[1]?.id === selectedThreadId}
              disabled={!visibleThreads[1]}
              title={visibleThreads[1] ? visibleThreads[1].title : "Empty thread slot"}
              onClick={() => {
                if (visibleThreads[1]) setSelectedThread(visibleThreads[1].id);
              }}
            />
          </div>

          <div className="grid-cell">
            <HardwareKey
              variant="dashed"
              icon={<CheckCircle2 size={20} />}
              label="Toggle Mode"
              title="Dashed toggle & trackball key"
              active={followActive}
              onClick={() => setFollowActive(!followActive)}
            />
          </div>

          {/* Row 2: Thread slots */}
          <div className="grid-cell">
            <HardwareKey
              variant="translucent"
              icon={<Plus size={18} />}
              badge={visibleThreads[2] ? 3 : undefined}
              active={visibleThreads[2]?.id === selectedThreadId}
              disabled={!visibleThreads[2]}
              title={visibleThreads[2] ? visibleThreads[2].title : "Empty thread slot"}
              onClick={() => {
                if (visibleThreads[2]) setSelectedThread(visibleThreads[2].id);
              }}
            />
          </div>

          <div className="grid-cell">
            <HardwareKey
              variant="translucent"
              icon={<Plus size={18} />}
              badge={visibleThreads[3] ? 4 : undefined}
              active={visibleThreads[3]?.id === selectedThreadId}
              disabled={!visibleThreads[3]}
              title={visibleThreads[3] ? visibleThreads[3].title : "Empty thread slot"}
              onClick={() => {
                if (visibleThreads[3]) setSelectedThread(visibleThreads[3].id);
              }}
            />
          </div>

          <div className="grid-cell">
            <HardwareKey
              variant="translucent"
              icon={<Plus size={18} />}
              label="New"
              disabled={!capabilities.canCreateThread}
              busy={busyAction === "newThread"}
              title={unavailable(capabilities.canCreateThread, "New thread")}
              onClick={() => run({ type: "newThread" })}
            />
          </div>

          <div className="grid-cell">
            <HardwareKey
              variant="translucent"
              icon={<Plus size={18} />}
              label="Review"
              disabled={!capabilities.canReviewChanges}
              busy={busyAction === "reviewChanges"}
              title={unavailable(capabilities.canReviewChanges, "Review changes")}
              onClick={() => run({ type: "reviewChanges" })}
            />
          </div>

          {/* Row 3: Solid Tactile Action Keys */}
          <div className="grid-cell">
            <HardwareKey
              variant="solid"
              icon={<Zap size={22} />}
              label="Focus Codex"
              disabled={!capabilities.canFocusApp}
              busy={busyAction === "focusApp"}
              title={unavailable(capabilities.canFocusApp, "Focus Codex")}
              onClick={() => run({ type: "focusApp" })}
            />
          </div>

          <div className="grid-cell">
            <HardwareKey
              variant="solid"
              tone="positive"
              icon={<Check size={24} />}
              label="Approve"
              disabled={!capabilities.canApprove}
              busy={busyAction === "approve"}
              title={unavailable(capabilities.canApprove, "Approve")}
              onClick={() => run({ type: "approve" })}
            />
          </div>

          <div className="grid-cell">
            <HardwareKey
              variant="solid"
              tone="danger"
              icon={<XCircle size={22} />}
              label="Reject"
              disabled={!capabilities.canReject}
              busy={busyAction === "reject"}
              title={unavailable(capabilities.canReject, "Reject")}
              onClick={() => run({ type: "reject" })}
            />
          </div>

          <div className="grid-cell">
            <HardwareKey
              variant="solid"
              icon={<GitBranch size={22} />}
              label="Switch / Branch"
              disabled={!capabilities.canSelectThread}
              busy={busyAction === "selectThread"}
              title={unavailable(capabilities.canSelectThread, "Switch thread")}
              onClick={() => {
                if (selectedThreadId) run({ type: "selectThread", payload: { threadId: selectedThreadId } });
              }}
            />
          </div>
        </div>

        {/* Row 4: Bottom Special Controls */}
        <div className="deck-bottom-row">
          {/* LED Indicators & Black Knob */}
          <div className="deck-led-cluster" title="System LED status indicators">
            <div className="led-bars">
              <span className="led-bar led-gold" />
              <span className="led-bar led-orange" />
              <span className="led-bar led-green" />
            </div>
            <div className="black-trackball" />
          </div>

          {/* Center Wide Dictation Key */}
          <div className="deck-pill-cell">
            <HardwareKey
              variant="pill"
              icon={<Mic size={24} />}
              label="Voice Dictation"
              disabled={!capabilities.canStartSystemDictation}
              busy={busyAction === "startSystemDictation"}
              title={unavailable(capabilities.canStartSystemDictation, "Voice dictation")}
              onClick={() => run({ type: "startSystemDictation" })}
            />
          </div>

          {/* Right Thought Cloud Key */}
          <div className="deck-thought-cell">
            <HardwareKey
              variant="thought"
              icon={<Cloud size={24} />}
              label="Reasoning Detail"
              disabled={!capabilities.canSetReasoningLevel}
              title="Toggle reasoning detail level"
              onClick={() => {
                const next = reasoningOptions[(reasoningOptions.indexOf(reasoningLevel) + 1) % reasoningOptions.length] || "Medium";
                void setReasoningLevel(next);
              }}
            />
          </div>
        </div>
      </div>
    </section>
  );
}
