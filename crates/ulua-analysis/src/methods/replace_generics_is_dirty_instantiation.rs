use crate::{
  enums::table_state::TableState,
  records::{generic_type::GenericType, replace_generics::ReplaceGenerics, table_type::TableType},
  type_aliases::type_id::TypeId,
};

impl ReplaceGenerics {
  pub fn is_dirty_type_id(&self, ty: TypeId) -> bool {
    let log = self.base.base.log;

    let ttv = unsafe { (*log).txn_log_get_mutable::<TableType, TypeId>(ty) };
    if !ttv.is_null() {
      return unsafe { (*ttv).state } == TableState::Generic;
    }

    let gtv = unsafe { (*log).txn_log_get_mutable::<GenericType, TypeId>(ty) };
    !gtv.is_null() && self.generics.contains(&ty)
  }
}
