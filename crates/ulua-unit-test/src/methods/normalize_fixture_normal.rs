//! @interface-stub
use ulua_analysis::type_aliases::type_id::TypeId;

use crate::records::normalize_fixture::NormalizeFixture;

impl NormalizeFixture {
  pub fn normal(&mut self, annotation: &str) -> TypeId {
    self.get_frontend();
    let norm = self
      .to_normalized_type(annotation, 0)
      .expect("expected normalized type");
    self.type_from_normal(norm.as_ref())
  }
}
