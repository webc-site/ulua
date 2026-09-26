use core::fmt::Arguments;

use crate::records::{
  ast_array::AstArray, ast_type::AstType, ast_type_error::AstTypeError, location::Location,
  parser::Parser,
};

impl Parser {
  pub fn report_missing_type_error(
    &mut self,
    parse_error_location: Location,
    ast_error_location: Location,
    format: Arguments<'_>,
  ) -> *mut AstType {
    self.report(parse_error_location, format);

    let message_index = (self.parse_errors.len() as u32).saturating_sub(1);

    self.alloc_type(AstTypeError::new(
      ast_error_location,
      AstArray::EMPTY,
      true, // is_missing
      message_index,
    ))
  }
}
