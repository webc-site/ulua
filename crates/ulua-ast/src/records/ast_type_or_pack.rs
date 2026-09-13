use core::ptr::null_mut;

use crate::records::{ast_type::AstType, ast_type_pack::AstTypePack};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AstTypeOrPack {
  pub r#type: *mut AstType,
  pub type_pack: *mut AstTypePack,
}

impl Default for AstTypeOrPack {
  fn default() -> Self {
    Self {
      r#type: null_mut(),
      type_pack: null_mut(),
    }
  }
}
