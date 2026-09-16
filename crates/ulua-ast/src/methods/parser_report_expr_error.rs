use core::fmt::Arguments;

use crate::{
  records::{
    allocator::Allocator, ast_array::AstArray, ast_expr::AstExpr, ast_expr_error::AstExprError,
    ast_node::AstNode, location::Location, parser::Parser,
  },
  rtti::AstNodeClass,
};

impl Parser {
  pub fn report_expr_error(
    &mut self,
    location: Location,
    expressions: AstArray<*mut AstExpr>,
    format: Arguments<'_>,
  ) -> *mut AstExprError {
    self.report(location, format);

    let message_index = (self.parse_errors.len() as u32).saturating_sub(1);

    unsafe {
      Allocator::alloc(
        &mut *self.allocator,
        AstExprError {
          base: AstExpr {
            base: AstNode {
              class_index: AstExprError::CLASS_INDEX,
              location,
            },
          },
          expressions,
          message_index,
        },
      )
    }
  }
}
