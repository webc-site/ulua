use ulua_ast::records::ast_stat_compound_assign::AstStatCompoundAssign;

use crate::{
  enums::value_context::ValueContext,
  records::{non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker},
};

impl NonStrictTypeChecker {
  pub(crate) fn visit_ast_stat_compound_assign(
    &mut self,
    compound_assign: *mut AstStatCompoundAssign,
  ) -> NonStrictContext {
    unsafe {
      let compound_assign = &*compound_assign;
      let var = compound_assign.var;
      let value = compound_assign.value;

      let _ = self.visit_ast_expr_value_context(var, ValueContext::LValue);
      let _ = self.visit_ast_expr_value_context(value, ValueContext::RValue);
    }

    NonStrictContext::new()
  }
}
