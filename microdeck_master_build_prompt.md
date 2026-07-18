# Master Build Prompt — MicroDeck for Codex

You are a senior Windows desktop engineer, Rust engineer, React/TypeScript engineer, accessibility-automation specialist, and product-quality QA lead.

Build a production-quality desktop application called **MicroDeck**: an unofficial software control surface for the **Codex mode inside the ChatGPT desktop app**. The product should recreate the usefulness and tactile simplicity of the Codex Micro hardware as a floating software controller, while using an original public-facing brand and visual identity.

The app is **Windows-first**, because Windows is the platform available for development and testing. However, the architecture must make a later macOS adapter possible without rewriting the shared UI, state, command, or settings layers.

Do not build only a visual mockup. Build the working application end to end.

---

## 1. Product definition

MicroDeck is a desktop companion that sits beside or above the ChatGPT desktop app while the user works in Codex mode.

It provides:

1. A compact, always-on-top floating controller inspired by a physical control deck.
2. A larger dashboard for threads, actions, diagnostics, settings, shortcuts, and recent activity.
3. One-tap controls for:
   - Open/focus ChatGPT Codex
   - New thread
   - Review changes
   - Approve the current approval request
   - Reject or decline the current approval request
   - Discard or undo changes only when that specific action is visibly available
   - Switch between threads
   - Submit a voice prompt
   - Change model/reasoning level when the current Codex UI exposes that option
4. Support for both:
   - Acting on the currently active Codex thread
   - Acting on a thread selected inside MicroDeck
5. Live connection and capability status.
6. Configurable global keyboard shortcuts.
7. Local-only settings and diagnostic logs.

The app must never pretend an action succeeded. Every action must return a typed result and display success, failure, timeout, unsupported, or ambiguous state.

---

## 2. Important product constraints

### 2.1 No private or invented API

Do not assume a private Codex API, WebSocket, local database schema, authentication token, or undocumented IPC protocol.

The first implementation must use supported operating-system interaction mechanisms:

1. Windows UI Automation as the primary mechanism.
2. Existing keyboard shortcuts as a secondary mechanism when they are known and verified at runtime.
3. User-assisted calibration only as an explicit last-resort fallback.

Do not use DLL injection, process-memory reading, credential extraction, session-token scraping, packet interception, or modification of ChatGPT/Codex files.

### 2.2 Target application migration

The product must support the current ChatGPT desktop application containing Codex mode and remain compatible with installations that still expose an older Codex-specific executable during migration.

Create configurable process/window matching rather than hardcoding one executable forever.

Initial candidates may include names such as:

- `ChatGPT.exe`
- `Codex.exe`

Treat these only as discovery candidates, not guaranteed permanent identifiers.

### 2.3 Brand and legal presentation

Use **MicroDeck** as the application name.

Public UI copy should say:

> Unofficial companion for Codex. Not affiliated with or endorsed by OpenAI.

Do not ship the official OpenAI wordmark, Codex Micro wordmark, official product photography, OpenAI Sans, or copied proprietary iconography unless the repository owner supplies assets they are authorized to use.

Create original icons and a visually related but distinct control-deck design.

### 2.4 Local-first privacy

The app must:

- Require no OpenAI API key.
- Read no project source files.
- Read no prompts or conversation text unless needed to identify UI state, and avoid persisting any such text.
- Make no network requests by default.
- Store settings and diagnostic metadata locally.
- Keep telemetry disabled unless a future explicit opt-in system is added.
- Redact window text from logs by default.
- Provide a “Clear local data” button.

---

## 3. Recommended technology stack

Use this stack unless the existing repository already has a well-justified equivalent:

### Desktop shell
- Tauri 2
- Rust stable
- React
- TypeScript with strict mode
- Vite

### Frontend
- React
- TypeScript
- Zustand for UI/application state
- CSS Modules or Tailwind CSS; choose one and use it consistently
- Lucide icons or original SVG icons
- React Testing Library
- Vitest

### Native backend
- Rust
- `windows` crate for Windows UI Automation and Win32 APIs
- Tauri commands/events for frontend-backend communication
- `serde` and `serde_json`
- `thiserror`
- `tracing` and `tracing-subscriber`
- Tauri plugins only where necessary:
  - global shortcut
  - store
  - single instance
  - autostart
  - notification
  - updater only after the core app is stable

Do not add a Node sidecar unless a concrete blocker requires it.

### Persistence
Use Tauri Store for:
- settings
- window position
- shortcut mappings
- selector profiles
- onboarding completion
- appearance preferences

Use a bounded local JSONL diagnostic log, with rotation and redaction. Do not add SQLite unless the product genuinely needs relational querying.

---

## 4. Architecture

Create clear boundaries so the macOS adapter can be added later.

Use this conceptual architecture:

```text
React UI
  |
  | Tauri commands and events
  v
Application Core
  |
  +-- Command Dispatcher
  +-- Capability Registry
  +-- Thread State Service
  +-- Settings Service
  +-- Diagnostic Service
  |
  v
AutomationAdapter trait
  |
  +-- WindowsAutomationAdapter
  +-- MockAutomationAdapter
  +-- MacAutomationAdapter (interface/stub only; no fake implementation)
  |
  v
Windows UI Automation + verified input fallback
  |
  v
ChatGPT desktop app / Codex mode
```

### 4.1 Required core interfaces

Define platform-neutral Rust types similar to the following. Exact syntax may be adjusted, but preserve the semantics.

```rust
pub type ThreadId = String;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum ConnectionState {
    NotRunning,
    RunningNotFocused,
    Connected,
    CodexModeNotDetected,
    PermissionRequired,
    Degraded,
    Error,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum ThreadStatus {
    Working,
    Thinking,
    WaitingForUser,
    WaitingForApproval,
    Completed,
    Failed,
    Idle,
    Unknown,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ThreadSummary {
    pub id: ThreadId,
    pub title: String,
    pub project: Option<String>,
    pub status: ThreadStatus,
    pub is_active: bool,
    pub updated_at_ms: Option<u64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum CodexAction {
    FocusApp,
    NewThread,
    ReviewChanges,
    Approve,
    Reject,
    DiscardChanges,
    SubmitPrompt { text: String },
    StartSystemDictation,
    SelectThread { thread_id: ThreadId },
    SetReasoningLevel { value: String },
    OpenShortcutHelp,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum ActionTarget {
    ActiveThread,
    SelectedThread(ThreadId),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum ActionOutcome {
    Succeeded,
    Unsupported,
    TargetNotFound,
    PermissionDenied,
    TimedOut,
    Ambiguous,
    AppNotRunning,
    CodexModeNotDetected,
    Failed,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionResult {
    pub action: CodexAction,
    pub target: ActionTarget,
    pub outcome: ActionOutcome,
    pub user_message: String,
    pub diagnostic_code: String,
    pub elapsed_ms: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CapabilitySet {
    pub can_list_threads: bool,
    pub can_select_thread: bool,
    pub can_create_thread: bool,
    pub can_review_changes: bool,
    pub can_approve: bool,
    pub can_reject: bool,
    pub can_discard_changes: bool,
    pub can_submit_prompt: bool,
    pub can_start_system_dictation: bool,
    pub can_read_reasoning_options: bool,
    pub can_set_reasoning_level: bool,
}

#[async_trait::async_trait]
pub trait AutomationAdapter: Send + Sync {
    async fn connection_state(&self) -> Result<ConnectionState, AutomationError>;
    async fn capabilities(&self) -> Result<CapabilitySet, AutomationError>;
    async fn list_threads(&self) -> Result<Vec<ThreadSummary>, AutomationError>;
    async fn active_thread(&self) -> Result<Option<ThreadSummary>, AutomationError>;
    async fn execute(
        &self,
        action: CodexAction,
        target: ActionTarget,
    ) -> Result<ActionResult, AutomationError>;
}
```

Do not allow frontend components to call Win32/UIA code directly.

### 4.2 Feature capability discovery

The Codex/ChatGPT UI may change between app versions. Therefore:

- Discover available controls at runtime.
- Expose a capability matrix to the UI.
- Disable unavailable buttons instead of guessing.
- Include a tooltip explaining why an action is unavailable.
- Cache selectors only for the current app version and invalidate them when the target application changes.
- Prefer accessible `AutomationId`, control type, name, and hierarchy relationships.
- Do not rely only on screen coordinates.
- Do not rely on English text alone; make selector profiles localizable.
- Allow multiple selectors per semantic action, ordered from strongest to weakest.
- Record which selector matched in redacted diagnostics.

---

## 5. Windows automation implementation

### 5.1 Process and window discovery

Build a `TargetAppLocator` that:

1. Enumerates top-level windows.
2. Matches configured process names and optional publisher metadata.
3. Confirms the candidate window exposes expected ChatGPT/Codex accessibility structure.
4. Returns a stable target handle and process identifier.
5. detects window recreation after an update or restart.
6. never assumes the first process-name match is correct.

Expose:

```rust
pub struct TargetWindow {
    pub hwnd: isize,
    pub process_id: u32,
    pub process_name: String,
    pub window_title: String,
}
```

### 5.2 UI Automation client

Create a dedicated COM/UIA worker thread.

Requirements:

- Initialize COM correctly.
- Keep UIA operations off the Tauri UI thread.
- Use timeouts around each automation operation.
- Recover from stale UI elements.
- Re-resolve target elements after navigation.
- Subscribe to relevant focus, structure, property, and notification events when possible.
- Fall back to bounded polling only where events are unavailable.
- Use exponential backoff when the target app is closed.
- Avoid polling faster than necessary.

### 5.3 Semantic selector system

Represent selectors as data, not scattered conditionals.

Example:

```rust
pub struct ElementSelector {
    pub automation_id: Option<String>,
    pub names: Vec<String>,
    pub control_type: Option<ControlType>,
    pub ancestor_hints: Vec<SelectorHint>,
    pub descendant_hints: Vec<SelectorHint>,
    pub required_patterns: Vec<AutomationPattern>,
}
```

Define a selector profile for:

- main ChatGPT window
- Codex mode indicator
- thread sidebar
- thread row
- active thread
- new thread button
- review button
- approve button
- reject/decline button
- discard/undo button
- prompt composer
- send button
- model/reasoning control
- available reasoning option
- approval dialog/panel
- shortcut-help page

Store built-in selector profiles in versioned files, for example:

```text
src-tauri/resources/selectors/windows/default.json
src-tauri/resources/selectors/windows/chatgpt-2026.json
```

User calibration profiles must be stored separately in the application data directory.

### 5.4 Action execution strategy

For each action:

1. Resolve target thread.
2. Focus or select the target thread when required.
3. Revalidate the visible context.
4. Discover the semantic control.
5. Invoke it through the strongest supported UIA pattern.
6. Use a verified keyboard shortcut only when UIA cannot invoke the control.
7. Wait for observable confirmation.
8. Return a typed `ActionResult`.

Examples of confirmation:

- New thread: a new composer/thread view becomes active.
- Review: diff/review region appears.
- Approve: approval prompt disappears and execution resumes.
- Reject: approval prompt disappears with rejected/declined state.
- Thread switch: selected thread reports active state or its title appears in the main view.
- Reasoning change: selected option reports the requested value.
- Prompt submission: composer clears or a new user message is observed.

Never mark success immediately after sending an input.

### 5.5 Dangerous and ambiguous actions

Treat these actions separately:

- `Reject`: decline the currently visible permission/approval request.
- `DiscardChanges`: discard or undo code changes.

Do not map both to one generic X button.

Before destructive actions:

- Verify exact context.
- Show confirmation when the result could remove work.
- Allow users to disable confirmation only for simple approval rejection, not for discarding code.
- Never invoke an ambiguous control.

---

## 6. Thread behavior

Support both active-thread and selected-thread modes.

### 6.1 Floating controller thread slots

Display four thread slots by default.

Each slot shows:

- index
- truncated title
- status
- active indicator
- selected indicator
- optional project name in tooltip

Interactions:

- Single click: select slot inside MicroDeck.
- Double click: select and focus that thread inside ChatGPT Codex.
- Mouse wheel over slots: move selection.
- Keyboard shortcuts: select slots 1–4.
- A “Follow active” toggle makes MicroDeck selection track the active Codex thread.
- When “Follow active” is off, actions target the selected MicroDeck thread.
- The action target must always be visible in the controller.

If more than four threads exist, support paging or a compact overflow list. Do not attempt to show every thread in the floating view.

### 6.2 Status accuracy

Map observed state only when there is evidence.

Possible evidence includes:

- accessible status text
- progress indicator
- approval panel presence
- active spinner/progress element
- completed result state
- error element

If evidence is incomplete, use `Unknown`, not a guessed state.

---

## 7. Voice input

Implement voice in stages behind one clean provider interface.

```rust
pub trait VoiceInputProvider: Send + Sync {
    async fn start(&self) -> Result<(), VoiceError>;
    async fn stop_and_transcribe(&self) -> Result<String, VoiceError>;
    fn mode(&self) -> VoiceMode;
}
```

### MVP provider: Windows system dictation

- Select/focus the target thread.
- Focus the Codex prompt composer.
- Invoke Windows system dictation.
- Clearly show that dictation is controlled by Windows.
- Do not claim MicroDeck has transcribed audio itself.
- Provide a configurable push-to-talk hotkey.
- On release, leave the user in control of reviewing and sending the text unless “send after dictation” is explicitly enabled.

### Future provider boundary

Keep the architecture ready for an optional local transcription provider, but do not bundle a large speech model in the MVP.

Microphone UI states:

- idle
- preparing
- listening
- transcribing/system dictation open
- ready to send
- error

Never record audio silently.

---

## 8. Reasoning control

The rotary control in the floating UI should represent the reasoning/model option that is actually exposed by the current Codex UI.

Requirements:

- Query available options dynamically where possible.
- Use discrete detents, not an arbitrary 0–100 value.
- Display the exact visible option label.
- Do not assume permanent labels such as Low, Medium, or High.
- Do not claim that a reasoning level changed without visible confirmation.
- Disable the control with an explanation when the option is not discoverable.
- Keyboard and mouse-wheel adjustments must be supported.
- Add a short debounce to prevent rapid accidental changes.

---

## 9. User interface

Create two synchronized windows.

### 9.1 Floating controller

The floating controller is the primary experience.

Visual direction:

- clean off-white or light neutral shell
- soft depth and restrained shadows
- original hardware-inspired layout
- rounded rectangular body
- high-contrast black line icons
- subtle status colors
- no copied OpenAI/Codex wordmark
- draggable empty region
- compact enough to sit next to ChatGPT
- scalable from 80% to 140%
- always-on-top option
- snap to screen edges
- remember position per monitor
- collapse to an even smaller strip
- optional reduced-motion mode

Suggested controls:

Top:
- MicroDeck label
- connection indicator
- settings button

Thread region:
- four thread/status slots
- next/previous page
- Follow active toggle

Action region:
- Review
- Approve
- Reject
- New thread
- Switch/focus selected thread
- Voice
- More

Reasoning region:
- rotary-style discrete selector
- current value label

Bottom:
- push-to-talk area
- target label: `Active: …` or `Selected: …`

Actions must have text labels or accessible tooltips. Do not depend on icons alone.

### 9.2 Full dashboard

Sections:

1. Overview
   - connection state
   - detected target executable/version
   - capability summary
   - current target mode
   - active and selected threads
2. Threads
   - searchable list
   - status
   - project
   - target/select/focus controls
3. Actions
   - full action grid
   - action result history
4. Shortcuts
   - editable global mappings
   - collision detection
   - restore defaults
5. Automation
   - detected selectors
   - capability diagnostics
   - test each action without hiding errors
   - open selector calibration wizard
6. Voice
   - dictation mode
   - push-to-talk mapping
   - send-after-dictation preference
7. Appearance
   - size
   - opacity
   - always on top
   - reduced motion
8. Privacy
   - what is read
   - what is stored
   - clear local data
   - export redacted diagnostic report
9. About
   - version
   - unofficial affiliation notice
   - licenses

### 9.3 Accessibility of MicroDeck itself

- Full keyboard navigation.
- Visible focus rings.
- Accessible names for every control.
- Minimum practical target size.
- High-contrast compatibility.
- Screen-reader-friendly live status announcements without spam.
- Do not use status color as the only signal.
- Respect Windows text scaling.
- Test at 100%, 125%, 150%, and 200% display scaling.

---

## 10. Onboarding

Build a first-run wizard:

1. Welcome and explanation.
2. Privacy summary.
3. Detect ChatGPT/Codex installation.
4. Ask user to open ChatGPT and switch to Codex mode.
5. Run accessibility capability scan.
6. Show detected actions.
7. Configure global shortcuts.
8. Test one safe action: focus the app.
9. Optionally test new thread after explicit confirmation.
10. Choose floating-controller size and location.
11. Finish.

Windows UI Automation generally does not require a macOS-style accessibility permission prompt, so do not invent one. Instead, display diagnostics when the target app does not expose required accessible elements or when privilege levels prevent interaction.

If MicroDeck and ChatGPT run at different integrity levels, explain that automation may fail and guide the user to run them at matching privilege levels. Do not automatically request administrator privileges.

---

## 11. Default shortcuts

Use uncommon, configurable defaults and detect collisions.

Suggested defaults:

- Show/hide floating controller: `Ctrl+Alt+Space`
- Focus ChatGPT Codex: `Ctrl+Alt+C`
- New thread: `Ctrl+Alt+N`
- Review changes: `Ctrl+Alt+R`
- Approve: `Ctrl+Alt+A`
- Reject approval: `Ctrl+Alt+D`
- Voice push-to-talk: `Ctrl+Alt+V`
- Previous thread: `Ctrl+Alt+Left`
- Next thread: `Ctrl+Alt+Right`
- Thread slots 1–4: `Ctrl+Alt+1` through `Ctrl+Alt+4`

Do not register shortcuts that conflict with OS-reserved combinations. Surface registration failures clearly.

---

## 12. Settings model

Define a versioned settings schema.

```rust
pub struct AppSettings {
    pub schema_version: u32,
    pub target_process_candidates: Vec<String>,
    pub always_on_top: bool,
    pub launch_at_startup: bool,
    pub follow_active_thread: bool,
    pub confirm_discard_changes: bool,
    pub send_after_dictation: bool,
    pub controller_scale: f32,
    pub controller_opacity: f32,
    pub reduced_motion: bool,
    pub shortcuts: ShortcutSettings,
    pub selector_profile: String,
    pub diagnostic_level: DiagnosticLevel,
}
```

Requirements:

- Validate on load.
- Migrate old schema versions.
- Restore safe defaults when corrupted.
- Never crash because of a settings file.
- Write atomically.

---

## 13. Diagnostics and resilience

Implement structured diagnostic codes, for example:

- `TARGET_APP_NOT_FOUND`
- `TARGET_WINDOW_AMBIGUOUS`
- `CODEX_MODE_NOT_DETECTED`
- `UIA_ELEMENT_NOT_FOUND`
- `UIA_PATTERN_UNSUPPORTED`
- `UIA_STALE_ELEMENT`
- `TARGET_PRIVILEGE_MISMATCH`
- `SHORTCUT_REGISTRATION_FAILED`
- `ACTION_CONFIRMATION_TIMEOUT`
- `REASONING_CONTROL_UNAVAILABLE`
- `THREAD_TARGET_NOT_FOUND`
- `VOICE_DICTATION_FAILED`

The UI must show human-readable recovery steps.

Add a redacted diagnostic export containing:

- MicroDeck version
- operating-system version
- target process name and version
- enabled selector profile
- capability matrix
- diagnostic codes
- timings
- no conversation content
- no repository paths unless explicitly included by the user
- no tokens or credentials

When the target application updates and selectors stop matching:

- enter degraded mode
- keep safe functions available
- disable unsupported actions
- offer diagnostics/calibration
- never fall back silently to hardcoded clicks

---

## 14. Selector calibration wizard

Build a developer/user-assisted calibration workflow as a fallback.

Flow:

1. User opens the relevant Codex screen.
2. MicroDeck asks them to hover or focus a target control.
3. Read the UIA element under focus/cursor.
4. Show sanitized properties:
   - name
   - automation ID
   - control type
   - supported patterns
   - ancestor path
5. User confirms the semantic meaning.
6. Save a local override selector.
7. Test the selector.
8. Allow reset to built-in profile.

Do not store absolute screen coordinates as the primary selector.

Add developer mode to export a sanitized selector profile for maintainers.

---

## 15. Required repository structure

Use a structure close to:

```text
microdeck/
├─ README.md
├─ LICENSE
├─ SECURITY.md
├─ PRIVACY.md
├─ package.json
├─ vite.config.ts
├─ tsconfig.json
├─ src/
│  ├─ app/
│  │  ├─ App.tsx
│  │  ├─ routes.tsx
│  │  └─ bootstrap.ts
│  ├─ components/
│  │  ├─ controller/
│  │  ├─ dashboard/
│  │  ├─ common/
│  │  └─ onboarding/
│  ├─ features/
│  │  ├─ actions/
│  │  ├─ threads/
│  │  ├─ voice/
│  │  ├─ reasoning/
│  │  ├─ shortcuts/
│  │  ├─ diagnostics/
│  │  └─ settings/
│  ├─ state/
│  ├─ api/
│  ├─ styles/
│  ├─ test/
│  └─ types/
├─ src-tauri/
│  ├─ Cargo.toml
│  ├─ tauri.conf.json
│  ├─ capabilities/
│  ├─ resources/
│  │  └─ selectors/
│  │     └─ windows/
│  └─ src/
│     ├─ lib.rs
│     ├─ commands.rs
│     ├─ events.rs
│     ├─ core/
│     │  ├─ mod.rs
│     │  ├─ actions.rs
│     │  ├─ capabilities.rs
│     │  ├─ threads.rs
│     │  └─ models.rs
│     ├─ automation/
│     │  ├─ mod.rs
│     │  ├─ adapter.rs
│     │  ├─ mock.rs
│     │  ├─ selectors.rs
│     │  └─ windows/
│     │     ├─ mod.rs
│     │     ├─ locator.rs
│     │     ├─ uia_client.rs
│     │     ├─ uia_worker.rs
│     │     ├─ element_query.rs
│     │     ├─ action_executor.rs
│     │     ├─ thread_reader.rs
│     │     ├─ events.rs
│     │     └─ input_fallback.rs
│     ├─ settings/
│     ├─ shortcuts/
│     ├─ diagnostics/
│     ├─ voice/
│     └─ platform/
└─ tests/
   ├─ fixtures/
   ├─ automation_contract/
   └─ manual/
```

Keep individual files focused. Avoid a giant `main.rs`, `App.tsx`, or automation file.

---

## 16. Development sequence

Implement in these phases. Each phase must leave the repository runnable and tested.

### Phase 0 — Research spike

Before production implementation:

1. Confirm the installed target process and window names.
2. Use Windows accessibility inspection tools to inspect:
   - thread list
   - new thread
   - review
   - approvals
   - composer
   - model/reasoning picker
3. Record sanitized UIA trees as test fixtures.
4. Document what is and is not exposed.
5. Build a tiny Rust proof of concept that:
   - finds the target window
   - prints a sanitized UIA subtree
   - focuses the app
6. Do not build the full UI until this spike proves the core automation route.

Deliverable:
`docs/research/windows-codex-uia-spike.md`

The document must clearly label:
- verified
- partially verified
- unavailable
- fragile

### Phase 1 — Project foundation and mock adapter

- Scaffold Tauri 2 + React + strict TypeScript.
- Add linting, formatting, Rust clippy, and tests.
- Implement all shared types.
- Implement `MockAutomationAdapter`.
- Build frontend against the mock adapter.
- Add CI for Windows build, frontend tests, Rust tests, clippy, and formatting.

### Phase 2 — Target detection and connection state

- Implement process/window locator.
- Detect Codex mode where possible.
- Add connection indicator.
- Add focus-app action.
- Add diagnostics for no target, ambiguity, and privilege mismatch.

### Phase 3 — Floating controller

- Build exact approved layout using original branding.
- Add drag, always-on-top, scaling, snapping, position persistence, collapse, and tray behavior.
- Connect to mock/live state.
- Ensure full keyboard accessibility.

### Phase 4 — Thread discovery and targeting

- Read visible threads through UIA.
- Determine active thread.
- Implement selected-thread state.
- Implement Follow active.
- Implement thread switching and observable confirmation.
- Add four thread slots and overflow.

### Phase 5 — Safe actions

Implement in this order:

1. New thread
2. Review changes
3. Approve current approval request
4. Reject current approval request
5. Discard/undo changes with mandatory confirmation

Each action needs:
- capability detection
- typed result
- timeout
- observable confirmation
- diagnostic code
- unit tests with fixtures
- manual test case

### Phase 6 — Prompt and voice

- Implement prompt-composer focus.
- Implement text prompt submission only after explicit send action.
- Implement Windows system dictation provider.
- Add push-to-talk shortcut and visible listening state.
- Do not capture audio independently in the MVP.

### Phase 7 — Reasoning/model control

- Inspect available options.
- Build discrete dial.
- Implement selection and confirmation.
- Disable gracefully when unavailable.

### Phase 8 — Full dashboard, onboarding, settings, and calibration

- Build all dashboard sections.
- Build onboarding.
- Add shortcut editor and conflict detection.
- Add diagnostics export.
- Add selector calibration wizard.
- Add privacy and reset controls.

### Phase 9 — Packaging and release readiness

- Build signed-ready Windows installer configuration.
- Verify clean install, upgrade, uninstall, and local-data behavior.
- Add crash-safe logging.
- Add release workflow.
- Add updater only if a secure signed update process is configured.
- Produce user documentation and troubleshooting guide.

### Phase 10 — macOS-ready boundary

Do not implement untestable macOS automation.

Instead:

- Ensure `AutomationAdapter` is platform-neutral.
- Add compile-gated module boundaries.
- Document expected `MacAutomationAdapter` behavior using macOS Accessibility APIs.
- Keep macOS code out of Windows production paths.
- Add a macOS implementation only when actual macOS testing is available.

---

## 17. Testing strategy

Use test-driven development for core behavior.

### 17.1 Rust unit tests

Test:

- selector ranking
- semantic action resolution
- capability calculation
- action result mapping
- thread status mapping
- settings validation and migration
- log redaction
- timeout behavior
- stale-element retry behavior
- target ambiguity handling

Mock the platform boundary. Do not require ChatGPT to be installed for unit tests.

### 17.2 Automation contract tests

Create fixture-based UIA trees for:

- no app
- ChatGPT open, not Codex mode
- empty Codex project
- multiple threads
- active working thread
- waiting approval
- review screen
- completed thread
- error thread
- reasoning control present
- reasoning control absent
- renamed/localized controls

Run the same adapter contract tests against each fixture.

### 17.3 Frontend tests

Test:

- buttons disabled from capabilities
- active vs selected target label
- Follow active behavior
- action loading/success/error states
- thread pagination
- shortcut collision UI
- destructive confirmation
- accessibility names and keyboard navigation
- settings persistence
- onboarding state transitions

### 17.4 End-to-end tests

Use the mock adapter for deterministic Tauri E2E coverage.

For live Codex testing, maintain a manual QA checklist because the target app is external and version-dependent.

### 17.5 Manual Windows matrix

Test at minimum:

- Windows 11 current stable
- standard user
- ChatGPT closed
- ChatGPT open in Chat mode
- ChatGPT open in Codex mode
- one thread
- four threads
- more than four threads
- waiting approval
- review screen open
- 100%, 125%, 150%, 200% scaling
- single monitor
- multiple monitors with mixed DPI
- target app restarted
- target app updated
- MicroDeck restarted
- shortcut collision
- target running elevated
- screen reader enabled
- high contrast
- reduced motion

---

## 18. Definition of done

The MVP is done only when all of the following are true:

1. A Windows installer builds successfully.
2. The application launches as a single instance.
3. The tray icon works.
4. The floating controller can be shown globally.
5. The app detects and focuses ChatGPT Codex.
6. It can list and switch between supported visible threads, or clearly report that the installed version does not expose them.
7. Both active-thread and selected-thread targeting work.
8. New thread, review, approve, and reject work with observable confirmation on the tested Codex version.
9. Discard changes is context-aware and confirmation-protected.
10. Voice opens Windows dictation on the correct composer.
11. Reasoning control works only when discoverable and otherwise disables honestly.
12. Every action reports a typed outcome.
13. No action relies solely on absolute screen coordinates.
14. No credentials or conversation content are logged.
15. Unit, contract, frontend, lint, format, and clippy checks pass.
16. The app remains usable with keyboard and Windows scaling.
17. README, privacy, security, setup, troubleshooting, architecture, and selector-maintenance docs exist.
18. The public UI contains the unofficial-affiliation notice.
19. There are no fake controls, placeholder handlers, “coming soon” buttons presented as working, or swallowed errors.
20. A fresh developer can clone the repo, run one documented setup command, and launch the development app.

---

## 19. Required documentation

Create:

- `README.md`
- `docs/architecture.md`
- `docs/automation-model.md`
- `docs/selector-maintenance.md`
- `docs/testing.md`
- `docs/releasing-windows.md`
- `docs/troubleshooting.md`
- `docs/research/windows-codex-uia-spike.md`
- `PRIVACY.md`
- `SECURITY.md`
- `CONTRIBUTING.md`

README must include:

- what MicroDeck does
- what it does not do
- current tested ChatGPT/Codex version
- Windows requirements
- installation
- development
- permissions/integrity-level notes
- privacy
- limitations caused by UI automation
- unofficial affiliation disclaimer

---

## 20. Engineering rules for the agent

While implementing:

1. Start by inspecting the repository and installed toolchain.
2. Do not overwrite working code without understanding it.
3. Write a short implementation plan before each phase.
4. Use small, reviewable commits.
5. Write failing tests before core implementation.
6. Run tests after every meaningful change.
7. Never report a feature complete without showing the verification command and result.
8. Never fake external-app state.
9. Never silently substitute coordinate clicking for UI Automation.
10. Keep OS-specific code behind the adapter boundary.
11. Keep UI components independent from automation details.
12. Prefer a smaller reliable feature over a broad unreliable one.
13. Document every target-app assumption.
14. When an external UI element cannot be discovered, stop and produce a diagnostic finding rather than inventing a selector.
15. Preserve user control around destructive actions.

---

## 21. First response required from the coding agent

Before writing production code, respond with:

1. Repository assessment.
2. Toolchain assessment.
3. Exact Phase 0 research procedure.
4. Proposed file tree.
5. Main technical risks.
6. Test strategy.
7. A list of facts that must be verified against the installed ChatGPT/Codex app.
8. The smallest proof of concept to build first.

Then begin Phase 0. Do not jump directly to styling the floating controller.

The most important success criterion is not visual similarity. It is **reliable, honest, observable control of the Codex desktop experience with graceful degradation when the external UI changes**.
