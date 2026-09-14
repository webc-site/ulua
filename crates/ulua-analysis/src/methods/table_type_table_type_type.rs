use core::ptr::null_mut;

use crate::{
  enums::table_state::TableState,
  records::{table_type::TableType, type_level::TypeLevel},
};
impl TableType {
  pub fn new() -> Self {
    Self::table_type_table_state_type_level_scope(
      TableState::Unsealed,
      TypeLevel::default(),
      null_mut(),
    )
  }
}

impl Default for TableType {
  fn default() -> Self {
    Self::new()
  }
}
