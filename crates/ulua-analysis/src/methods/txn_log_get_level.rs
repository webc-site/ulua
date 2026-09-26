use crate::{
  enums::table_state::TableState,
  functions::get_mutable_type,
  records::{
    free_type::FreeType, function_type::FunctionType, table_type::TableType, txn_log::TxnLog,
    type_level::TypeLevel,
  },
  type_aliases::type_id::TypeId,
};

impl TxnLog {
  pub fn get_level(&self, ty: TypeId) -> Option<TypeLevel> {
    // Check FreeType
    if let Some(ftv) = get_mutable_type::get_mutable::<FreeType>(ty).as_ref() {
      return Some(ftv.level);
    }

    // Check TableType with Free or Generic state
    if let Some(ttv) = get_mutable_type::get_mutable::<TableType>(ty).as_ref()
      && matches!(ttv.state, TableState::Free | TableState::Generic)
    {
      return Some(ttv.level);
    }

    // Check FunctionType
    if let Some(ftv) = get_mutable_type::get_mutable::<FunctionType>(ty).as_ref() {
      return Some(ftv.level);
    }

    None
  }
}
