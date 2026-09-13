//! Source: `Compiler/include/Luau/Compiler.h:84` + Compiler.cpp (hand-ported)
//! C varargs printf-style raise -> core::fmt::Arguments (project-wide precedent).

use alloc::fmt::format;
use core::fmt::Arguments;
use std::panic::panic_any;

use ulua_ast::records::location::Location;

use crate::records::compile_error::CompileError;

impl CompileError {
  /// C++ `static LUAU_NORETURN void raise(const Location&, const char* format, ...)`
  /// Callers pass `format_args!(...)` (the varargs convention).
  pub fn raise(location: &Location, args: Arguments<'_>) -> ! {
    panic_any(CompileError::new(*location, format(args)))
  }
}

/// Free-fn spelling some earlier translations import.
pub fn compile_error_raise(location: Location, args: Arguments<'_>) -> ! {
  CompileError::raise(&location, args)
}
