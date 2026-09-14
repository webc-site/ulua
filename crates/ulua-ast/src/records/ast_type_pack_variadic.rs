use crate::{
  records::{ast_type::AstType, ast_type_pack::AstTypePack},
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstTypePackVariadic {
  pub base: AstTypePack,
  pub variadic_type: *mut AstType,
}

impl AstNodeClass for AstTypePackVariadic {
  const CLASS_INDEX: i32 = ast_rtti_index("AstTypePackVariadic");
}
