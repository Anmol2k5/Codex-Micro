# Architecture

MicroDeck separates shared product logic from operating-system automation.

```text
React UI
  -> Tauri commands
    -> AppState / AutomationAdapter
      -> MockAutomationAdapter (development/tests)
      -> WindowsAutomationAdapter (Windows UI Automation; Phase 0 verification required)
      -> MacAutomationAdapter (future, only when macOS testing is available)
```

## Rules

- The React UI never calls Win32 or UI Automation directly.
- Every external action returns an explicit outcome.
- Unsupported capabilities disable controls instead of guessing.
- Absolute screen coordinates are not a primary selector mechanism.
- No action is considered successful until observable target state confirms it.
- Conversation text and credentials must not be persisted in diagnostics.

## Current state

The repository currently runs against a deterministic mock adapter. The Windows adapter boundary exists, but live selector discovery and action execution must be verified on a Windows machine with the installed ChatGPT/Codex desktop experience.
