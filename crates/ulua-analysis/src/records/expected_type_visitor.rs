use core::ffi::c_void;

use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_index_expr::AstExprIndexExpr,
  ast_expr_type_assertion::AstExprTypeAssertion, ast_node::AstNode, ast_stat_assign::AstStatAssign,
  ast_stat_compound_assign::AstStatCompoundAssign, ast_stat_local::AstStatLocal,
  ast_stat_return::AstStatReturn, ast_type::AstType, ast_visitor::AstVisitor,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{builtin_types::BuiltinTypes, scope::Scope, type_arena::TypeArena},
  type_aliases::type_id::TypeId,
};
#[derive(Debug, Clone)]
pub struct ExpectedTypeVisitor {
  pub(crate) ast_types: *mut DenseHashMap<*const AstExpr, TypeId>,
  pub(crate) ast_expected_types: *mut DenseHashMap<*const AstExpr, TypeId>,
  pub(crate) ast_resolved_types: *mut DenseHashMap<*const AstType, TypeId>,
  pub(crate) ast_overload_resolved_types: *mut DenseHashMap<*const AstNode, TypeId>,
  pub(crate) arena: *mut TypeArena,
  pub(crate) builtin_types: *mut BuiltinTypes,
  pub(crate) root_scope: *mut Scope,
}

impl AstVisitor for ExpectedTypeVisitor {
  fn visit_stat_assign(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_assign(node as *mut AstStatAssign)
  }

  fn visit_stat_local(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_local(node as *mut AstStatLocal)
  }

  fn visit_stat_compound_assign(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_compound_assign(node as *mut AstStatCompoundAssign)
  }

  fn visit_stat_return(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_stat_return(node as *mut AstStatReturn)
  }

  fn visit_expr_call(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_call(node as *mut AstExprCall)
  }

  fn visit_expr_index_expr(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_index_expr(node as *mut AstExprIndexExpr)
  }

  fn visit_expr_type_assertion(&mut self, node: *mut c_void) -> bool {
    self.visit_ast_expr_type_assertion(node as *mut AstExprTypeAssertion)
  }
}
