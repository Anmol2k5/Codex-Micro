# Troubleshooting

## MicroDeck says “Not running”

Open the ChatGPT desktop app. If the window is open but still not detected, record its exact window title and executable name and update the discovery candidates after verification.

## “Focus Codex” returns `TARGET_PRIVILEGE_MISMATCH`

Windows can prevent one process from foregrounding another when privilege levels differ. Run MicroDeck and ChatGPT at the same privilege level. Do not automatically run MicroDeck as administrator.

## Most action buttons are disabled

This is expected in the production Windows adapter until Codex-specific UI Automation selectors are verified on your installed version. Use `MICRODECK_USE_MOCK=1` only for development/demo behavior.

## The controller disappeared

Open the dashboard and choose **Show controller**.

## Browser mode looks functional but does not control ChatGPT

The browser development bridge is a deterministic demo. Run `npm run tauri dev` on Windows for native app discovery.
