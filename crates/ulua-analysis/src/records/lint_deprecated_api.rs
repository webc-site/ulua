use alloc::vec::Vec;
use core::ffi::c_void;

use ulua_ast::records::{
  ast_expr_call::AstExprCall, ast_expr_global::AstExprGlobal,
  ast_expr_index_name::AstExprIndexName, ast_expr_local::AstExprLocal,
  ast_stat_function::AstStatFunction, ast_stat_local_function::AstStatLocalFunction,
  ast_visitor::AstVisitor,
};

use crate::records::{function_type::FunctionType, lint_context::LintContext};
#[derive(Debug, Clone)]
pub struct LintDeprecatedApi {
  pub(crate) context: *mut LintContext,
  pub(crate) function_type_scope_stack: Vec<*const FunctionType>,
}

impl LintDeprecatedApi {
  pub fn lint_deprecated_api(&mut self, context: *mut LintContext) {
    self.context = context;
    self.function_type_scope_stack = Vec::new();
  }
}

impl AstVisitor for LintDeprecatedApi {
  fn visit_node(&mut self, _node: *mut c_void) -> bool {
    true
  }

  fn visit_expr_index_name(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_index_name(node as *mut AstExprIndexName)
  }

  fn visit_expr_local(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_local(node as *mut AstExprLocal)
  }

  fn visit_expr_global(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_global(node as *mut AstExprGlobal)
  }

  fn visit_expr_call(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_call(node as *mut AstExprCall)
  }

  fn visit_stat_local_function(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_local_function(node as *mut AstStatLocalFunction)
  }

  fn visit_stat_function(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_function(node as *mut AstStatFunction)
  }

  fn visit_attr(&mut self, _node: *mut c_void) -> bool {
    false
  }

  fn visit_type(&mut self, _node: *mut c_void) -> bool {
    false
  }

  fn visit_type_pack(&mut self, _node: *mut c_void) -> bool {
    false
  }
}
