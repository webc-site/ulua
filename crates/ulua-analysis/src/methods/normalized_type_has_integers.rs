use ulua_common::fflag;

use crate::{
  functions::get_type,
  records::{never_type::NeverType, normalized_type::NormalizedType},
};

impl NormalizedType {
  pub fn has_integers(&self) -> bool {
    if fflag::LuauIntegerType2.get() {
      get_type::get::<NeverType>(self.integers).is_none()
    } else {
      false
    }
  }
}
