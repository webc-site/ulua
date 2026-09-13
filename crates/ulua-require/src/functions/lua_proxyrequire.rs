use ulua_vm::{macros::lua_l_checkstring::luaL_checkstring, records::lua_state::lua_State};

use crate::functions::lua_requireinternal::lua_requireinternal;

/// # Safety
///
/// Caller must ensure `l` is a valid pointer to a `lua_State`.
pub unsafe extern "C-unwind" fn lua_proxyrequire(l: *mut lua_State) -> i32 {
  unsafe {
    let requirer_chunkname = luaL_checkstring!(l, 2);
    lua_requireinternal(l, requirer_chunkname)
  }
}
