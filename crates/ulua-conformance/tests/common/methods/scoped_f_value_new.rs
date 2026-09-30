use ulua_common::records::f_value::{FValue, FValueOverridable};

use crate::common::records::scoped_f_value::ScopedFValue;

impl<T: FValueOverridable + 'static> ScopedFValue<T> {
  /// cpp `tests/ScopedFlags.h:16-21`：`ScopedFValue(FValue<T>& fvalue, T newValue)`
  /// —— 记录旧值并写入新值。本端口用线程本地覆盖栈实现“记录旧值”。
  pub fn new(fvalue: &'static FValue<T>, new_value: T) -> Self {
    fvalue.push_test_override(new_value);

    ScopedFValue { value: fvalue }
  }
}
