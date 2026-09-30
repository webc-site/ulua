use core::ffi::c_char;

use crate::{functions::cstr_bytes, records::lua_state::LuaState};

/// # Safety
/// `l` 须为存活 `LuaState`；`tname` 须为非空、NUL 结尾的 C 字符串名（`new_metatable_by_bytes` 会读取，
/// 不能为 NULL）；操作 registry 并可建表/分配/抛错，须在受保护帧内调用。cpp `laux.cpp:110`。
pub unsafe fn lua_l_newmetatable(l: *mut LuaState, tname: *const c_char) -> i32 {
  unsafe {
    let name_bytes = cstr_bytes(tname);
    (*l).new_metatable_by_bytes(name_bytes)
  }
}
