# Contributing to MicroDeck

MicroDeck controls another desktop application, so reliability matters more than feature count.

## Rules

1. Never add hard-coded screen-coordinate automation as a production fallback.
2. Never claim an external action succeeded without observing confirmation.
3. Keep OS-specific code behind `AutomationAdapter`.
4. Add tests for pure behavior before implementation.
5. Do not commit conversation content, tokens, or unredacted accessibility dumps.

## Local checks

```bash
npm install
npm run check
```

On a Windows machine with Rust installed:

```powershell
cargo test --manifest-path src-tauri/Cargo.toml
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
npm run tauri build
```

For browser/demo development, run `npm run dev`. To force the Tauri app to use the deterministic mock adapter, set `MICRODECK_USE_MOCK=1` before launching it.
