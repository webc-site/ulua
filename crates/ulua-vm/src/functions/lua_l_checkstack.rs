use core::ffi::c_int;

use crate::{
  functions::{lua_checkstack::lua_checkstack, lua_l_error_l::lua_l_error_l},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_l_checkstack(l: *mut lua_State, space: c_int, mes: &str) {
  unsafe {
    if lua_checkstack(l, space) == 0 {
      lua_l_error_l(
        l,
        c"stack overflow (%s)".as_ptr(),
        format_args!("stack overflow ({})", mes),
      );
    }
  }
}
