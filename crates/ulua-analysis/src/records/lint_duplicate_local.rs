use core::{ffi::c_void, ptr::null_mut};

use ulua_ast::records::{
  ast_expr_function::AstExprFunction, ast_local::AstLocal, ast_node::AstNode,
  ast_stat_local::AstStatLocal, ast_visitor::AstVisitor,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::records::lint_context::LintContext;
#[derive(Debug, Clone)]
pub struct LintDuplicateLocal {
  pub(crate) context: *mut LintContext,
  pub(crate) locals: DenseHashMap<*mut AstLocal, *mut AstNode>,
}

impl LintDuplicateLocal {
  pub fn new() -> Self {
    Self {
      context: null_mut(),
      locals: DenseHashMap::new(null_mut()),
    }
  }

  /// # Safety
  /// 调用方须保证 `local` 等裸指针参数有效，且满足 C++ 原实现的调用契约。
  pub unsafe fn ignore_duplicate(&self, local: *mut AstLocal) -> bool {
    let local = unsafe { &*local };
    local.name.as_bytes() == b"_"
  }
}

impl Default for LintDuplicateLocal {
  fn default() -> Self {
    Self::new()
  }
}

impl AstVisitor for LintDuplicateLocal {
  fn visit_stat_local(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_local(node as *mut AstStatLocal)
  }

  fn visit_expr_function(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_function(node as *mut AstExprFunction)
  }

  fn visit_node(&mut self, _node: *mut c_void) -> bool {
    true
  }

  fn visit_type(&mut self, _node: *mut c_void) -> bool {
    false
  }

  fn visit_type_pack(&mut self, _node: *mut c_void) -> bool {
    false
  }
}
