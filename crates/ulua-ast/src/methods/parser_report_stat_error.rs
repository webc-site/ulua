use core::fmt::Arguments;

use crate::records::{
  ast_array::AstArray, ast_expr::AstExpr, ast_stat::AstStat, ast_stat_error::AstStatError,
  location::Location, parser::Parser,
};

impl Parser {
  pub fn report_stat_error(
    &mut self,
    location: Location,
    expressions: AstArray<*mut AstExpr>,
    statements: AstArray<*mut AstStat>,
    format: Arguments<'_>,
  ) -> *mut AstStatError {
    self.report(location, format);

    let message_index = (self.parse_errors.len() as u32).saturating_sub(1);

    unsafe {
      let allocator = &mut *self.allocator;
      allocator.alloc(AstStatError::new(
        location,
        expressions,
        statements,
        message_index,
      ))
    }
  }
}
