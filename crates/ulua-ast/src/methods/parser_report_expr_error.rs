use core::fmt::Arguments;

use crate::records::{
  ast_array::AstArray, ast_expr::AstExpr, ast_expr_error::AstExprError, location::Location,
  parser::Parser,
};

impl Parser {
  pub fn report_expr_error(
    &mut self,
    location: Location,
    expressions: AstArray<*mut AstExpr>,
    format: Arguments<'_>,
  ) -> *mut AstExpr {
    self.report(location, format);

    let message_index = (self.parse_errors.len() as u32).saturating_sub(1);

    self.alloc_expr(AstExprError::new(location, expressions, message_index))
  }
}
