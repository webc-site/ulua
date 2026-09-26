//! cpp 各 CLI 入口签名都是 `main(int argc, char** argv)`，`argv` 里的字节直接
//! 当文件名用；Rust 的 [`std::env::args`] 遇到非 UTF-8 参数会 panic。统一用本
//! 函数收敛一次，非法字节转为 U+FFFD，行为与上游一致（文件名照样打不开并报错，
//! 而不是进程崩溃）。注意：这一致性只覆盖本来打不开的非法名——locale 合法但
//! 非 UTF-8 编码的有效文件名，cpp 按字节 `fopen` 可读，lossy 后必报
//! "Error opening"，属全库 `&str` 路径设计的既有取舍。

use alloc::{string::String, vec::Vec};
use std::env::args_os;

/// 返回 lossy 转换后的命令行参数，含程序名。
pub fn argv() -> Vec<String> {
  args_os()
    .map(|arg| arg.to_string_lossy().into_owned())
    .collect()
}

/// cpp 各 CLI `main` 的 `argv[0]`（程序名，用于 help/错误文案）：空参数表时落回
/// 编译期默认名。泛型 `AsRef<str>` 同时覆盖 lossy [`argv`] 产物与 repl 侧的
/// `&[impl AsRef<str>]` 入口，各 CLI 共此一读。
pub fn argv0<'a, S: AsRef<str>>(args: &'a [S], fallback: &'a str) -> &'a str {
  args.first().map_or(fallback, |s| s.as_ref())
}
