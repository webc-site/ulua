use alloc::vec::Vec;

use crate::{
  records::type_fun::TypeFun,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

#[derive(Debug, Clone, PartialEq)]
pub struct InstantiationSignature {
  pub(crate) fn_sig: TypeFun,
  pub(crate) arguments: Vec<TypeId>,
  pub(crate) pack_arguments: Vec<TypePackId>,
}
