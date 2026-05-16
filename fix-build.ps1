# Fix build script
$env:Path = "C:\Program Files\nodejs;$env:USERPROFILE\.cargo\bin;$env:Path"

# Copy ld.lld.exe to cargo bin as ld.lld.exe
$source = "$env:USERPROFILE\.rustup\toolchains\stable-x86_64-pc-windows-gnu\lib\rustlib\x86_64-pc-windows-gnu\bin\gcc-ld\ld.lld.exe"
$dest = "$env:USERPROFILE\.cargo\bin\ld.lld.exe"
Copy-Item $source $dest -Force

# Set PATH
$mingwBin = "$env:USERPROFILE\.rustup\toolchains\stable-x86_64-pc-windows-gnu\lib\rustlib\x86_64-pc-windows-gnu\bin"
$gccLd = "$env:USERPROFILE\.rustup\toolchains\stable-x86_64-pc-windows-gnu\lib\rustlib\x86_64-pc-windows-gnu\bin\gcc-ld"
$selfContained = "$env:USERPROFILE\.rustup\toolchains\stable-x86_64-pc-windows-gnu\lib\rustlib\x86_64-pc-windows-gnu\bin\self-contained"
$env:Path = "$gccLd;$selfContained;$mingwBin;$env:Path"

# Switch to GNU toolchain
rustup default stable-x86_64-pc-windows-gnu

# Build
cd src-tauri
cargo clean
cargo check
