use core::ffi::c_int;

use crate::{
  functions::{lua_insert::lua_insert, lua_pushboolean::lua_pushboolean},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_coresumefinish"))]
pub(crate) unsafe fn coresumefinish(l: *mut lua_State, r: c_int) -> c_int {
  unsafe {
    if r < 0 {
      lua_pushboolean(l, 0);
      lua_insert(l, -2);
      2
    } else {
      lua_pushboolean(l, 1);
      lua_insert(l, -(r + 1));
      r + 1
    }
  }
}
