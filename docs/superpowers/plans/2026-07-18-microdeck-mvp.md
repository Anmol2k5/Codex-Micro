# MicroDeck MVP Implementation Plan

> **For agentic workers:** implement phase-by-phase with verification after each task.

**Goal:** Ship a reliable Windows-first MicroDeck foundation that can control only capabilities verified against the Codex desktop UI.

**Architecture:** Tauri 2 + React/TypeScript with a Rust `AutomationAdapter` boundary. Build all UI and targeting behavior against a deterministic mock first, then replace the Windows adapter one verified action at a time.

**Tech Stack:** Tauri 2, Rust, React, TypeScript, Zustand, Vitest, Windows UI Automation.

## Global Constraints

- Never invent a private Codex API.
- Never use absolute screen coordinates as the primary automation strategy.
- Never report external actions as successful without observable confirmation.
- Keep macOS-specific automation out of Windows production code until macOS hardware is available.
- Persist no credentials or conversation content.

---

### Task 1: Foundation and mock adapter

- Create the Tauri/React project shell.
- Define shared action, target, capability, thread, connection, and result models.
- Implement a deterministic mock adapter.
- Add active-thread vs selected-thread targeting tests.
- Build the floating controller and dashboard against the mock bridge.

### Task 2: Windows Phase 0 research

- Run accessibility inspection on the real ChatGPT/Codex app.
- Capture sanitized UIA fixture trees.
- Implement target-window discovery.
- Prove safe focus-app behavior.
- Record unavailable or fragile controls honestly.

### Task 3: Threads and safe actions

- Implement thread discovery and active-thread detection.
- Implement selected-thread switching with observable confirmation.
- Add new-thread, review, approve, and reject actions one by one.
- Add action-specific fixtures and tests.

### Task 4: Voice, reasoning, settings, and packaging

- Add Windows system dictation provider.
- Add dynamically discovered reasoning choices.
- Add shortcuts, local settings, diagnostics export, calibration, tray behavior, and installer configuration.
- Run Windows manual QA matrix and CI.
