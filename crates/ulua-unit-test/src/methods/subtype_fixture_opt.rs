use ulua_analysis::type_aliases::type_id::TypeId;

use crate::records::subtype_fixture::SubtypeFixture;

impl SubtypeFixture {
  pub fn opt(&mut self, ty: TypeId) -> TypeId {
    self.join(ty, self.builtin_types.nil_type)
  }
}
