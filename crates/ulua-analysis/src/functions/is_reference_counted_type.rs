use crate::{
  enums::table_state::TableState,
  functions::get_type,
  records::{
    blocked_type::BlockedType, free_type::FreeType, pending_expansion_type::PendingExpansionType,
    table_type::TableType,
  },
  type_aliases::type_id::TypeId,
};

pub fn is_reference_counted_type(typ: TypeId) -> bool {
  if let Some(tt) = get_type::get::<TableType>(typ).as_ref() {
    matches!(tt.state, TableState::Free | TableState::Unsealed)
  } else {
    get_type::get::<FreeType>(typ).is_some()
      || get_type::get::<BlockedType>(typ).is_some()
      || get_type::get::<PendingExpansionType>(typ).is_some()
  }
}
