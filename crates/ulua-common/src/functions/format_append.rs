//! Port of `Luau::formatAppend` / `Luau::vformatAppend` from
//! `Common/src/StringUtils.cpp`.
//!
//! **Deviation (documented, behavior-faithful):** the C++ originals are
//! `printf`-style variadics (`void formatAppend(std::string& str, const char*
//! fmt, ...)`, `void vformatAppend(std::string& ret, const char* fmt, va_list
//! args)`) built on `vsnprintf`. C `va_list`/varargs have no stable Rust
//! equivalent, and this crate targets stable + `wasm32`, so the formatting
//! mechanism is `core::fmt`: callers pass `core::fmt::Arguments` (produced by
//! `format_args!`) instead of a `%`-format string plus a `va_list`. The
//! observable effect — appending formatted text to a string — is preserved;
//! only the *spelling* of the format moves to Rust's `{}`.

use alloc::string::String;
use core::fmt::{Arguments, Write};

/// 把 `args` 格式化后追加到 `sink`（cpp `formatAppend`/`vformatAppend` 的单一实现：
/// 两者在 Rust 侧只差有无 `va_list`，而 `Arguments` 已同时覆盖两形态）。
/// builder 式的 `&mut String` sink 是 Rust 惯用法，不改返回值。
pub fn format_append(sink: &mut String, args: Arguments<'_>) {
  // 写入 `String` 不会失败（无 IO、容量自动增长），因此丢弃 `Result`。
  let _ = sink.write_fmt(args);
}
