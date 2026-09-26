//! 汇编 builder 共享的行为 trait。
//!
//! C++ 中对 `AssemblyBuilder`（X64/A64）的模板代码会调用 `build.logAppend(...)`；
//! Rust 版把该能力类型擦除，收敛到此 trait 之后。

use core::fmt::Arguments;
pub trait LogAppend {
  fn log_append(&mut self, args: Arguments<'_>);
}

pub mod tag_access;
