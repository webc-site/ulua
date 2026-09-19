//! cpp 各 CLI 入口签名都是 `main(int argc, char** argv)`，`argv` 里的字节直接
//! 当文件名用；Rust 的 [`std::env::args`] 遇到非 UTF-8 参数会 panic。统一用本
//! 函数收敛一次，非法字节转为 U+FFFD，行为与上游一致（文件名照样打不开并报错，
//! 而不是进程崩溃）。

use alloc::{string::String, vec::Vec};
use std::env::args_os;

/// 返回 lossy 转换后的命令行参数，含程序名。
pub fn argv() -> Vec<String> {
  args_os()
    .map(|arg| arg.to_string_lossy().into_owned())
    .collect()
}
