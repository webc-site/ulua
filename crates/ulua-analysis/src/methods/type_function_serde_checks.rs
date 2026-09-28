//! `TypeFunctionSerializer` / `TypeFunctionDeserializer` 的同形检查对共享实现。
//!
//! 两个 serde 宿主的 `state: *mut TypeFunctionRuntimeBuilderState` 与
//! `steps: i32`、`queue: Vec<_>` 字段同型同义（cpp `TypeFunctionRuntimeBuilder`
//! 的序列化/反序列化两侧共用同一 state 与迭代限检查），原先四个方法两两逐字
//! 相同，收口于此，各自 `impl` 只留委托。

use ulua_common::{dfint, fflag};

use crate::records::type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState;

/// cpp serde 宿主的 `hasErrors()`：state 未接线时恒 false；结构化错误旗标开则读
/// `errors`，否则读 `errors_deprecated`。
///
/// # Safety 契约说明
/// `state` 由调用宿主在构造期接线、判空后解引用，指向本次 serde 会话存活的
/// builder state。
pub(crate) fn builder_state_has_errors(state: *mut TypeFunctionRuntimeBuilderState) -> bool {
  if state.is_null() {
    return false;
  }

  // Safety: 见函数级契约——非空且存活，仅读取两个错误容器。
  unsafe {
    if fflag::LuauTypeFunctionStructuredErrors.get() {
      !(*state).errors.is_empty()
    } else {
      !(*state).errors_deprecated.is_empty()
    }
  }
}

/// cpp serde 宿主的 `hasExceededIterationLimit()`：`steps + queue 长度` 达到
/// `LuauTypeFunctionSerdeIterationLimit`（0 表示不限）即超限。
pub(crate) fn exceeded_serde_iteration_limit(steps: i32, queue_len: usize) -> bool {
  let limit = dfint::LuauTypeFunctionSerdeIterationLimit.get();
  limit != 0 && steps + queue_len as i32 >= limit
}
