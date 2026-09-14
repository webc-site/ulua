use core::ptr::null_mut;

use ulua_analysis::type_aliases::type_id::TypeId;
use ulua_ast::records::position::Position;

use crate::records::fixture::Fixture;
impl Fixture {
  pub fn require_type_at_position_module_name_position(
    &mut self,
    module_name: &str,
    position: Position,
  ) -> TypeId {
    let ty = self.find_type_at_position_module_name_position(module_name, position);
    ulua_common::LUAU_ASSERT!(ty.is_some());
    match ty {
      Some(ty) => ty,
      None => {
        // Mirror the C++ contract: assert that the type must exist.
        // Return a null pointer on mismatch to keep this function total.
        null_mut()
      }
    }
  }
}
