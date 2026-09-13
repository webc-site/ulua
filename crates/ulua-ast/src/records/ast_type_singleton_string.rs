use core::ffi::c_char;

use crate::{
  records::{ast_array::AstArray, ast_type::AstType},
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstTypeSingletonString {
  pub base: AstType,
  pub value: AstArray<c_char>,
}

impl AstNodeClass for AstTypeSingletonString {
  const CLASS_INDEX: i32 = ast_rtti_index("AstTypeSingletonString");
}
