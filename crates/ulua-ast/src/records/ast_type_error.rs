use crate::records::{ast_array::AstArray, ast_type::AstType};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstTypeError {
  pub base: AstType,
  pub types: AstArray<*mut AstType>,
  pub is_missing: bool,
  pub message_index: u32,
}
