use alloc::string::String;
use core::fmt::{Arguments, Write};

pub(crate) fn append(result: &mut String, args: Arguments<'_>) {
  // write! returns fmt::Result; truncation is acceptable (the C++ version
  // also truncated to a 256-byte buffer).
  let _ = result.write_fmt(args);
}
