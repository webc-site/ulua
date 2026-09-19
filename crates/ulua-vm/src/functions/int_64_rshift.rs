use core::ffi::c_int;

use crate::{
  functions::{
    lua_l_checkinteger_64::lua_l_checkinteger_64, lua_pushinteger_64::lua_pushinteger_64,
  },
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_int64_rshift"))]
pub(crate) unsafe extern "C-unwind" fn int64_rshift(l: *mut lua_State) -> c_int {
  unsafe {
    let n = lua_l_checkinteger_64(l, 1) as u64;
    let i = lua_l_checkinteger_64(l, 2);

    if (-63..=63).contains(&i) {
      lua_pushinteger_64(l, if i < 0 { n << (-i) } else { n >> i } as i64);
    } else {
      lua_pushinteger_64(l, 0);
    }

    1
  }
}
