//! @interface-stub
use core::ptr::null_mut;

use crate::{
  enums::table_state::TableState,
  functions::{
    as_mutable_type::as_mutable_type_id, follow_type::follow_type_id, get_type_alt_j::get_type_id,
  },
  records::{free_type::FreeType, table_type::TableType, type_checker::TypeChecker},
  type_aliases::{type_id::TypeId, type_variant::TypeVariant},
};
impl TypeChecker {
  pub fn tablify(&mut self, ty: TypeId) {
    let ty = follow_type_id(ty);

    if let Some(free) = get_type_id::<FreeType>(ty) {
      // SAFETY: as_mutable_type_id 为句柄可变访问（C++ asMutable(ty)->ty.emplace<TableType>）。
      unsafe {
        (*as_mutable_type_id(ty)).ty =
          TypeVariant::Table(TableType::table_type_table_state_type_level_scope(
            TableState::Free,
            free.level,
            null_mut(),
          ));
      }
    }
  }
}
