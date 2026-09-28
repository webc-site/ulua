use ulua_vm::records::lua_state::LuaState;

use crate::functions::lua_requireinternal::lua_requireinternal;

/// # Safety
///
/// Caller must ensure `l` is a valid pointer to a `LuaState`.
pub(crate) unsafe extern "C-unwind" fn lua_proxyrequire(l: *mut LuaState) -> i32 {
  // Safety: 真 FFI 入口：l 是 VM 调 proxyrequire 闭包时传入的存活 LuaState；check_bytes 保证栈槽 2 为字符串并返回切片（否则抛错发散）。
  unsafe {
    let requirer_chunkname = (*l).check_bytes(2);
    lua_requireinternal(l, requirer_chunkname)
  }
}
