use core::{ffi::c_char, ptr::null};

use crate::{
  functions::index_2_addr::index_2_addr,
  macros::{getstr::getstr, lua_s_updateatom::lua_s_updateatom},
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
///
/// `l` 必须是正在执行的 C 函数帧的存活 `LuaState`，`idx` 为其合法栈索引；`atom` 必须
/// 为可写 `i32` 槽或 null。非串值直接返回 null 且不写 `atom`；串值经
/// `lua_s_updateatom` 惰性登记 interning 槽号（可进 GC 重查串表），返回值指向该栈槽串
/// 内部字节，下一次操作 `l` 前有效。cpp lapi.cpp:517。
pub unsafe fn lua_tostringatom(l: *mut LuaState, idx: i32, atom: *mut i32) -> *const c_char {
  unsafe {
    let o: StkId = index_2_addr(l, idx);

    if !(*o).is_string() {
      return null();
    }

    let s = (*o).as_string_ptr();
    if !atom.is_null() {
      lua_s_updateatom!(l, s);
      *atom = (*s).atom as i32;
    }

    getstr(s)
  }
}
