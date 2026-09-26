use alloc::string::String;
use core::fmt::{Arguments, Write};

pub(crate) fn append(result: &mut String, args: Arguments<'_>) {
  // write! 返回 fmt::Result；截断可接受（C++ 版同样
  // 截断到 256 字节缓冲区）。
  let _ = result.write_fmt(args);
}
