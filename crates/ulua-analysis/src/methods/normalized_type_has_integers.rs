use ulua_common::fflag;

use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{never_type::NeverType, normalized_type::NormalizedType},
};

impl NormalizedType {
  pub fn has_integers(&self) -> bool {
    if fflag::LuauIntegerType2.get() {
      get_type_id::<NeverType>(self.integers).is_none()
    } else {
      false
    }
  }
}
