# Testing

## Frontend

```bash
npm run typecheck
npm run test
npm run build
```

The frontend suite currently covers:

- active-versus-selected targeting;
- thread paging;
- action capability mapping;
- settings migration and clamping;
- diagnostic redaction.

## Rust

Run on a machine with the Rust toolchain:

```bash
cargo test --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
```

## Browser demo

`npm run dev` runs the React UI with a deterministic browser bridge. It is useful for UI work and does not control ChatGPT.

## Tauri mock mode

On Windows:

```powershell
$env:MICRODECK_USE_MOCK="1"
npm run tauri dev
```

This exercises both Tauri windows and the Rust command boundary without depending on the external ChatGPT UI.

## Manual Windows QA

At minimum verify:

- ChatGPT closed;
- ChatGPT open and unfocused;
- ChatGPT foreground;
- standard user versus elevated target;
- 100%, 125%, 150%, and 200% display scaling;
- single and mixed-DPI multi-monitor setups;
- controller show/hide;
- always-on-top preference;
- diagnostic export redaction;
- clean install and uninstall.
