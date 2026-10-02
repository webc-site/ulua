use alloc::string::String;
use core::ffi::c_char;

use crate::{
  functions::{
    cstr_bytes, cstr_cow, currentline::currentline, getluaproto::get_lua_proto,
    lua_o_chunkid::lua_o_chunkid, lua_o_pushfstring::lua_o_pushfstring,
  },
  macros::{getstr::getstr, is_lua::isLua, lua_idsize::LUA_IDSIZE},
  records::lua_state::LuaState,
};

/// 安全字节切片版错误信息压栈。
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn pusherror_bytes(l: *mut LuaState, msg: &[u8]) {
  unsafe {
    let ci = (*l).ci;

    if isLua!(ci) {
      let proto = get_lua_proto(ci);
      let source = (*proto).source;

      let mut chunkbuf: [c_char; LUA_IDSIZE as usize] = [0; LUA_IDSIZE as usize];
      let chunkid = lua_o_chunkid(
        chunkbuf.as_mut_ptr(),
        chunkbuf.len(),
        getstr(source),
        (*source).len as usize,
      );

      let line = currentline(&*ci);
      let chunk = cstr_cow(chunkid);
      let msg_str = String::from_utf8_lossy(msg);
      lua_o_pushfstring(&mut *l, format_args!("{}:{}: {}", chunk, line, msg_str));
    } else {
      (*l).push_bytes(msg);
    }
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn pusherror(l: *mut LuaState, msg: *const c_char) {
  unsafe {
    pusherror_bytes(l, cstr_bytes(msg));
  }
}
