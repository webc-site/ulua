use core::ffi::c_void;

use crate::{
  functions::shrinkstack::shrinkstack, records::call_context_lgc::CallContext,
  type_aliases::lua_state::lua_State,
};

impl CallContext {
  /// # Safety
  /// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
  pub unsafe extern "C-unwind" fn run(l: *mut lua_State, _ud: *mut c_void) {
    unsafe {
      shrinkstack(l);
    }
  }
}
