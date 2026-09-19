use core::ffi::c_void;

use crate::{
  functions::lua_d_call::lua_d_call, macros::cast_to::cast_to, records::call_s::CallS,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe extern "C-unwind" fn f_call(l: *mut lua_State, ud: *mut c_void) {
  unsafe {
    let c = cast_to!(*mut CallS, ud);
    lua_d_call(l, (*c).func, (*c).nresults);
  }
}
