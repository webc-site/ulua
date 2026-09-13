use ulua_ast::records::ast_name::AstName;
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::symbol::Symbol;

impl Symbol {
  pub fn ast_name(&self) -> AstName {
    if !self.local.is_null() {
      return unsafe { (*self.local).name };
    }

    LUAU_ASSERT!(!self.global.value.is_null());
    self.global
  }
}
