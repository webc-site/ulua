//! Source: `Analysis/src/SubtypingUnifier.cpp:24-31` — `SubtypingUnifier::can_be_unified`.

use crate::{
  enums::table_state::TableState,
  functions::{
    follow_type::follow_type_id, get_type_alt_j::get_type_id, is_blocked_type_utils::is_blocked,
  },
  records::{free_type::FreeType, subtyping_unifier::SubtypingUnifier, table_type::TableType},
  type_aliases::type_id::TypeId,
};

impl SubtypingUnifier {
  pub fn can_be_unified(&self, ty: TypeId) -> bool {
    let ty = follow_type_id(ty);
    if let Some(tbl) = get_type_id::<TableType>(ty) {
      return tbl.state != TableState::Sealed;
    }

    get_type_id::<FreeType>(ty).is_some() || is_blocked(ty)
  }
}
