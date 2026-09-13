//! Faithful port of Luau `Lexeme` (`Ast/include/Luau/Lexer.h`).
//!
//! The token payload is a C++ union (`const char* data`/`name`, `unsigned
//! codepoint`); a Rust `union` reproduces it. Unions can derive `Clone`/`Copy`
//! but not `Debug`, so `LexemeData` gets a hand-written `Debug` (it can't know
//! which arm is active) and `Lexeme` then derives `Debug` normally.

use core::{
  ffi::c_char,
  fmt::{Debug, Formatter, Result},
  ptr::null,
};

pub use crate::enums::type_lexer::Type;
use crate::records::{ast_name::AstName, location::Location};

/// `Lexeme::QuoteStyle` (`Ast/include/Luau/Lexer.h`) — the delimiter of a quoted
/// string token, returned by `get_quote_style`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuoteStyle {
  Single,
  Double,
}

#[derive(Debug, Clone, Copy)]
pub struct Lexeme {
  pub r#type: Type,
  pub location: Location,
  pub(crate) length: u32,
  pub data: LexemeData,
}

impl Lexeme {
  /// Returns the name payload as an `AstName`.
  #[inline]
  pub fn name(&self) -> AstName {
    AstName {
      value: unsafe { self.data.name },
    }
  }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union LexemeData {
  pub data: *const c_char,
  pub name: *const c_char,
  pub codepoint: u32,
}

impl Debug for LexemeData {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    // The active arm is determined by `Lexeme::type`; print opaquely.
    f.write_str("LexemeData(..)")
  }
}

impl Default for LexemeData {
  fn default() -> Self {
    Self { data: null() }
  }
}
