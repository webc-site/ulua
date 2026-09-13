use ulua_ast::records::ast_stat_return::AstStatReturn;

use crate::{
  enums::value_context::ValueContext,
  records::{non_strict_context::NonStrictContext, non_strict_type_checker::NonStrictTypeChecker},
};

impl NonStrictTypeChecker {
  pub(crate) fn visit_ast_stat_return(
    &mut self,
    return_statement: *mut AstStatReturn,
  ) -> NonStrictContext {
    let return_statement_ref = unsafe { &*return_statement };
    let list = return_statement_ref.list;
    for &expr in list.as_slice() {
      let _ = self.visit_ast_expr_value_context(expr, ValueContext::RValue);
    }
    NonStrictContext::new()
  }
}
