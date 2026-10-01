use ulua_analysis::type_aliases::type_id::TypeId;
use ulua_ast::records::position::Position;

use crate::records::fixture::Fixture;

impl Fixture {
  /// cpp `Fixture::requireTypeAtPosition`：找不到类型即判定夹具失败。
  pub fn require_type_at_position_position(&mut self, position: Position) -> TypeId {
    let ty = self.find_type_at_position_module_name_position("".as_ref(), position);
    ulua_common::LUAU_ASSERT!(ty.is_some());
    ty.expect("require_type_at_position: no type found at position")
  }
}

impl Fixture {
  /// cpp `Fixture::requireTypeAtPosition(moduleName, position)`：找不到类型即判定夹具失败。
  pub fn require_type_at_position_module_name_position(
    &mut self,
    module_name: &str,
    position: Position,
  ) -> TypeId {
    let ty = self.find_type_at_position_module_name_position(module_name, position);
    ulua_common::LUAU_ASSERT!(ty.is_some());
    ty.expect("require_type_at_position: no type found at position in module")
  }
}
