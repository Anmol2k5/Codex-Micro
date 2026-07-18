# Releasing on Windows

## Prerequisites

- Node.js 22 or compatible LTS;
- Rust stable with Cargo;
- Microsoft C++ Build Tools required by Tauri/WebView2 tooling;
- WebView2 runtime on the target system.

## Verify

```powershell
npm ci
npm run check
cargo test --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
```

## Build installers

```powershell
npm run tauri build
```

The Tauri bundle configuration targets NSIS and MSI. Code signing is intentionally not configured in the repository because signing certificates are publisher-specific secrets.

Before publishing, perform the manual QA matrix in `docs/testing.md` and update the README with the exact tested ChatGPT/Codex app version.
