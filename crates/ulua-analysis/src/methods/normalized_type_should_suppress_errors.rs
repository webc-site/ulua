use crate::{
  functions::get_type,
  records::{any_type::AnyType, normalized_type::NormalizedType},
};

impl NormalizedType {
  pub fn should_suppress_errors(&self) -> bool {
    self.has_errors() || get_type::get::<AnyType>(self.tops).is_some()
  }
}
