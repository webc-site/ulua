use crate::{
  functions::get_type,
  records::{never_type::NeverType, normalized_type::NormalizedType},
};

impl NormalizedType {
  pub fn has_numbers(&self) -> bool {
    get_type::get::<NeverType>(self.numbers).is_none()
  }
}
