use crate::records::{ast_name::AstName, ast_type_pack::AstTypePack};

#[repr(C)]
#[derive(Debug)]
pub struct AstTypePackGeneric {
  pub base: AstTypePack,
  pub generic_name: AstName,
}
