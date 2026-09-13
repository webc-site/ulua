use core::ptr::null_mut;

use crate::{
  enums::polarity::Polarity,
  functions::fresh_index::fresh_index,
  records::{generic_type::GenericType, type_level::TypeLevel},
  type_aliases::name_type::Name,
};
impl GenericType {
  pub fn generic_type_type_level_name(_level: TypeLevel, _name: &Name) -> Self {
    let index = fresh_index();
    GenericType {
      index,
      level: _level,
      scope: null_mut(),
      name: _name.clone(),
      explicit_name: true,
      polarity: Polarity::Unknown,
    }
  }
}
