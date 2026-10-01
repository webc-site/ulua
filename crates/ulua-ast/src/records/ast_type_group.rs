use crate::records::ast_type::AstType;

#[repr(C)]
#[derive(Debug)]
pub struct AstTypeGroup {
  pub base: AstType,
  pub type_: *mut AstType,
}
