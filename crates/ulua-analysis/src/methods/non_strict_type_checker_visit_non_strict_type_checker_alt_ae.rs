use ulua_ast::records::ast_expr_local::AstExprLocal;

use crate::{
  enums::value_context::ValueContext,
  records::{non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker},
};

impl NonStrictTypeChecker {
  pub fn visit_ast_expr_local_value_context(
    &mut self,
    _local: *mut AstExprLocal,
    _context: ValueContext,
  ) -> NonStrictContext {
    // C++ `visit(AstExprLocal*, ValueContext) { return {}; }`
    NonStrictContext::new()
  }
}
