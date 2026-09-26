use ulua_common::functions::c_str::cstr_bytes;
use ulua_vm::{macros::lua_l_checkstring::luaL_checkstring, records::lua_state::LuaState};

use crate::functions::lua_requireinternal::lua_requireinternal;

/// # Safety
///
/// Caller must ensure `l` is a valid pointer to a `LuaState`.
pub(crate) unsafe extern "C-unwind" fn lua_proxyrequire(l: *mut LuaState) -> i32 {
  // Safety: 真 FFI 入口：l 是 VM 调 proxyrequire 闭包时传入的存活 LuaState；luaL_checkstring 保证栈槽 2 为字符串并返回指向该 VM 字符串的 NUL 结尾指针（否则抛错发散），调用帧存续期间串不被回收，故 cstr_bytes 门面读取安全。
  unsafe {
    let requirer_chunkname = luaL_checkstring!(l, 2);
    // 真 FFI 入口：C 串 → 字节经 cstr_bytes 门面单点收口（checkstring 保证非
    // null），一次性转换后内部链只用字节串
    lua_requireinternal(l, cstr_bytes(requirer_chunkname))
  }
}
