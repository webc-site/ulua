use core::ptr::null_mut;

use crate::{
  enums::polarity::Polarity,
  functions::fresh_index::fresh_index,
  records::{generic_type::GenericType, type_level::TypeLevel},
  type_aliases::name_type::Name,
};
impl GenericType {
  pub fn generic_type_type_level(_level: TypeLevel) -> Self {
    let index = fresh_index();
    let name = Name::from(format!("g{}", index).as_str());
    GenericType {
      index,
      level: _level,
      scope: null_mut(),
      name,
      explicit_name: false,
      polarity: Polarity::Unknown,
    }
  }
}
