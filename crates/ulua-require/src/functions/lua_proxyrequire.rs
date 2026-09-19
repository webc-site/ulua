use core::ffi::CStr;

use ulua_vm::{macros::lua_l_checkstring::luaL_checkstring, records::lua_state::lua_State};

use crate::functions::lua_requireinternal::lua_requireinternal;

/// # Safety
///
/// Caller must ensure `l` is a valid pointer to a `lua_State`.
pub unsafe extern "C-unwind" fn lua_proxyrequire(l: *mut lua_State) -> i32 {
  unsafe {
    let requirer_chunkname = luaL_checkstring!(l, 2);
    // 真 FFI 入口：CStr::from_ptr 安全性由 luaL_checkstring 保证（返回指向 NUL 结尾
    // 字符串的有效指针），一次性 `.to_bytes()` 后内部链只用字节串
    lua_requireinternal(l, CStr::from_ptr(requirer_chunkname).to_bytes())
  }
}
