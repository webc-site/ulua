use core::ffi::c_int;

use crate::{
  functions::{
    lua_l_checkinteger_64::lua_l_checkinteger_64, lua_l_error_l::lua_l_error_l,
    lua_l_optinteger_64::lua_l_optinteger_64, lua_pushinteger_64::lua_pushinteger_64,
  },
  macros::lua_l_argcheck::luaL_argcheck,
  type_aliases::lua_state::LuaState,
};

/// cpp lintlib.cpp `mask64(w)`：低 w 位全 1 掩码。
const fn mask64(w: u32) -> u64 {
  u64::MAX >> (64 - w)
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
/// cpp lintlib.cpp `int64_replace`：把 r 的低 w 位写入 n 的第 f 位起字段。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_int64_replace"))]
pub(crate) unsafe extern "C-unwind" fn int64_replace(l: *mut LuaState) -> c_int {
  unsafe {
    let n = lua_l_checkinteger_64(l, 1);
    let r = lua_l_checkinteger_64(l, 2);
    let f = lua_l_checkinteger_64(l, 3);
    let w = lua_l_optinteger_64(l, 4, 1);

    luaL_argcheck!(l, (0..=63).contains(&f), 2, "field cannot be negative");
    luaL_argcheck!(l, 0 < w, 4, "width must be positive");
    if f + w > 64 {
      lua_l_error_l(
        l,
        c"trying to access non-existent bits".as_ptr(),
        core::format_args!("trying to access non-existent bits"),
      );
    }

    let n = n as u64;
    let r = r as u64;
    let f = f as u32;
    let w = w as u32;

    let base_mask = mask64(w);
    let replacement = (r & base_mask) << f;
    let mask = u64::MAX ^ (base_mask << f);

    lua_pushinteger_64(l, ((n & mask) | replacement) as i64);

    1
  }
}
