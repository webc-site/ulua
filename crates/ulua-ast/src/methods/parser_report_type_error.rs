use core::fmt::Arguments;

use crate::records::{
  ast_array::AstArray, ast_type::AstType, ast_type_error::AstTypeError, location::Location,
  parser::Parser,
};

impl Parser {
  pub fn report_type_error(
    &mut self,
    location: Location,
    types: AstArray<*mut AstType>,
    args: Arguments<'_>,
  ) -> *mut AstType {
    self.report(location, args);

    let message_index = (self.parse_errors.len() as u32).saturating_sub(1);

    self.alloc_type(AstTypeError::new(location, types, false, message_index))
  }
}
