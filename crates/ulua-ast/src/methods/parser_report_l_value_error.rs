use crate::{
  records::{
    ast_expr::AstExpr, ast_expr_error::AstExprError, ast_expr_local::AstExprLocal,
    ast_node::AstNode, parser::Parser,
  },
  rtti::ast_node_as,
};

impl Parser {
  pub(crate) fn report_l_value_error(&mut self, expr: *mut AstExpr) -> *mut AstExprError {
    let local_expr = unsafe { ast_node_as::<AstExprLocal>(expr as *mut AstNode) };
    if !local_expr.is_null() {
      let local = unsafe { &*local_expr };
      if !local.local.is_null() && unsafe { (*local.local).is_const } {
        let location = unsafe { (*expr).base.location };
        let expressions = self.copy_initializer_list_t(&[expr]);
        let name = unsafe { (*local.local).name };
        return self.report_expr_error(
          location,
          expressions,
          format_args!("Variable '{}' is constant and may not be reassigned", name),
        );
      }
    }

    let location = unsafe { (*expr).base.location };
    let expressions = self.copy_initializer_list_t(&[expr]);
    self.report_expr_error(
      location,
      expressions,
      format_args!("Assigned expression must be a variable or a field"),
    )
  }
}
