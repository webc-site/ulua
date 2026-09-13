use core::ptr::null_mut;

use crate::{
  enums::polarity::Polarity,
  functions::fresh_index::fresh_index,
  records::{generic_type_pack::GenericTypePack, type_level::TypeLevel},
  type_aliases::name_type::Name,
};
impl GenericTypePack {
  pub fn new() -> Self {
    Self {
      index: fresh_index(),
      level: TypeLevel::default(),
      scope: null_mut(),
      name: Name::default(),
      explicit_name: false,
      polarity: Polarity::Unknown,
    }
  }

  pub fn new_name(name: Name) -> Self {
    Self {
      index: fresh_index(),
      level: TypeLevel::default(),
      scope: null_mut(),
      name,
      explicit_name: true,
      polarity: Polarity::Unknown,
    }
  }
}

impl Default for GenericTypePack {
  fn default() -> Self {
    Self::new()
  }
}
