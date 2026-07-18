# Selector Maintenance

Codex UI selectors must be treated as versioned compatibility data.

## Verification procedure

1. Record the installed ChatGPT/Codex app version.
2. Inspect the relevant screen with Accessibility Insights for Windows or Inspect.exe.
3. Capture only sanitized metadata: Automation ID, control type, accessible name, patterns, and a short ancestor path.
4. Add the selector to a versioned profile.
5. Add a fixture/contract test for the semantic action.
6. Verify the action and its observable confirmation on the actual app.
7. Only then set the corresponding capability to `true`.

## Failure behavior

When a selector stops matching, MicroDeck should enter degraded mode for that capability, record a diagnostic code, and disable the action. It must not silently switch to mouse coordinates.
