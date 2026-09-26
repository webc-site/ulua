#[repr(C)]
#[derive(Debug, Clone)]
pub struct AstTypeSingletonBool {
  pub base: AstType,
  pub value: bool,
}

use crate::records::ast_type::AstType;
