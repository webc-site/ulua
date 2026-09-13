use core::ptr::null_mut;

use crate::records::{ast_array::AstArray, ast_type::AstType, ast_type_pack::AstTypePack};

#[derive(Debug, Clone, Copy)]
pub struct AstTypeList {
  pub types: AstArray<*mut AstType>,
  pub tail_type: *mut AstTypePack,
}

impl Default for AstTypeList {
  fn default() -> Self {
    Self {
      types: AstArray {
        data: null_mut(),
        size: 0,
      },
      tail_type: null_mut(),
    }
  }
}
