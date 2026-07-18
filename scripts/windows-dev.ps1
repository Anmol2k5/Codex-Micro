$ErrorActionPreference = "Stop"

if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
  throw "Node.js is required."
}
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
  throw "Rust/Cargo is required for Tauri development."
}

npm install
npm run tauri dev
