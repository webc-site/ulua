use alloc::{string::String, vec::Vec};

use crate::{
  enums::reduction::Reduction,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
#[derive(Debug, Clone)]
pub struct TypeFunctionReductionResult {
  pub result: Option<TypeId>,
  pub reduction_status: Reduction,
  pub blocked_types: Vec<TypeId>,
  pub blocked_packs: Vec<TypePackId>,
  pub error: Option<String>,
  pub messages: Vec<String>,
}
