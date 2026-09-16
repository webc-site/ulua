use alloc::vec::Vec;
use core::ptr::null_mut;

use ulua_ast::records::ast_name::AstName;
use ulua_common::records::{dense_hash_map::DenseHashMap, dense_hash_table::DenseDefault};

use crate::records::{global_linter_alt_b::Global, lint_global_local::LintGlobalLocal};
impl DenseDefault for Global {
  fn dense_default() -> Self {
    Self::default()
  }
}

impl LintGlobalLocal {
  pub fn new() -> Self {
    Self {
      context: null_mut(),
      globals: DenseHashMap::new(AstName::new()),
      global_refs: Vec::new(),
      function_stack: Vec::new(),
    }
  }
}

impl Default for LintGlobalLocal {
  fn default() -> Self {
    Self::new()
  }
}
