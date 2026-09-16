//! Node: `cxx:Function:Luau.Analysis:Analysis/src/Symbol.cpp:21:to_string`
//! Source: `Analysis/src/Symbol.cpp:21-28` (hand-ported)
extern crate alloc;

use alloc::string::String;

use crate::records::symbol::Symbol;

/// C++ `std::string to_string(const Symbol& name)`.
pub fn to_string(name: &Symbol) -> String {
  name.name().to_string()
}

pub use to_string as to_string_symbol;
