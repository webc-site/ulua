use core::cmp::Ordering::{Equal, Less};

use crate::{
  functions::{lua_l_checknumber::lua_l_checknumber, lua_pushnumber::lua_pushnumber},
  macros::lua_l_argcheck::luaL_argcheck,
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe extern "C-unwind" fn math_clamp(l: *mut LuaState) -> i32 {
  unsafe {
    let v = lua_l_checknumber(l, 1);
    let min = lua_l_checknumber(l, 2);
    let max = lua_l_checknumber(l, 3);

    luaL_argcheck!(
      l,
      matches!(min.partial_cmp(&max), Some(Less | Equal)),
      3,
      "max must be greater than or equal to min"
    );

    let r = if v < min { min } else { v };
    let r = if r > max { max } else { r };

    lua_pushnumber(l, r);
    1
  }
}
