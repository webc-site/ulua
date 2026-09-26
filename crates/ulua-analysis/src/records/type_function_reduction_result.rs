use alloc::{string::String, vec::Vec};

use crate::{
  enums::reduction::Reduction,
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
/// C++ `TypeFunctionReductionResult<T>`（TypeFunction.h，模板）；默认参数
/// `TypeId` 保持既有 type-family 调用点零改动，pack family 用 `<_<TypePackId>>`。
#[derive(Debug, Clone)]
pub struct TypeFunctionReductionResult<T = TypeId> {
  pub result: Option<T>,
  pub reduction_status: Reduction,
  pub blocked_types: Vec<TypeId>,
  pub blocked_packs: Vec<TypePackId>,
  pub error: Option<String>,
  pub messages: Vec<String>,
}

impl<T> TypeFunctionReductionResult<T> {
  /// 成功归约：携带结果、MaybeOk、无阻塞无错误（cpp `{result, Reduction::MaybeOk, {}, {}, {}, {}}`）
  pub fn reduction(result: T) -> Self {
    Self {
      result: Some(result),
      reduction_status: Reduction::MaybeOk,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    }
  }

  /// 错误不归约：无结果、Erroneous、无阻塞无错误（cpp `{nullopt, Reduction::Erroneous, {}, {}, {}, {}}`）
  pub fn erroneous() -> Self {
    Self {
      result: None,
      reduction_status: Reduction::Erroneous,
      blocked_types: Vec::new(),
      blocked_packs: Vec::new(),
      error: None,
      messages: Vec::new(),
    }
  }
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
