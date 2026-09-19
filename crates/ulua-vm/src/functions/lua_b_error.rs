//! Source: `VM/src/lbaselib.cpp:48-59` (hand-ported)

use crate::{
  functions::{
    lua_concat::lua_concat, lua_error::lua_error, lua_isstring::lua_isstring,
    lua_l_optinteger::lua_l_optinteger, lua_l_where::lua_l_where, lua_pushvalue::lua_pushvalue,
    lua_settop::lua_settop,
  },
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe extern "C-unwind" fn lua_b_error(l: *mut lua_State) -> i32 {
  unsafe {
    let level = lua_l_optinteger(l, 2, 1);
    lua_settop(l, 1);
    if lua_isstring(l, 1) != 0 && level > 0 {
      lua_l_where(l, level);
      lua_pushvalue(l, 1);
      lua_concat(l, 2);
    }
    lua_error(l);
  }
}
