//! 发布工具，两个子命令各管一件事：
//!   build-core  从 src/ 编译分发二进制 dist\core.exe（Rust crate）
//!   src-zip     把 src/ 打包成 AES-256 加密的 dist\src
//!
//! 用法（在仓库根）：
//!   cargo run --release --manifest-path tools/Cargo.toml -- build-core
//!   cargo run --release --manifest-path tools/Cargo.toml -- src-zip --src src --out dist/src

mod build_core;
mod src_zip;

use std::process::ExitCode;

const USAGE: &str = "用法：tools build-core | tools src-zip --src <源目录> --out <输出 zip>";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) {
        Some("build-core") if args.len() == 1 => build_core::run(),
        Some("src-zip") => src_zip::run(&args[1..]),
        _ => Err(USAGE.to_string()),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(msg) => {
            eprintln!("错误：{msg}");
            ExitCode::from(2)
        }
    }
}
