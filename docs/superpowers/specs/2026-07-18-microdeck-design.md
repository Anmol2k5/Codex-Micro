# MicroDeck Design

## Goal

Build a Windows-first, cross-platform-ready software control surface for the Codex desktop experience, with both a compact floating controller and a full diagnostics/settings dashboard.

## Product decisions

- Windows ships first; macOS is an adapter added later when test hardware is available.
- The floating controller is the primary interaction surface.
- A full dashboard exposes threads, capabilities, results, settings, and diagnostics.
- Actions target the active Codex thread by default or an explicitly selected MicroDeck thread when Follow active is disabled.
- Windows UI Automation is the primary external-control mechanism.
- Keyboard shortcuts are a verified fallback only.
- No private API, token scraping, memory injection, or coordinate-only automation.
- The public product name is MicroDeck with an unofficial-affiliation notice.

## Initial architecture

Tauri 2 hosts a React/TypeScript frontend and a Rust application core. The core depends only on an `AutomationAdapter` trait. The Windows implementation will use UI Automation behind that trait; development and deterministic tests use a mock adapter.

## Reliability model

Every action returns `ActionResult` with an explicit outcome and diagnostic code. Capabilities are discovered at runtime. The UI disables unavailable actions. Live Windows selectors are not shipped until verified against the installed target application.

## Testing

Pure targeting rules are covered in frontend unit tests. Adapter behavior is covered in Rust tests. Live Codex automation requires a Windows manual/fixture capture pass before selectors are promoted to production.
