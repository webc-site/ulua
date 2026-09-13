use core::ptr::null_mut;

use crate::{
  enums::polarity::Polarity,
  functions::fresh_index::fresh_index,
  records::{generic_type::GenericType, type_level::TypeLevel},
  type_aliases::name_type::Name,
};
impl GenericType {
  pub fn generic_type_name_polarity(name: &Name, polarity: Polarity) -> Self {
    GenericType {
      index: fresh_index(),
      level: TypeLevel::default(),
      scope: null_mut(),
      name: name.clone(),
      explicit_name: true,
      polarity,
    }
  }
}
