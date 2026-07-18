$ErrorActionPreference = "Stop"
$env:MICRODECK_USE_MOCK = "1"
npm install
npm run tauri dev
