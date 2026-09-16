//! @interface-stub
use ulua_analysis::{records::normalized_type::NormalizedType, type_aliases::type_id::TypeId};

use crate::records::normalize_fixture::NormalizeFixture;

impl NormalizeFixture {
  pub fn type_from_normal(&mut self, norm: &NormalizedType) -> TypeId {
    self.get_frontend();
    self
      .normalizer
      .as_mut()
      .expect("NormalizeFixture normalizer")
      .type_from_normal(norm)
  }
}
