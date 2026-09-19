use core::ffi::c_int;

use crate::{
  functions::{lua_l_checklstring::lua_l_checklstring, lua_pushinteger::lua_pushinteger},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_str_len"))]
pub(crate) unsafe extern "C-unwind" fn str_len(l: *mut lua_State) -> c_int {
  unsafe {
    let mut len: usize = 0;
    lua_l_checklstring(l, 1, &mut len);
    lua_pushinteger(l, len as c_int);
    1
  }
}
