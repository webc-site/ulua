use crate::{
  records::{ast_name::AstName, ast_type_pack::AstTypePack},
  rtti::{AstNodeClass, ast_rtti_index},
};

#[repr(C)]
#[derive(Debug)]
pub struct AstTypePackGeneric {
  pub base: AstTypePack,
  pub generic_name: AstName,
}

impl AstNodeClass for AstTypePackGeneric {
  const CLASS_INDEX: i32 = ast_rtti_index("AstTypePackGeneric");
}
