use ulua_analysis::{functions::follow_type::follow_type_id, type_aliases::type_id::TypeId};

use crate::records::fixture::Fixture;
impl Fixture {
  pub fn require_type_alias(&mut self, name: &str) -> TypeId {
    let ty = self.lookup_type(name);
    let ty = ty.expect("type alias not found");
    follow_type_id(ty)
  }
}
