use ulua_ast::{enums::ast_expr_ref::AstExprRef, records::ast_expr::AstExpr};

use crate::{
  enums::value_context::ValueContext,
  records::type_checker::TypeChecker,
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};

impl TypeChecker {
  // cpp TypeInfer.cpp:3476 checkLValueBinding 的多路分发
  pub fn check_l_value(&mut self, scope: &ScopePtr, expr: &AstExpr, ctx: ValueContext) -> TypeId {
    match expr.as_expr_ref() {
      AstExprRef::Local(local) => self.check_l_value_binding_scope_ptr_ast_expr_local(scope, local),
      AstExprRef::Global(global) => {
        self.check_l_value_binding_scope_ptr_ast_expr_global(scope, global)
      }
      AstExprRef::IndexName(index_name) => self
        .check_l_value_binding_scope_ptr_ast_expr_index_name_value_context(scope, index_name, ctx),
      AstExprRef::IndexExpr(index_expr) => self
        .check_l_value_binding_scope_ptr_ast_expr_index_expr_value_context(scope, index_expr, ctx),
      AstExprRef::Error(error) => {
        for sub_expr in error.expressions.iter_nodes() {
          self.check_expr(scope, sub_expr, None, false);
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
