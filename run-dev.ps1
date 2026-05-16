$env:CARGO_HOME = "C:\Users\zhang\.cargo"
$env:RUSTUP_HOME = "C:\Users\zhang\.rustup"
$gnuBin = "$env:RUSTUP_HOME\toolchains\stable-x86_64-pc-windows-gnu\bin"
$rustlibBin = "$env:RUSTUP_HOME\toolchains\stable-x86_64-pc-windows-gnu\lib\rustlib\x86_64-pc-windows-gnu\bin"
$selfContained = "$rustlibBin\self-contained"
$nodeBin = "C:\Users\zhang\AppData\Local\Temp\nodejs\node-v20.12.2-win-x64"
$env:PATH = "$env:CARGO_HOME\bin;$gnuBin;$rustlibBin;$selfContained;$nodeBin;$env:PATH"

Write-Host "PATH: $env:PATH"
Write-Host "CARGO_HOME: $env:CARGO_HOME"
Write-Host "RUSTUP_HOME: $env:RUSTUP_HOME"

# Kill any existing cargo/rustc processes
Get-Process | Where-Object { $_.ProcessName -match "cargo|rustc" } | Stop-Process -Force -ErrorAction SilentlyContinue
Start-Sleep -Seconds 2

# Clean target directory
Remove-Item -Path "src-tauri\target" -Recurse -Force -ErrorAction SilentlyContinue

# Run tauri dev
npm run tauri-dev
