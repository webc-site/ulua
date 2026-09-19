use core::ptr::null_mut;

use ulua_ast::records::ast_name::AstName;
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_table::DenseDefault};

use crate::records::{
  global_linter_alt_c::Global, lint_local_hygiene::LintLocalHygiene, local_linter::Local,
};
impl DenseDefault for Global {
  fn dense_default() -> Self {
    Self::default()
  }
}

impl DenseDefault for Local {
  fn dense_default() -> Self {
    Self::default()
  }
}

impl LintLocalHygiene {
  pub fn new() -> Self {
    LintLocalHygiene {
      context: null_mut(),
      locals: DenseHashMap::new(null_mut()),
      imports: DenseHashMap::new(AstName::new()),
      globals: DenseHashMap::new(AstName::new()),
    }
  }
}

impl Default for LintLocalHygiene {
  fn default() -> Self {
    Self::new()
  }
}
