# MicroDeck

MicroDeck is a Windows-first desktop companion that turns supported Codex desktop actions into a compact floating software control deck.

> **Unofficial companion for Codex. Not affiliated with or endorsed by OpenAI.**

## What is included

- Tauri 2 desktop shell with separate dashboard and floating controller windows
- React + TypeScript interface
- active-thread and selected-thread targeting model
- four-slot thread paging in the controller
- capability-aware action controls
- reasoning-option boundary
- local settings with migration, validation, scaling, opacity, reduced motion, and always-on-top preference
- bounded in-memory action history and redacted diagnostic export
- deterministic browser and Rust mock adapters
- Windows top-level ChatGPT/Codex window discovery and verified focus action
- Windows installer configuration for NSIS/MSI
- frontend tests and Windows CI configuration

## Important reliability boundary

The production Windows adapter currently enables **window discovery and Focus Codex**. Review, approve, reject, thread discovery/switching, voice, discard, prompt submission, and reasoning selection remain disabled until their Windows UI Automation selectors are verified against the installed ChatGPT/Codex version.

MicroDeck intentionally does **not** fake those controls with hard-coded coordinates.

## Quick start — UI demo

```bash
npm install
npm run dev
```

Open the URL printed by Vite. Browser mode uses deterministic demo data and does not control ChatGPT.

To preview only the floating controller, open:

```text
http://localhost:1420/?view=controller
```

## Windows desktop development

Install Node.js and the Rust/Tauri prerequisites, then:

```powershell
npm install
npm run tauri dev
```

For a full mock-mode desktop demo:

```powershell
./scripts/windows-demo.ps1
```

For the real Windows adapter:

```powershell
./scripts/windows-dev.ps1
```

## Build Windows installers

```powershell
./scripts/windows-build.ps1
```

The Tauri config targets NSIS and MSI installers.

## Verification

```bash
npm run check
```

Windows/Rust verification:

```powershell
cargo test --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
```

## Architecture

```text
React UI
  |
  | Tauri commands
  v
Application state
  |
  v
AutomationAdapter
  |-- MockAutomationAdapter
  `-- WindowsAutomationAdapter
        `-- conservative Win32 discovery/focus today
        `-- verified UI Automation selectors later
```

See `docs/architecture.md`, `docs/automation-model.md`, and `docs/selector-maintenance.md`.

## Privacy

MicroDeck requires no OpenAI API key. The current implementation does not persist conversation content. Diagnostic exports include capability state and action metadata and redact common Windows user paths and bearer-like secrets.

See `PRIVACY.md`.

## Development status

This repository is a complete, runnable foundation and browser/mock demo, plus a working Windows app-discovery/focus adapter. The final Codex-specific UI Automation selector pass must be done on a Windows machine with the actual target application installed; capabilities should only be enabled after that verification.
