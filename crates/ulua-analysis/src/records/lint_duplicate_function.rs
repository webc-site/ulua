use alloc::string::String;
use core::ffi::c_void;

use ulua_ast::records::{
  ast_stat_block::AstStatBlock, ast_visitor::AstVisitor, location::Location,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::lint_context::LintContext;
#[derive(Debug, Clone)]
pub struct LintDuplicateFunction {
  pub(crate) context: *mut LintContext,
  pub(crate) defns: DenseHashMap<String, Location>,
}

impl LintDuplicateFunction {
  pub fn new(context: *mut LintContext) -> Self {
    Self {
      context,
      defns: DenseHashMap::new(String::new()),
    }
  }
}

impl AstVisitor for LintDuplicateFunction {
  fn visit_stat_block(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_block(node as *mut AstStatBlock)
  }

  fn visit_expr(&mut self, _node: *mut c_void) -> bool {
    true
  }

  fn visit_stat(&mut self, _node: *mut c_void) -> bool {
    true
  }

  fn visit_type(&mut self, _node: *mut c_void) -> bool {
    false
  }

  fn visit_type_pack(&mut self, _node: *mut c_void) -> bool {
    false
  }
}
