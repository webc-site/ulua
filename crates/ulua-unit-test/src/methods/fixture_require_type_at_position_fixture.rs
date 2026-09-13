use core::ptr::null_mut;

use ulua_analysis::type_aliases::type_id::TypeId;
use ulua_ast::records::position::Position;

use crate::records::fixture::Fixture;
impl Fixture {
  pub fn require_type_at_position_position(&mut self, position: Position) -> TypeId {
    let ty = self.find_type_at_position_module_name_position("".as_ref(), position);
    if let Some(ty) = ty {
      ty
    } else {
      ulua_common::LUAU_ASSERT!(false);
      null_mut()
    }
  }
}
