import { bridge } from "../../api/bridge";
import {
  CheckCircle2,
  Download,
  Focus,
  PanelsTopLeft,
  RefreshCw,
  ShieldCheck,
  Trash2,
} from "lucide-react";
import { buildDiagnosticReport } from "../../features/diagnostics/report";
import { useMicroDeckStore } from "../../state/useMicroDeckStore";
import { StatusBadge } from "../common/StatusBadge";

function capabilityLabel(name: string): string {
  return name.replace(/^can/, "").replace(/[A-Z]/g, (match) => ` ${match}`).trim();
}

function downloadText(filename: string, text: string): void {
  const blob = new Blob([text], { type: "application/json" });
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = filename;
  anchor.click();
  URL.revokeObjectURL(url);
}

export function Dashboard() {
  const {
    connectionState,
    capabilities,
    threads,
    selectedThreadId,
    lastActionResult,
    actionHistory,
    error,
    settings,
    refresh,
    setSelectedThread,
    executeAction,
    updateSetting,
    completeOnboarding,
    resetLocalData,
  } = useMicroDeckStore();

  const capabilityCount = Object.values(capabilities).filter(Boolean).length;
  const totalCapabilities = Object.keys(capabilities).length;

  const exportDiagnostics = () => {
    const report = buildDiagnosticReport({
      version: "0.1.0",
      connectionState,
      capabilities,
      actionHistory: actionHistory.map((entry) => ({
        action: entry.action.type,
        outcome: entry.outcome,
        diagnosticCode: entry.diagnosticCode,
        elapsedMs: entry.elapsedMs,
        userMessage: entry.userMessage,
      })),
    });
    downloadText("microdeck-diagnostics.json", report);
  };

  return (
    <main className="dashboard">
      <section className="dashboard-hero">
        <div>
          <p className="eyebrow">CODEX CONTROL SURFACE</p>
          <h1>MicroDeck</h1>
          <p className="hero-copy">
            A Windows-first software control deck for supported Codex desktop actions. MicroDeck
            discovers capabilities at runtime and disables controls it cannot verify.
          </p>
          <div className="hero-actions">
            <button
              type="button"
              className="primary-button"
              disabled={!capabilities.canFocusApp}
              onClick={() => void executeAction({ type: "focusApp" })}
            >
              <Focus size={17} /> Focus Codex
            </button>
            <button type="button" className="secondary-button" onClick={() => void refresh()}>
              <RefreshCw size={17} /> Refresh status
            </button>
            <button
              type="button"
              className="secondary-button"
              onClick={() => void bridge.showWindow("controller")}
            >
              <PanelsTopLeft size={17} /> Show controller
            </button>
          </div>
        </div>
        <div className="hero-status">
          <StatusBadge status={connectionState} />
          <strong>{capabilityCount}/{totalCapabilities}</strong>
          <span>capabilities detected</span>
        </div>
      </section>

      {!settings.onboardingComplete ? (
        <section className="onboarding-card">
          <div>
            <p className="eyebrow">FIRST RUN</p>
            <h2>Set up MicroDeck safely</h2>
            <p>
              Open the ChatGPT desktop app, switch to Codex, then refresh detection. The Windows
              adapter can locate and focus the app today; deeper controls stay disabled until their
              UI Automation selectors are verified on the installed version.
            </p>
          </div>
          <ol className="setup-steps">
            <li><CheckCircle2 size={16} /> Install or open ChatGPT for Windows.</li>
            <li><CheckCircle2 size={16} /> Open the Codex experience.</li>
            <li><CheckCircle2 size={16} /> Keep MicroDeck and ChatGPT at matching privilege levels.</li>
            <li><CheckCircle2 size={16} /> Refresh and test “Focus Codex”.</li>
          </ol>
          <button type="button" className="primary-button" onClick={() => void completeOnboarding()}>
            I understand — finish setup
          </button>
        </section>
      ) : null}

      <section className="dashboard-grid">
        <article className="panel">
          <div className="panel-heading">
            <div>
              <p className="eyebrow">THREADS</p>
              <h2>Target a task</h2>
            </div>
            <span className="panel-count">{threads.length}</span>
          </div>
          <div className="thread-list">
            {threads.length ? threads.map((thread) => (
              <button
                type="button"
                key={thread.id}
                className={`thread-row ${selectedThreadId === thread.id ? "thread-row-selected" : ""}`}
                onClick={() => setSelectedThread(thread.id)}
              >
                <div>
                  <strong>{thread.title}</strong>
                  <span>{thread.project ?? "No project"}</span>
                </div>
                <div className="thread-row-meta">
                  {thread.isActive ? <span className="pill">Active</span> : null}
                  <StatusBadge status={thread.status} />
                </div>
              </button>
            )) : (
              <p className="empty-copy">
                No threads are exposed by the current adapter. This is expected until Codex thread
                selectors are verified.
              </p>
            )}
          </div>
        </article>

        <article className="panel">
          <div className="panel-heading">
            <div>
              <p className="eyebrow">AUTOMATION</p>
              <h2>Capability matrix</h2>
            </div>
          </div>
          <div className="capability-list">
            {Object.entries(capabilities).map(([name, available]) => (
              <div className="capability-row" key={name}>
                <span>{capabilityLabel(name)}</span>
                <span className={available ? "capability-yes" : "capability-no"}>
                  {available ? "Available" : "Unavailable"}
                </span>
              </div>
            ))}
          </div>
          <div className="integrity-note">
            <ShieldCheck size={18} />
            <p>
              Unavailable means unavailable. MicroDeck does not silently fall back to hard-coded
              screen coordinates or claim that an action succeeded without confirmation.
            </p>
          </div>
        </article>

        <article className="panel activity-panel">
          <div className="panel-heading">
            <div>
              <p className="eyebrow">LATEST RESULT</p>
              <h2>Action diagnostics</h2>
            </div>
            <button type="button" className="small-button" onClick={exportDiagnostics}>
              <Download size={15} /> Export redacted report
            </button>
          </div>
          {error ? <div className="error-banner">{error}</div> : null}
          {lastActionResult ? (
            <dl className="diagnostic-grid">
              <div><dt>Action</dt><dd>{lastActionResult.action.type}</dd></div>
              <div><dt>Outcome</dt><dd>{lastActionResult.outcome}</dd></div>
              <div><dt>Diagnostic</dt><dd>{lastActionResult.diagnosticCode}</dd></div>
              <div><dt>Elapsed</dt><dd>{lastActionResult.elapsedMs} ms</dd></div>
              <div className="diagnostic-message"><dt>Message</dt><dd>{lastActionResult.userMessage}</dd></div>
            </dl>
          ) : (
            <p className="empty-copy">Run an action from the controller to see an honest result here.</p>
          )}

          {actionHistory.length > 0 ? (
            <div className="history-list" aria-label="Recent action history">
              {actionHistory.slice(-6).reverse().map((entry) => (
                <div className="history-row" key={`${entry.recordedAt}-${entry.action.type}`}>
                  <span>{entry.action.type}</span>
                  <code>{entry.diagnosticCode}</code>
                  <strong>{entry.outcome}</strong>
                </div>
              ))}
            </div>
          ) : null}
        </article>

        <article className="panel settings-panel" id="settings-panel">
          <div className="panel-heading">
            <div>
              <p className="eyebrow">SETTINGS</p>
              <h2>Controller behavior</h2>
            </div>
          </div>

          <label className="setting-row">
            <span><strong>Always on top</strong><small>Keep the controller above other windows.</small></span>
            <input
              type="checkbox"
              checked={settings.alwaysOnTop}
              onChange={(event) => void updateSetting("alwaysOnTop", event.target.checked)}
            />
          </label>
          <label className="setting-row">
            <span><strong>Confirm discard</strong><small>Always confirm potentially destructive discard actions.</small></span>
            <input
              type="checkbox"
              checked={settings.confirmDiscardChanges}
              onChange={(event) => void updateSetting("confirmDiscardChanges", event.target.checked)}
            />
          </label>
          <label className="setting-row">
            <span><strong>Reduced motion</strong><small>Disable non-essential animation.</small></span>
            <input
              type="checkbox"
              checked={settings.reducedMotion}
              onChange={(event) => void updateSetting("reducedMotion", event.target.checked)}
            />
          </label>
          <label className="range-setting">
            <span>Controller scale <strong>{Math.round(settings.controllerScale * 100)}%</strong></span>
            <input
              type="range"
              min="0.8"
              max="1.4"
              step="0.05"
              value={settings.controllerScale}
              onChange={(event) => void updateSetting("controllerScale", Number(event.target.value))}
            />
          </label>
          <label className="range-setting">
            <span>Controller opacity <strong>{Math.round(settings.controllerOpacity * 100)}%</strong></span>
            <input
              type="range"
              min="0.55"
              max="1"
              step="0.05"
              value={settings.controllerOpacity}
              onChange={(event) => void updateSetting("controllerOpacity", Number(event.target.value))}
            />
          </label>
          <button type="button" className="danger-button" onClick={() => void resetLocalData()}>
            <Trash2 size={16} /> Clear local MicroDeck data
          </button>
        </article>

        <article className="panel privacy-panel">
          <div className="panel-heading">
            <div>
              <p className="eyebrow">PRIVACY & LIMITATIONS</p>
              <h2>Local-first by design</h2>
            </div>
          </div>
          <p>
            MicroDeck requires no OpenAI API key and does not persist conversation content. The
            diagnostics exporter stores only capability state, action outcomes, timings, and
            redacted messages.
          </p>
          <p>
            Live review, approval, thread discovery, voice, and reasoning controls remain disabled
            until the installed ChatGPT/Codex accessibility tree has been inspected and the matching
            selectors have been verified. This is a deliberate reliability boundary, not a missing
            error handler.
          </p>
        </article>
      </section>

      <footer className="dashboard-footer">
        MicroDeck is an unofficial companion for Codex and is not affiliated with or endorsed by OpenAI.
      </footer>
    </main>
  );
}
