use core::fmt::Arguments;

use crate::{
  records::{
    ast_array::AstArray, ast_node::AstNode, ast_type::AstType, ast_type_error::AstTypeError,
    location::Location, parser::Parser,
  },
  rtti::AstNodeClass,
};

impl Parser {
  pub fn report_type_error(
    &mut self,
    location: Location,
    types: AstArray<*mut AstType>,
    args: Arguments<'_>,
  ) -> *mut AstTypeError {
    self.report(location, args);

    let message_index = (self.parse_errors.len() as u32).saturating_sub(1);

    unsafe {
      let allocator = &mut *self.allocator;
      allocator.alloc(AstTypeError {
        base: AstType {
          base: AstNode {
            class_index: AstTypeError::CLASS_INDEX,
            location,
          },
        },
        types,
        is_missing: false,
        message_index,
      })
    }
  }
}
