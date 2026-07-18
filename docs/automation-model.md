# Automation Model

MicroDeck separates shared product behavior from operating-system automation through the Rust `AutomationAdapter` trait.

## Current Windows behavior

The production Windows adapter currently performs two verified foundation tasks:

- discover a visible top-level ChatGPT/Codex window using process-name and window-title candidates;
- restore and focus that window through Win32.

All Codex-specific controls remain unavailable until their Windows UI Automation selectors are inspected and verified against an installed app version. This prevents the UI from presenting fake functionality.

## Action contract

Every action returns an `ActionResult` containing:

- the requested action;
- the target mode;
- an explicit outcome;
- a user-safe message;
- a stable diagnostic code;
- elapsed time.

Unsupported or unverified controls return `unsupported` with `UIA_SELECTOR_NOT_VERIFIED` rather than attempting coordinate clicks.

## Selector profiles

Built-in selector profiles belong under `src-tauri/resources/selectors/windows/`. The shipped default profile is intentionally empty. Future verified selectors should prefer, in order:

1. Automation ID;
2. control type plus supported UIA pattern;
3. stable ancestor/descendant relationships;
4. accessible name as a weaker, localizable hint.

Absolute screen coordinates are not valid primary selectors.
