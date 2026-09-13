use alloc::boxed::Box;

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::records::scoped_exit::ScopedExit;
impl ScopedExit {
  pub fn scoped_exit_function_void(f: Box<dyn FnOnce()>) -> Self {
    let func = Some(f);
    LUAU_ASSERT!(func.is_some());
    Self { func }
  }
}
