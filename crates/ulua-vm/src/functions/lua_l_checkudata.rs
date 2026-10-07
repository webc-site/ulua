use core::ffi::c_void;

use crate::{
  functions::{lua_l_typeerror_l::lua_l_typeerror_l, lua_touserdata::lua_touserdata},
  macros::lua_registryindex::LUA_REGISTRYINDEX,
  records::lua_state::LuaState,
};

/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载）：
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub fn lua_l_checkudata(l: &mut LuaState, ud: i32, tname: &str) -> *mut c_void {
  unsafe {
    if let Some(p) = lua_touserdata(l, ud)
      && l.get_metatable(ud)
    {
      l.get_field_str(LUA_REGISTRYINDEX, tname);

      if l.raw_equal(-1, -2) {
        l.pop(2);
        return p as *mut c_void;
      }

      l.pop(2); // remove both metatables if they didn't match
    }

    // lua_l_typeerror_l is l_noret (returns !), so this call never returns.
    lua_l_typeerror_l(l, ud, tname);
  }
}
