use crate::{
  records::{ast_array::AstArray, ast_type::AstType},
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstTypeUnion {
  pub base: AstType,
  pub types: AstArray<*mut AstType>,
}

impl AstNodeClass for AstTypeUnion {
  const CLASS_INDEX: i32 = ast_rtti_index("AstTypeUnion");
}
