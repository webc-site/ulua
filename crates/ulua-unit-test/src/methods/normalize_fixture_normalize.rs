//! @interface-stub
use alloc::sync::Arc;

use ulua_analysis::{records::normalized_type::NormalizedType, type_aliases::type_id::TypeId};

use crate::records::normalize_fixture::NormalizeFixture;

impl NormalizeFixture {
  pub fn normalize(&mut self, ty: TypeId) -> Option<Arc<NormalizedType>> {
    self.get_frontend();
    self
      .normalizer
      .as_mut()
      .expect("NormalizeFixture normalizer")
      .try_normalize(ty)
  }
}
