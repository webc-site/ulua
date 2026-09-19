use crate::{
  records::{ast_array::AstArray, ast_type::AstType},
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstTypeIntersection {
  pub base: AstType,
  pub types: AstArray<*mut AstType>,
}

impl AstNodeClass for AstTypeIntersection {
  const CLASS_INDEX: i32 = ast_rtti_index("AstTypeIntersection");
}
