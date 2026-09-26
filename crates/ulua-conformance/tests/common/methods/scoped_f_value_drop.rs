use ulua_common::records::f_value::FValueOverridable;

use crate::common::records::scoped_f_value::ScopedFValue;

/// cpp `tests/ScopedFlags.h:41-45` 的 `~ScopedFValue()`：`if (value) value->value =
/// oldValue;`。上游要判空是因为移动构造会把源对象的 `value` 置 `nullptr`；Rust 的移动
/// 语义下 `Drop` 对每个值恰好执行一次，被接管过的 flag 必然存在，无需判空。
impl<T: FValueOverridable + 'static> Drop for ScopedFValue<T> {
  fn drop(&mut self) {
    self.value.pop_test_override();
  }
}
