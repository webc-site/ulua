use core::ffi::c_int;

use crate::{
  enums::lua_status::LuaStatus,
  functions::{lua_d_throw_ldo::luaD_throw, lua_g_pusherror::lua_g_pusherror},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_break(l: *mut lua_State) -> c_int {
  unsafe {
    if (*l).n_ccalls > (*l).base_ccalls {
      lua_g_pusherror(
        l,
        c"attempt to break across metamethod/C-call boundary".as_ptr(),
      );
      luaD_throw(l, LuaStatus::ErrRun as c_int);
    }

    (*l).status = LuaStatus::Break as u8;
    -1
  }
}
