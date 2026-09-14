use core::ffi::{c_char, c_int};

use crate::{
  macros::{
    api_check::api_check, fixedbit::FIXEDBIT, l_setbit::l_setbit, lua_lutag_limit::LUA_LUTAG_LIMIT,
    lua_s_new::luaS_new,
  },
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_lua_setlightuserdataname")]
pub unsafe fn lua_setlightuserdataname(l: *mut lua_State, tag: c_int, name: *const c_char) {
  unsafe {
    api_check!(l, (tag as u32) < LUA_LUTAG_LIMIT as u32);
    // renaming not supported
    api_check!(l, (*(*l).global).lightuserdataname[tag as usize].is_null());

    if (*(*l).global).lightuserdataname[tag as usize].is_null() {
      let ts = luaS_new(l, name);
      (*(*l).global).lightuserdataname[tag as usize] = ts;
      l_setbit!((*ts).hdr.marked, FIXEDBIT); // never collect these names
    }
  }
}
