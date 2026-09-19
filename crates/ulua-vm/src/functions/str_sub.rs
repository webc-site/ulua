use core::ffi::c_int;

use crate::{
  functions::{
    lua_l_checkinteger::lua_l_checkinteger, lua_l_checklstring::lua_l_checklstring,
    lua_l_optinteger::lua_l_optinteger, lua_pushlstring::lua_pushlstring, posrelat::posrelat,
  },
  macros::lua_pushliteral::lua_pushliteral,
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_str_sub"))]
pub(crate) unsafe extern "C-unwind" fn str_sub(l: *mut lua_State) -> c_int {
  unsafe {
    let mut len: usize = 0;
    let s = lua_l_checklstring(l, 1, &mut len);
    let mut start = posrelat(lua_l_checkinteger(l, 2), len);
    let mut end = posrelat(lua_l_optinteger(l, 3, -1), len);

    if start < 1 {
      start = 1;
    }
    if end > len as c_int {
      end = len as c_int;
    }

    if start <= end {
      lua_pushlstring(l, s.add((start - 1) as usize), (end - start + 1) as usize);
    } else {
      lua_pushliteral(l, c"".as_ptr());
    }
    1
  }
}
