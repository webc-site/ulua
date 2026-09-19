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

impl TypeFunctionReductionResult {
  /// 本轮不归约：MaybeOk；`blocked_types` 非空时等待这些类型解阻塞
  pub fn no_reduction(blocked_types: Vec<TypeId>) -> Self {
    Self {
      result: None,
      reduction_status: Reduction::MaybeOk,
      blocked_types,
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    }
  }
}
