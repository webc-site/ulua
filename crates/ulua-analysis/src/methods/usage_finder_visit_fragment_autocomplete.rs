use core::str::from_utf8;

use ulua_ast::records::ast_expr_constant_string::AstExprConstantString;

use crate::records::usage_finder::UsageFinder;
impl UsageFinder {
  pub(crate) fn visit_ast_expr_constant_string(
    &mut self,
    expr: *mut AstExprConstantString,
  ) -> bool {
    let expr_ref = unsafe { &*expr };
    let value_slice = expr_ref.value.as_bytes();
    let name = from_utf8(value_slice).unwrap_or("");
    self.referenced_bindings.push(name.into());
    true
  }
}
