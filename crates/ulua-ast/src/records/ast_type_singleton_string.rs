use crate::records::{ast_array::AstArray, ast_type::AstType};

#[repr(C)]
#[derive(Debug)]
pub struct AstTypeSingletonString {
  pub base: AstType,
  pub value: AstArray<u8>,
}
