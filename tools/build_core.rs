//! 从 src/ 源码编译分发二进制 dist\core.exe（Rust crate；承载全部子命令，
//! snapshot 恢复账号方案、setup 构建数据库等）。中间产物落在 src\target\（不入库）。

use std::fs;
use std::path::Path;
use std::process::Command;

pub fn run() -> Result<(), String> {
    // 本工具在 tools\；仓库根是其父目录。src\ 是 core crate。
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("无法定位仓库根")?;
    let src = repo_root.join("src");
    let dist = repo_root.join("dist");
    fs::create_dir_all(&dist).map_err(|e| format!("无法创建 {}: {e}", dist.display()))?;

    let manifest = src.join("Cargo.toml");
    println!("编译 core -> {}\\", dist.display());
    let status = Command::new("cargo")
        .args(["build", "--release", "--manifest-path"])
        .arg(&manifest)
        .status()
        .map_err(|e| format!("无法启动 cargo: {e}"))?;
    if !status.success() {
        let code = status.code().map_or("未知".to_string(), |c| c.to_string());
        return Err(format!("cargo 编译失败（退出码 {code}）"));
    }

    let produced = src.join("target").join("release").join("core.exe");
    if !produced.is_file() {
        return Err(format!("构建产物未找到: {}", produced.display()));
    }
    let target = dist.join("core.exe");
    fs::copy(&produced, &target).map_err(|e| format!("复制产物失败: {e}"))?;
    println!("  ok: {}", target.display());
    Ok(())
}
