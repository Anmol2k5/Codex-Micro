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

## Capturing a live Windows snapshot

Run the capture script with ChatGPT open in the Codex experience:

```powershell
./scripts/windows-capture-uia.ps1 -OutputPath .\microdeck-uia-snapshot.json
```

Optional limits:

```powershell
./scripts/windows-capture-uia.ps1 -MaxDepth 12 -MaxChildrenPerNode 300
```

The script uses the .NET Windows UI Automation client and Control View walker. It records names, AutomationIds, control types, class names, enabled/offscreen state, supported patterns, and hierarchy. It deliberately excludes absolute screen geometry. Common Windows user paths and token-like strings are redacted and long text is truncated, but maintainers must still review snapshots before sharing or committing them.

### Automation Lab workflow

1. Import the snapshot into the dashboard's Automation Lab.
2. Search for a candidate control by accessible name, AutomationId, class, control type, or supported pattern.
3. Inspect the selected element's metadata.
4. Map it to a semantic MicroDeck key.
5. Export the candidate selector profile.
6. Verify the selector on the same app version by executing the intended action.
7. Confirm success through an observable state change; never treat a click/invoke call alone as success.
8. Only then promote the selector into a versioned built-in profile.

A selector candidate is not a verified selector. A built-in profile must record the ChatGPT/Codex app version it was tested against and should include a fallback selector only when the fallback is also independently verified.
