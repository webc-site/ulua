use core::ptr::{null, null_mut};

use ulua_ast::records::ast_name::AstName;
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_table::DenseDefault};

use crate::records::{global_linter::Global, lint_context::LintContext};
impl DenseDefault for Global {
  fn dense_default() -> Self {
    Self::default()
  }
}

impl LintContext {
  pub fn lint_context(&mut self) {
    self.root = null_mut();
    self.placeholder = AstName::new();
    self.builtin_globals = DenseHashMap::new(AstName::new());
    self.module = null();
  }
}
