use crate::{
  records::{pending_type::PendingType, txn_log::TxnLog, r#type::Type},
  type_aliases::type_id::TypeId,
};

impl TxnLog {
  pub fn replace_type_id_t<T>(&mut self, ty: TypeId, replacement: T) -> *mut PendingType
  where
    T: Into<Type>,
  {
    let replacement_type: Type = replacement.into();
    self.replace_type_id_type_item(ty, replacement_type)
  }
}
