use crate::{
  functions::get_type_alt_j::get_type_id,
  records::{never_type::NeverType, normalized_type::NormalizedType},
};

impl NormalizedType {
  pub fn has_booleans(&self) -> bool {
    get_type_id::<NeverType>(self.booleans).is_none()
  }
}
