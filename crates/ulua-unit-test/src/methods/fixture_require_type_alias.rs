use ulua_analysis::{functions::follow_type, type_aliases::type_id::TypeId};

use crate::records::fixture::Fixture;
impl Fixture {
  pub fn require_type_alias(&mut self, name: &str) -> TypeId {
    let ty = self.lookup_type(name);
    let ty = ty.expect("type alias not found");
    follow_type::follow(ty)
  }
}
