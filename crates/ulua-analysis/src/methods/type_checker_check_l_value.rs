use ulua_ast::{
  records::{
    ast_expr::AstExpr, ast_expr_error::AstExprError, ast_expr_global::AstExprGlobal,
    ast_expr_index_expr::AstExprIndexExpr, ast_expr_index_name::AstExprIndexName,
    ast_expr_local::AstExprLocal,
  },
  rtti::{AstNodeClass, ast_node_as_unchecked},
};

use crate::{
  enums::value_context::ValueContext,
  records::type_checker::TypeChecker,
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};

impl TypeChecker {
  // cpp TypeInfer.cpp:3476 checkLValueBinding 的多路分发
  pub fn check_l_value(&mut self, scope: &ScopePtr, expr: &AstExpr, ctx: ValueContext) -> TypeId {
    match expr.base.class_index {
      AstExprLocal::CLASS_INDEX => {
        let local: &AstExprLocal = unsafe { ast_node_as_unchecked(&expr.base) };
        self.check_l_value_binding_scope_ptr_ast_expr_local(scope, local)
      }
      AstExprGlobal::CLASS_INDEX => {
        let global: &AstExprGlobal = unsafe { ast_node_as_unchecked(&expr.base) };
        self.check_l_value_binding_scope_ptr_ast_expr_global(scope, global)
      }
      AstExprIndexName::CLASS_INDEX => {
        let index_name: &AstExprIndexName = unsafe { ast_node_as_unchecked(&expr.base) };
        self
          .check_l_value_binding_scope_ptr_ast_expr_index_name_value_context(scope, index_name, ctx)
      }
      AstExprIndexExpr::CLASS_INDEX => {
        let index_expr: &AstExprIndexExpr = unsafe { ast_node_as_unchecked(&expr.base) };
        self
          .check_l_value_binding_scope_ptr_ast_expr_index_expr_value_context(scope, index_expr, ctx)
      }
      AstExprError::CLASS_INDEX => {
        let error: &AstExprError = unsafe { ast_node_as_unchecked(&expr.base) };
        for &sub_expr in error.expressions.as_slice() {
          self.check_expr(scope, unsafe { &*sub_expr }, None, false);
        }
        self.error_recovery_type_scope_ptr(scope)
      }
      _ => {
        self.ice_string_location("Unexpected AST node in checkLValue", &expr.base.location);
        self.error_recovery_type_scope_ptr(scope)
      }
    }
  }
}
