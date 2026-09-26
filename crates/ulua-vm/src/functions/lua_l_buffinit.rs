use core::{ffi::c_char, ptr::null_mut};

use crate::records::{
  lua_l_strbuf::{LUA_BUFFERSIZE, LuaLStrbuf},
  lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_l_buffinit(l: *mut LuaState, b: *mut LuaLStrbuf) {
  unsafe {
    let b = &mut *b;
    // start with an internal buffer
    b.p = b.buffer.as_mut_ptr() as *mut c_char;
    b.end = b.p.wrapping_add(LUA_BUFFERSIZE);

    b.l = l;
    b.storage = null_mut();
  }
}
