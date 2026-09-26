use core::ffi::c_char;

use crate::{
  functions::{
    cstr_cow, currentline::currentline, getluaproto::get_lua_proto, lua_o_chunkid::lua_o_chunkid,
    lua_o_pushfstring::lua_o_pushfstring, lua_pushstring::lua_pushstring,
  },
  macros::{getstr::getstr, is_lua::isLua, lua_idsize::LUA_IDSIZE},
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn pusherror(l: *mut LuaState, msg: *const c_char) {
  unsafe {
    let ci = (*l).ci;

    // isLua! 宏接收 CallInfo 指针而非解引用结构体
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

      let line = currentline(l, ci);

      // The `to_string_lossy()` `Cow`s must be bound to locals so they outlive
      // the `format_args!` that borrows them: a `fmt::Arguments` can never
      // outlive its captured temporaries, so storing it in a `let` and using
      // it in a *later* statement dangles (E0716 — the temporaries are dropped
      // at the end of the `let`). Inline `format_args!` into the call instead.
      let chunk = cstr_cow(chunkid);
      let msg_str = cstr_cow(msg);
      // 对应 cpp ldebug.cpp 的 `luaO_pushfstring(L, "%s:%d: %s", chunkid, line, msg)`
      lua_o_pushfstring(l, format_args!("{}:{}: {}", chunk, line, msg_str));
    } else {
      lua_pushstring(l, msg);
    }
  }
}
