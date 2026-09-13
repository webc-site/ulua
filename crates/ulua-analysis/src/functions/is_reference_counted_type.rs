use crate::{
  enums::table_state::TableState,
  functions::get_type_alt_j::get_type_id,
  records::{
    blocked_type::BlockedType, free_type::FreeType, pending_expansion_type::PendingExpansionType,
    table_type::TableType,
  },
  type_aliases::type_id::TypeId,
};

pub fn is_reference_counted_type(typ: TypeId) -> bool {
  if let Some(tt) = get_type_id::<TableType>(typ).as_ref() {
    tt.state == TableState::Free || tt.state == TableState::Unsealed
  } else {
    !get_type_id::<FreeType>(typ).is_none()
      || !get_type_id::<BlockedType>(typ).is_none()
      || !get_type_id::<PendingExpansionType>(typ).is_none()
  }
}
