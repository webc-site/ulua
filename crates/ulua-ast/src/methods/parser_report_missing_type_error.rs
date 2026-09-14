use core::{fmt::Arguments, ptr::null_mut};

use crate::{
  records::{
    ast_array::AstArray, ast_node::AstNode, ast_type::AstType, ast_type_error::AstTypeError,
    location::Location, parser::Parser,
  },
  rtti::AstNodeClass,
};

impl Parser {
  pub fn report_missing_type_error(
    &mut self,
    parse_error_location: Location,
    ast_error_location: Location,
    format: Arguments<'_>,
  ) -> *mut AstTypeError {
    self.report(parse_error_location, format);

    let message_index = (self.parse_errors.len() as u32).saturating_sub(1);

    unsafe {
      (*self.allocator).alloc(AstTypeError {
        base: AstType {
          base: AstNode {
            class_index: AstTypeError::CLASS_INDEX,
            location: ast_error_location,
          },
        },
        types: AstArray {
          data: null_mut(),
          size: 0,
        },
        is_missing: true,
        message_index,
      })
    }
  }
}
