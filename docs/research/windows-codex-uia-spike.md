# Windows Codex UI Automation Spike

## Environment status

This document intentionally distinguishes verified facts from implementation assumptions.

### Verified in the current build environment

- The shared automation contract is platform-neutral.
- The mock adapter supports deterministic active/selected thread targeting.
- The frontend disables capabilities that an adapter reports as unavailable.
- The Windows target locator is isolated behind a Windows-only module boundary.

### Not verified in the current build environment

The current execution environment is Linux and does not contain the Windows ChatGPT desktop application. Therefore the following must be verified on the developer's Windows machine before production selectors are added:

- actual executable/process names
- top-level window identity
- whether Codex mode exposes stable UI Automation IDs
- thread list control hierarchy
- active-thread semantics
- new-thread control
- review/diff control
- approve and reject controls
- discard/undo control and destructive context
- prompt composer and send button
- reasoning/model picker
- approval panel state transitions
- observable confirmation for every action

## Required Windows procedure

1. Open the installed ChatGPT application and enter Codex mode.
2. Inspect the top-level window with Accessibility Insights for Windows or Inspect.exe.
3. Record sanitized properties only: control type, AutomationId, accessible name, supported patterns, and ancestor hierarchy.
4. Repeat for each semantic action in the capability model.
5. Confirm that controls are invokable through UI Automation patterns before adding any keyboard fallback.
6. Capture sanitized fixture trees for automated contract tests.
7. Implement `TargetAppLocator::locate` using top-level window enumeration plus accessibility-structure verification.
8. Add one safe proof: locate and focus the target app.
9. Only then implement new-thread and thread-switching actions.

## Safety rule

If a semantic control cannot be found reliably, the corresponding capability remains false. Do not silently replace the action with hardcoded coordinate clicks.
