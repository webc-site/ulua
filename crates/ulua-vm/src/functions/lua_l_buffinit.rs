use core::{ffi::c_char, ptr::null_mut};

use crate::{
  records::lua_l_strbuf::{LUA_BUFFERSIZE, LuaLStrbuf},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_lua_l_buffinit"))]
pub(crate) unsafe fn lua_l_buffinit(l: *mut lua_State, b: *mut LuaLStrbuf) {
  unsafe {
    // start with an internal buffer
    (*b).p = (*b).buffer.as_mut_ptr() as *mut c_char;
    (*b).end = (*b).p.wrapping_add(LUA_BUFFERSIZE);

    (*b).l = l;
    (*b).storage = null_mut();
  }
}
