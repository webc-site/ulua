//! `TypeFunctionSerializer` / `TypeFunctionDeserializer` 的同形检查对共享实现。
//!
//! 两个 serde 宿主的 `state: *mut TypeFunctionRuntimeBuilderState` 与
//! `steps: i32`、`queue: Vec<_>` 字段同型同义（cpp `TypeFunctionRuntimeBuilder`
//! 的序列化/反序列化两侧共用同一 state 与迭代限检查），原先四个方法两两逐字
//! 相同，收口于此，各自 `impl` 只留委托。

use ulua_common::{dfint, fflag};

use crate::records::{
  arena_handle::Handle, type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState,
};

/// cpp serde 宿主的 `hasErrors()`:state 未装配时恒 false;结构化错误旗标开则读
/// `errors`,否则读 `errors_deprecated`。
///
/// 原实现收 `*mut` 并手写判空解引用;`Option<Handle<_>>` 把同一判空语义交给类型,
/// `is_none` 即原 `is_null`,其余全程 safe(review.md §2)。
pub(crate) fn builder_state_has_errors(
  state: Option<Handle<TypeFunctionRuntimeBuilderState>>,
) -> bool {
  let Some(state) = state else {
    return false;
  };
  let state = state.get();

  if fflag::LuauTypeFunctionStructuredErrors.get() {
    !state.errors.is_empty()
  } else {
    !state.errors_deprecated.is_empty()
  }
}

/// cpp serde 宿主的 `hasExceededIterationLimit()`: `steps + queue 长度` 达到
/// `LuauTypeFunctionSerdeIterationLimit`(0 表示不限)即超限。
pub(crate) fn exceeded_serde_iteration_limit(steps: i32, queue_len: usize) -> bool {
  let limit = dfint::LuauTypeFunctionSerdeIterationLimit.get();
  limit != 0 && steps + queue_len as i32 >= limit
}

/// serde 会话读点的「state 已装配」契约文案单点表:装配只发生在
/// `type_function_serializer` / `type_function_deserializer` 入口,其后整轮会话内恒已接线
/// (原实现在同一前提下直接裸解引用,即 UB 与 panic 之别)。
pub(crate) const STATE_WIRED: &str = "serde 会话 state 由 builder 入口装配，本轮会话内恒已接线";
pub(crate) const RUNTIME_WIRED: &str =
  "serde 会话 runtime 由 builder 入口自 ctx 装配，本轮会话内恒已接线";
