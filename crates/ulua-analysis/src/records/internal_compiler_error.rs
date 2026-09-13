use alloc::{ffi::CString, string::String};
use core::fmt::{Display, Formatter, Result};
use std::error::Error;

use ulua_ast::records::location::Location;
#[derive(Debug, Clone)]
pub struct InternalCompilerError {
  pub message: String,
  pub module_name: Option<String>,
  pub location: Option<Location>,
  /// A NUL-terminated copy of `message` for the C++-style `what()`, which
  /// returns `*const c_char` read with `CStr::from_ptr`. A Rust `String` is
  /// not NUL-terminated, so `message.as_ptr()` would over-read past the Buffer
  /// (UB; flaky garbage across allocators). Built once at construction via
  /// [`InternalCompilerError::new`] so the pointer stays valid for `&self`.
  pub(crate) c_message: CString,
}

impl InternalCompilerError {
  /// Build an `InternalCompilerError`, materializing the NUL-terminated
  /// `what()` view from `message`.
  pub fn new(message: String, module_name: Option<String>, location: Option<Location>) -> Self {
    let c_message = nul_terminated(&message);
    Self {
      message,
      module_name,
      location,
      c_message,
    }
  }
}

/// NUL-terminated C string from `s`, stripping any (never-expected) interior
/// NULs so construction cannot fail even mid-panic.
pub(crate) fn nul_terminated(s: &str) -> CString {
  match CString::new(s) {
    Ok(c) => c,
    Err(_) => CString::new(s.replace('\0', "")).unwrap_or_default(),
  }
}

unsafe impl Send for InternalCompilerError {}
unsafe impl Sync for InternalCompilerError {}

#[cfg(feature = "std")]
impl Error for InternalCompilerError {}

impl Display for InternalCompilerError {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "{}", self.message)
  }
}
