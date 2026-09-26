use crate::records::{
  ast_array::AstArray, ast_type::AstType, ast_type_error::AstTypeError, location::Location,
};

impl_ast_node_new!(
  AstTypeError,
  AstType,
  location: Location,
  types: AstArray<*mut AstType>,
  is_missing: bool,
  message_index: u32,
);
