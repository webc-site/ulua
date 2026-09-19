use core::ptr::null_mut;

use ulua_ast::records::ast_name::AstName;

use crate::records::symbol::Symbol;
impl Symbol {
  pub fn symbol_ast_name(global: AstName) -> Self {
    Symbol {
      local: null_mut(),
      global,
    }
  }
}
