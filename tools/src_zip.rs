//! 把 src/ 打包成 AES-256 加密的标准 zip（WinZip AES）。
//!
//! 产物可用 7-Zip、WinRAR、PeaZip、Keka、pyzipper 等带密码打开
//! （Windows 资源管理器自带解压不支持 AES zip）。
//!
//! 密码是必填项：终端下交互安全提示（不回显，需输入两次确认）；非终端（管道）时读 stdin 单行。
//! 两种方式都不让密码出现在命令行。

use std::fs::{self, File};
use std::io::{self, BufRead, IsTerminal};
use std::path::{Path, PathBuf};

use zip::write::SimpleFileOptions;
use zip::{AesMode, CompressionMethod, ZipWriter};

const EXCLUDE_DIRS: &[&str] = &["target", "__pycache__", ".pytest_cache"];
const EXCLUDE_SUFFIXES: &[&str] = &[".pyc"];

pub fn run(args: &[String]) -> Result<(), String> {
    let (src, out) = parse_args(args)?;

    let password = read_password()?;
    if password.is_empty() {
        return Err("密码不能为空".into());
    }
    if !src.is_dir() {
        return Err(format!("源目录不存在：{}", src.display()));
    }

    let entries = entries(&src)?;
    if entries.is_empty() {
        return Err(format!("{} 下没有可打包的文件", src.display()));
    }

    if let Some(parent) = out.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(parent).map_err(|e| format!("无法创建 {}: {e}", parent.display()))?;
    }
    let file = File::create(&out).map_err(|e| format!("无法写入 {}: {e}", out.display()))?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .with_aes_encryption(AesMode::Aes256, &password);
    for (abs_path, arcname) in &entries {
        zip.start_file(arcname.as_str(), options)
            .map_err(|e| format!("写入 {arcname} 失败: {e}"))?;
        let mut reader =
            File::open(abs_path).map_err(|e| format!("无法读取 {}: {e}", abs_path.display()))?;
        io::copy(&mut reader, &mut zip).map_err(|e| format!("写入 {arcname} 失败: {e}"))?;
    }
    zip.finish().map_err(|e| format!("收尾失败: {e}"))?;

    println!("  {} 个文件 -> {}（AES-256）", entries.len(), out.display());
    Ok(())
}

fn parse_args(args: &[String]) -> Result<(PathBuf, PathBuf), String> {
    let (mut src, mut out) = (None, None);
    let mut it = args.iter();
    while let Some(flag) = it.next() {
        let slot = match flag.as_str() {
            "--src" => &mut src,
            "--out" => &mut out,
            other => return Err(format!("未知参数：{other}")),
        };
        *slot = Some(PathBuf::from(it.next().ok_or(format!("{flag} 缺少取值"))?));
    }
    match (src, out) {
        (Some(src), Some(out)) => Ok((src, out)),
        _ => Err("需要 --src <源目录> 与 --out <输出 zip>".into()),
    }
}

/// 终端下安全提示并二次确认；管道输入时读 stdin 单行。
fn read_password() -> Result<String, String> {
    if io::stdin().is_terminal() {
        let pw = rpassword::prompt_password("源码包密码: ").map_err(|e| e.to_string())?;
        let again = rpassword::prompt_password("再次输入确认: ").map_err(|e| e.to_string())?;
        if pw != again {
            return Err("两次输入的密码不一致".into());
        }
        return Ok(pw);
    }
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).map_err(|e| e.to_string())?;
    Ok(line.trim_end_matches(['\r', '\n']).to_string())
}

/// 返回 [(绝对路径, zip 内相对名), ...]，相对名以 src 的父目录为基准（解压得到 src/…）。
fn entries(src: &Path) -> Result<Vec<(PathBuf, String)>, String> {
    let src = fs::canonicalize(src).map_err(|e| format!("{}: {e}", src.display()))?;
    let base = src.parent().ok_or("源目录没有父目录")?.to_path_buf();
    let mut out = Vec::new();
    walk(&src, &base, &mut out)?;
    Ok(out)
}

fn walk(dir: &Path, base: &Path, out: &mut Vec<(PathBuf, String)>) -> Result<(), String> {
    let mut items: Vec<_> = fs::read_dir(dir)
        .and_then(|rd| rd.collect::<io::Result<Vec<_>>>())
        .map_err(|e| format!("{}: {e}", dir.display()))?;
    items.sort_by_key(|e| e.file_name());

    let mut subdirs = Vec::new();
    for item in items {
        let path = item.path();
        let name = item.file_name().to_string_lossy().into_owned();
        if path.is_dir() {
            if !EXCLUDE_DIRS.contains(&name.as_str()) {
                subdirs.push(path);
            }
        } else if !EXCLUDE_SUFFIXES.iter().any(|s| name.ends_with(s)) {
            let rel = path.strip_prefix(base).map_err(|e| e.to_string())?;
            let arcname = rel.to_string_lossy().replace('\\', "/");
            out.push((path, arcname));
        }
    }
    // 与 os.walk 一致：先本目录文件，再逐个子目录。
    for sub in subdirs {
        walk(&sub, base, out)?;
    }
    Ok(())
}
