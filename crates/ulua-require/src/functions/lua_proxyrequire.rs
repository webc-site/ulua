use core::ffi::CStr;

use ulua_vm::{macros::lua_l_checkstring::luaL_checkstring, records::lua_state::lua_State};

use crate::functions::lua_requireinternal::lua_requireinternal;

/// # Safety
///
/// Caller must ensure `l` is a valid pointer to a `lua_State`.
pub unsafe extern "C-unwind" fn lua_proxyrequire(l: *mut lua_State) -> i32 {
  unsafe {
    let requirer_chunkname = luaL_checkstring!(l, 2);
    // CStr::from_ptr 安全性：luaL_checkstring 保证返回指向 NUL 结尾字符串的有效指针
    let requirer_chunkname = CStr::from_ptr(requirer_chunkname);
    lua_requireinternal(l, requirer_chunkname)
  }
}
