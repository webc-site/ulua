#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstTypeSingletonBool {
  pub base: AstType,
  pub value: bool,
}

impl AstNodeClass for AstTypeSingletonBool {
  const CLASS_INDEX: i32 = ast_rtti_index("AstTypeSingletonBool");
}
use crate::{
  records::ast_type::AstType,
  rtti::{AstNodeClass, ast_rtti_index},
};
