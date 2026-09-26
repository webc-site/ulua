//! Port of `Luau::format` / `Luau::vformat` from `Common/src/StringUtils.cpp`.
//!
//! 格式化形态的偏差说明见 [`super::format_append`]：C++ 的 `printf` 风格变参在
//! Rust 侧落成 `core::fmt::Arguments`，故 `format`/`vformat` 两个薄入口合并为一个
//! `format`。

use alloc::string::String;
use core::fmt::Arguments;

/// `args` 格式化为新串（cpp `format(...)` / `vformat(fmt, args)` 的单一实现）。
pub fn format(args: Arguments<'_>) -> String {
  // `Arguments` 实现 `Display`，`to_string` 即「建串 + `write_fmt`」的收口写法。
  args.to_string()
}
