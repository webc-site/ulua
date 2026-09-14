use core::ptr::null_mut;

use crate::records::symbol::Symbol;
impl Symbol {
  pub fn new() -> Self {
    Symbol::symbol_ast_local(null_mut())
  }
}
