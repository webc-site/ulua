use ulua_common::FFlag;

use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{never_type::NeverType, normalized_type::NormalizedType},
};

impl NormalizedType {
  pub fn has_integers(&self) -> bool {
    if FFlag::LuauIntegerType2.get() {
      get_type_id::<NeverType>(self.integers).is_none()
    } else {
      false
    }
  }
}
