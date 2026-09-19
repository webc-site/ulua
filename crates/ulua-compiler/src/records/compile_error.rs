extern crate alloc;

use alloc::{ffi::CString, string::String};
use core::fmt::{Display, Formatter, Result};
use std::error::Error;

use ulua_ast::records::location::Location;

/// 编译期常量/跳转超限的通用报错文案（C++ 端字面量，勿改文案以免影响差异测试）。
pub(crate) const ERR_EXCEEDED_CONSTANT_LIMIT: &str =
  "Exceeded constant limit; simplify the code to compile";

/// 跳转距离超限的通用报错文案。
pub(crate) const ERR_EXCEEDED_JUMP_DISTANCE_LIMIT: &str =
  "Exceeded jump distance limit; simplify the code to compile";

#[derive(Debug, Clone)]
pub struct CompileError {
  pub(crate) location: Location,
  pub(crate) message: String,
  /// A NUL-terminated copy of `message` for the C++-style [`CompileError::what`],
  /// which returns `*const c_char` and is read by callers with `CStr::from_ptr`.
  ///
  /// A Rust `String` is **not** NUL-terminated, so handing out
  /// `message.as_ptr()` makes the reader over-run past the Buffer into adjacent
  /// memory — undefined behavior that surfaces as flaky trailing garbage
  /// depending on the allocator / load (the cross-platform failure that
  /// followed issue #3). Materialized once at construction so the pointer stays
  /// valid for `&self`'s lifetime, mirroring C++'s `std::string::c_str()`.
  pub(crate) c_message: CString,
}

impl CompileError {
  /// Build a `CompileError`, materializing the NUL-terminated `what()` view.
  pub(crate) fn new(location: Location, message: String) -> CompileError {
    let c_message = nul_terminated(&message);
    CompileError {
      location,
      message,
      c_message,
    }
  }
}

/// Build a NUL-terminated C string from `s`. Compile-error messages never
/// contain interior NUL bytes; strip any defensively so construction (which may
/// run while a panic is being raised) can never itself fail.
pub(crate) fn nul_terminated(s: &str) -> CString {
  match CString::new(s) {
    Ok(c) => c,
    Err(_) => CString::new(s.replace('\0', "")).unwrap_or_default(),
  }
}

impl Display for CompileError {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}", self.message)
  }
}

impl Error for CompileError {}
