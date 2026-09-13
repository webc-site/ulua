//! Port of `Luau::vformat` from `Common/src/StringUtils.cpp`.
//!
//! See [`crate::functions::vformat_append`] for the documented deviation: the
//! C++ `std::string vformat(const char* fmt, va_list args)` becomes a
//! `core::fmt::Arguments` consumer so the port stays on stable + `wasm32`.

use alloc::string::String;
use core::fmt::Arguments;

use crate::functions::vformat_append::vformatAppend;

pub fn vformat(args: Arguments<'_>) -> String {
  let mut ret = String::new();
  vformatAppend(&mut ret, args);
  ret
}
