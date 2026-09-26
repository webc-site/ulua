use crate::records::{ast_type::AstType, ast_type_pack::AstTypePack};

#[repr(C)]
#[derive(Debug)]
pub struct AstTypePackVariadic {
  pub base: AstTypePack,
  pub variadic_type: *mut AstType,
}
