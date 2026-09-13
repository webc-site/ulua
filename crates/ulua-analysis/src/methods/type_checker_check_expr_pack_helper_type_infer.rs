use alloc::vec::Vec;

use ulua_ast::records::{ast_expr::AstExpr, ast_expr_call::AstExprCall};

use crate::{
  records::{
    ast_node::AstNode, type_checker::TypeChecker, type_pack::TypePack, type_pack_var::TypePackVar,
    with_predicate::WithPredicate,
  },
  rtti::{ast_node_as, ast_rtti_index},
  type_aliases::{scope_ptr_type::ScopePtr, type_pack_id::TypePackId},
};
impl TypeChecker {
  pub fn check_expr_pack_helper_scope_ptr_ast_expr(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExpr,
  ) -> WithPredicate<TypePackId> {
    let expr_ptr = expr as *const AstExpr as *mut AstExpr;
    if unsafe { (*expr_ptr).base.class_index == ast_rtti_index("AstExprCall") } {
      let call_expr = unsafe { &*ast_node_as::<AstExprCall>(expr_ptr as *mut AstNode) };
      self.check_expr_pack_helper_scope_ptr_ast_expr_call(scope, call_expr)
    } else if unsafe { (*expr_ptr).base.class_index == ast_rtti_index("AstExprVarargs") } {
      if scope.vararg_pack.is_none() {
        return WithPredicate::with_predicate_t(
          self.error_recovery_type_pack_scope_ptr(scope.clone()),
        );
      }
      WithPredicate::with_predicate_t(scope.vararg_pack.unwrap())
    } else {
      let type_result =
        self.check_expr_scope_ptr_ast_expr_optional_type_id_bool(scope, expr, None, false);
      WithPredicate::with_predicate_t(self.add_type_pack_type_pack_var(TypePackVar::from(
        TypePack {
          head: Vec::from([type_result.r#type]),
          tail: None,
        },
      )))
    }
  }
}
