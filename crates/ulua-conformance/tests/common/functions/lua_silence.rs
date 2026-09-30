use core::ffi::c_int;

use ulua_vm::records::lua_state::LuaState;

/// 静音 print 的 C 回调，签名与 LuaCFunction 对齐，避免 transmute。
pub extern "C-unwind" fn lua_silence(_l: *mut LuaState) -> c_int {
  0
}
