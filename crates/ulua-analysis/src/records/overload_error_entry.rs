use alloc::vec::Vec;

use crate::{
  records::txn_log::TxnLog,
  type_aliases::{error_vec::ErrorVec, type_id::TypeId},
};
#[derive(Debug, Clone)]
pub struct OverloadErrorEntry {
  pub(crate) log: TxnLog,
  pub(crate) errors: ErrorVec,
  pub arguments: Vec<TypeId>,
  /// C++ `const FunctionType* fnTy`。本端口里 `TypeId` 本身就是 `*const Type`，
  /// 存 TypeId 与存派生指针等价，但不跨越 arena 的可变性假设。
  pub(crate) fn_ty: TypeId,
}
