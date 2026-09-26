use core::{ffi::c_char, ptr::null};

use crate::{
  macros::{getstr::getstr, lua_s_updateatom::lua_s_updateatom},
  records::lua_state::LuaState,
};

/// # Safety
/// `l` 须为存活 `LuaState`：读 `(*l).namecall`（上次 __namecall 的串，允许 NULL→返回 NULL 不解引用）；
/// `atom` 允许 NULL（仅当非空才 `lua_s_updateatom` 并写回 `*atom`，此时须为可写 i32），非空 namecall 时其
/// `atom` 字段可读。`getstr(s)` 返回的 C 串指针生命周期随该 TString。纯读，不分配、不抛错。
/// cpp VM/src/lapi.cpp:554
pub unsafe fn lua_namecallatom(l: *mut LuaState, atom: *mut i32) -> *const c_char {
  unsafe {
    let s = (*l).namecall;
    if s.is_null() {
      return null();
    }
    if !atom.is_null() {
      lua_s_updateatom!(l, s);
      *atom = (*s).atom as i32;
    }
    getstr(s)
  }
}
