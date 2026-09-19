use ulua_ast::records::{ast_local::AstLocal, ast_name::AstName};

use crate::records::symbol::Symbol;
impl Symbol {
  pub fn symbol_ast_local(local: *mut AstLocal) -> Self {
    Symbol {
      local,
      global: AstName::default(),
    }
  }
}
