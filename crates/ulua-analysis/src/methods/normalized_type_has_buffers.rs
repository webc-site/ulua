use crate::{
  functions::get_type,
  records::{never_type::NeverType, normalized_type::NormalizedType},
};

impl NormalizedType {
  pub fn has_buffers(&self) -> bool {
    get_type::get::<NeverType>(self.buffers).is_none()
  }
}
