use core::ptr::from_mut;

use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_index_expr::AstExprIndexExpr,
  ast_expr_type_assertion::AstExprTypeAssertion, ast_node::AstNode, ast_stat_assign::AstStatAssign,
  ast_stat_compound_assign::AstStatCompoundAssign, ast_stat_local::AstStatLocal,
  ast_stat_return::AstStatReturn, ast_type::AstType, ast_visitor::AstVisitor,
};
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  records::{
    arena_handle::{Handle, alias},
    builtin_types::BuiltinTypes,
    scope::Scope,
    type_arena::TypeArena,
  },
  type_aliases::type_id::TypeId,
};
#[derive(Debug, Clone)]
pub struct ExpectedTypeVisitor {
  pub(crate) ast_types: *mut DenseHashMap<*const AstExpr, TypeId>,
  pub(crate) ast_expected_types: *mut DenseHashMap<*const AstExpr, TypeId>,
  pub(crate) ast_resolved_types: *mut DenseHashMap<*const AstType, TypeId>,
  pub(crate) ast_overload_resolved_types: *mut DenseHashMap<*const AstNode, TypeId>,
  pub(crate) arena: Handle<TypeArena>,
  pub(crate) builtin_types: Handle<BuiltinTypes>,
  pub(crate) root_scope: *mut Scope,
}

impl ExpectedTypeVisitor {
  /// §2 裸指针收口单点：`root_scope` 由构造期注入（C++ `NotNull<Scope>` 直译），
  /// 遍历会话期内存活且非空；解引用只发生在 [`arena_handle::alias`] 一处。
  /// `find_narrowest_scope_containing` 需要可变自借（cpp 同款），故取可变形态。
  pub(crate) fn root_scope_mut(&mut self) -> &mut Scope {
    alias(self.root_scope)
  }
}

impl AstVisitor for ExpectedTypeVisitor {
  fn visit_stat_assign(&mut self, node: &mut AstStatAssign) -> bool {
    self.visit_ast_stat_assign(from_mut(node))
  }

  fn visit_stat_local(&mut self, node: &mut AstStatLocal) -> bool {
    self.visit_ast_stat_local(from_mut(node))
  }

  fn visit_stat_compound_assign(&mut self, node: &mut AstStatCompoundAssign) -> bool {
    self.visit_ast_stat_compound_assign(from_mut(node))
  }

  fn visit_stat_return(&mut self, node: &mut AstStatReturn) -> bool {
    self.visit_ast_stat_return(from_mut(node))
  }

  fn visit_expr_call(&mut self, node: &mut AstExprCall) -> bool {
    self.visit_ast_expr_call(from_mut(node))
  }

  fn visit_expr_index_expr(&mut self, node: &mut AstExprIndexExpr) -> bool {
    self.visit_ast_expr_index_expr(from_mut(node))
  }

  fn visit_expr_type_assertion(&mut self, node: &mut AstExprTypeAssertion) -> bool {
    self.visit_ast_expr_type_assertion(from_mut(node))
  }
}
