use alloc::string::String;

use crate::type_aliases::{reducer_function::ReducerFunction, type_pack_id::TypePackId};
#[derive(Debug, Clone)]
pub struct TypePackFunction {
  pub name: String,
  /// cpp `TypePackFunction::reducer` = `ReducerFunction<TypePackId>`（subject 为
  /// TypePackId，结果携带 `TypeFunctionReductionResult<TypePackId>`）。
  pub reducer: ReducerFunction<TypePackId>,
  pub can_reduce_generics: bool,
}
