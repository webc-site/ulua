use core::{mem::zeroed, ptr::null_mut};

use crate::records::{ast_type::AstType, name::Name, position::Position};

#[derive(Debug, Clone)]
pub struct Binding {
  pub name: Name,
  pub annotation: *mut AstType,
  pub colon_position: Position,
  pub is_const: bool,
}

impl Default for Binding {
  fn default() -> Self {
    Self {
      name: unsafe { zeroed() },
      annotation: null_mut(),
      colon_position: Position { line: 0, column: 0 },
      is_const: false,
    }
  }
}
