use crate::{
  functions::get_type,
  records::{never_type::NeverType, normalized_type::NormalizedType},
};

impl NormalizedType {
  pub fn has_booleans(&self) -> bool {
    get_type::get::<NeverType>(self.booleans).is_none()
  }
}
