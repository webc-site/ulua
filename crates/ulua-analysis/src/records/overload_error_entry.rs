use alloc::vec::Vec;

use crate::{
  records::{function_type::FunctionType, txn_log::TxnLog},
  type_aliases::{error_vec::ErrorVec, type_id::TypeId},
};
#[derive(Debug, Clone)]
pub struct OverloadErrorEntry {
  pub(crate) log: TxnLog,
  pub(crate) errors: ErrorVec,
  pub arguments: Vec<TypeId>,
  pub(crate) fn_ty: *const FunctionType,
}
