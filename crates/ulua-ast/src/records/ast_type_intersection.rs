use crate::records::{ast_array::AstArray, ast_type::AstType};

#[repr(C)]
#[derive(Debug)]
pub struct AstTypeIntersection {
  pub base: AstType,
  pub types: AstArray<*mut AstType>,
}
