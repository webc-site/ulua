use core::ffi::c_int;

use crate::{
  functions::{
    check_div_args_64::check_div_args_64, lua_l_checkinteger_64::lua_l_checkinteger_64,
    lua_pushinteger_64::lua_pushinteger_64,
  },
  type_aliases::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_int64_idiv"))]
pub(crate) unsafe extern "C-unwind" fn int64_idiv(l: *mut LuaState) -> c_int {
  unsafe {
    let a = lua_l_checkinteger_64(l, 1);
    let b = lua_l_checkinteger_64(l, 2);

    check_div_args_64(l, a, b);

    let result = a / b;
    if result < 0 && a % b != 0 {
      lua_pushinteger_64(l, result - 1);
    } else {
      lua_pushinteger_64(l, result);
    }

    1
  }
}
