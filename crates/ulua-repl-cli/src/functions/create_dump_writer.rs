//! profiler/counters/coverage 三个 dump 函数共用的建文件开头：cpp 各自
//! `fopen(path, "wb")` 失败即 `printf("Error opening …")` + return，同构样板
//! 收口于此，调用点只留主题词差异。

use std::{fs::File, io::BufWriter};

/// cpp `fopen(path, "wb")` 的写模式打开并包 [`BufWriter`]（cpp fprintf 自带
/// 缓冲，等价 stdio 的隐式缓冲）；失败打印 `Error opening {subject} {path}`
/// 并返回 [`None`]，调用方据此提前返回。
pub(crate) fn create_dump_writer(path: &str, subject: &str) -> Option<BufWriter<File>> {
  match File::create(path) {
    Ok(file) => Some(BufWriter::new(file)),
    Err(_) => {
      eprintln!("Error opening {subject} {path}");
      None
    }
  }
}
