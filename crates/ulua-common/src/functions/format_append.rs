//! Port of `Luau::formatAppend` from `Common/src/StringUtils.cpp`.
//!
//! See [`crate::functions::vformat_append`] for the documented deviation: the
//! C++ variadic `void formatAppend(std::string& str, const char* fmt, ...)`
//! becomes a `core::fmt::Arguments` consumer (callers pass `format_args!(...)`)
//! so the port stays on stable + `wasm32`.

mod _inner {
  use alloc::string::String;
  use core::fmt::Arguments;

  use crate::functions::vformat_append::vformat_append;

  pub fn format_append(str: &mut String, args: Arguments<'_>) {
    vformat_append(str, args);
  }
}

pub use _inner::{format_append, format_append as formatAppend};
