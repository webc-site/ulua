//! Shared behavior traits for the assembly builders.
//!
//! C++ templates over `AssemblyBuilder` (X64/A64) call `build.logAppend(...)`;
//! the Rust translation type-erases that capability behind this trait.

use core::fmt::Arguments;
pub trait LogAppend {
  fn log_append(&mut self, args: Arguments<'_>);
}
