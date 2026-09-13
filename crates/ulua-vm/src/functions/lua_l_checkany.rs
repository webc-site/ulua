//! Node: `cxx:Function:Luau.VM:VM/src/laux.cpp:159:luaL_checkany`
//! Source: `VM/src/laux.cpp:159-163` (hand-ported)

use core::ffi::c_int;

use crate::{
  enums::lua_type::LuaType,
  functions::{lua_l_error_l::lua_l_error_l, lua_type::lua_type},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_l_checkany(l: *mut lua_State, narg: c_int) {
  unsafe {
    if lua_type(l, narg) == LuaType::None as i32 {
      lua_l_error_l(
        l,
        c"missing argument #%d".as_ptr(),
        format_args!("missing argument #{}", narg),
      );
    }
  }
}
