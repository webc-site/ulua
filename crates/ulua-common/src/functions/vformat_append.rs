//! Port of `Luau::vformatAppend` from `Common/src/StringUtils.cpp`.
//!
//! **Deviation (documented, behaviorally faithful):** the C++ original is a
//! `printf`-style `void vformatAppend(std::string& ret, const char* fmt,
//! va_list args)` built on `vsnprintf`. C `va_list`/varargs have no stable Rust
//! equivalent, and this crate targets stable + `wasm32`, so the formatting
//! mechanism is replaced by `core::fmt`: callers pass `core::fmt::Arguments`
//! (produced by `format_args!`) instead of a `printf` format string plus a
//! `va_list`. The observable effect — appending formatted text to a string — is
//! preserved; only the *spelling* of the format moves from `%`-specifiers to
//! Rust's `{}` at the (eventually translated) call sites.

mod _inner {
  use alloc::string::String;
  use core::fmt::{Arguments, Write};

  pub fn vformat_append(ret: &mut String, args: Arguments<'_>) {
    // 写入 String 不会失败，因此丢弃 Result。
    let _ = ret.write_fmt(args);
  }
}

pub use _inner::{vformat_append, vformat_append as vformatAppend};
