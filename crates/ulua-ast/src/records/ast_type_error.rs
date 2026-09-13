use crate::{
  records::{ast_array::AstArray, ast_type::AstType},
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstTypeError {
  pub base: AstType,
  pub types: AstArray<*mut AstType>,
  pub is_missing: bool,
  pub message_index: u32,
}

impl AstNodeClass for AstTypeError {
  const CLASS_INDEX: i32 = ast_rtti_index("AstTypeError");
}
