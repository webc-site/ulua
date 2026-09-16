use ulua_ast::records::{
  ast_expr_binary::AstExprBinary, ast_stat_compound_assign::AstStatCompoundAssign,
};

use crate::{
  enums::control_flow::ControlFlow, records::type_checker::TypeChecker,
  type_aliases::scope_ptr_type::ScopePtr,
};

impl TypeChecker {
  pub fn check_scope_ptr_ast_stat_compound_assign(
    &mut self,
    scope: &ScopePtr,
    assign: &AstStatCompoundAssign,
  ) -> ControlFlow {
    let expr = AstExprBinary::new(
      assign.base.base.location,
      assign.op,
      assign.var,
      assign.value,
    );

    let left = self
      .check_expr_scope_ptr_ast_expr_optional_type_id_bool(
        scope,
        unsafe { &*expr.left },
        None,
        false,
      )
      .r#type;

    let right = self
      .check_expr_scope_ptr_ast_expr_optional_type_id_bool(
        scope,
        unsafe { &*expr.right },
        None,
        false,
      )
      .r#type;

    let result = self.check_binary_operation(scope, &expr, left, right, &Default::default());

    self.unify_type_id_type_id_scope_ptr_location(result, left, scope, &assign.base.base.location);

    ControlFlow::None
  }
}
