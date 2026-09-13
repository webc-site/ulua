use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{any_type::AnyType, normalized_type::NormalizedType},
};

impl NormalizedType {
  pub fn should_suppress_errors(&self) -> bool {
    self.has_errors() || !get_type_id::<AnyType>(self.tops).is_none()
  }
}
