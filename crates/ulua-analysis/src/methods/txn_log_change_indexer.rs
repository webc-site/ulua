use crate::{
  functions::get_mutable_txn_log::get_mutable_pending_type,
  records::{
    pending_type::PendingType, table_indexer::TableIndexer, table_type::TableType, txn_log::TxnLog,
  },
  type_aliases::type_id::TypeId,
};
impl TxnLog {
  pub(crate) fn change_indexer(
    &mut self,
    ty: TypeId,
    indexer: Option<TableIndexer>,
  ) -> *mut PendingType {
    // SAFETY: We assume get<TableType>(ty) is valid per the C++ assertion
    // and that queue_type_id returns a valid PendingType pointer
    let new_ty = unsafe { self.queue_type_id(ty) };

    // SAFETY: get_mutable_pending_type is the Rust analog of get_mutable<TableType>
    // We assume the type is a TableType as per the C++ assertion；未命中（原 null
    // 哨兵）由 `if let None` 分支按 no-op 处理，行为与 cpp 判空短路一致。
    if let Some(table_type) = unsafe { get_mutable_pending_type::<TableType>(new_ty) } {
      table_type.indexer = indexer;
    }

    new_ty
  }
}
