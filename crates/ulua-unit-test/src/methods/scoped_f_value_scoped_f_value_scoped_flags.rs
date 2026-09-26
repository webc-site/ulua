use core::marker::PhantomData;

use ulua_common::records::f_value::{FValue, FValueOverridable};

use crate::records::scoped_f_value::ScopedFValue;

impl<T: FValueOverridable> ScopedFValue<T> {
  /// Idiomatic alias for the C++ `ScopedFValue(FValue<T>&, T)` ctor. Takes a
  /// shared `&FValue<T>` (the flag's interior mutability handles the write), so
  /// `ScopedFastFlag::new(&FFlag::SomeFlag, true)` binds against the `'static`
  /// flag instances — the spelling the test bodies use.
  pub fn new(fvalue: &FValue<T>, new_value: T) -> Self {
    let old_value = fvalue.get();
    fvalue.push_test_override(new_value);

    ScopedFValue {
      value: fvalue as *const FValue<T> as *mut FValue<T>,
      old_value,
      _marker: PhantomData,
    }
  }
}

impl<T: FValueOverridable> Drop for ScopedFValue<T> {
  fn drop(&mut self) {
    // Remove the thread-local override this guard installed (a moved-from
    // guard has a null `value` and pops nothing — exactly one pop per push).
    if !self.value.is_null() {
      // Safety: self.value 为 ScopedFValue 构造时取得的 FValue<T> 线程本地/静态地址（程序生命周期内固定存活）；行 28 滤掉被移出守卫的 null 标记，pop 与构造期 push 配对，cpp ScopedFValue 析构同款。
      unsafe {
        (*self.value).pop_test_override();
      }
    }
  }
}
