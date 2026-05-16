use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let filtered_args: Vec<&String> = args.iter()
        .filter(|arg| *arg != "--64")
        .collect();
    
    let gcc_path = r"C:\Users\zhang\.rustup\toolchains\stable-x86_64-pc-windows-gnu\lib\rustlib\x86_64-pc-windows-gnu\bin\self-contained\x86_64-w64-mingw32-gcc.exe";
    
    let mut cmd = Command::new(gcc_path);
    cmd.arg("-c");
    for arg in filtered_args {
        cmd.arg(arg);
    }
    
    let status = cmd.status().expect("Failed to execute gcc");
    std::process::exit(status.code().unwrap_or(1));
}
