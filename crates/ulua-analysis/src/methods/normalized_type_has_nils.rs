use crate::{
  functions::get_type,
  records::{never_type::NeverType, normalized_type::NormalizedType},
};

impl NormalizedType {
  pub fn has_nils(&self) -> bool {
    get_type::get::<NeverType>(self.nils).is_none()
  }
}
