use core::mem::zeroed;

use ulua_vm::{
  functions::lua_getinfo::lua_getinfo,
  records::{lua_debug::LuaDebug, lua_state::LuaState},
};

use crate::common::functions::{cstr::cstr, cstr_text::cstr_raw};
/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
pub unsafe fn get_first_luau_frame_debug_info(l: *mut LuaState) -> Option<LuaDebug> {
  // 自 0 层起向上扫栈，直到找到首个 Lua 帧或栈尽（`getinfo` 返回 0 即 None）；
  // `level` 是栈层级，本帧局部递增。
  let mut level = 0;
  loop {
    // Safety: `ar` 由 `zeroed()` 置为全零 LuaDebug——该记录所有字段都可全零表示
    // （与 cpp `lua_Debug ar = {}` 同形），随后 lua_getinfo 按掩码填充所需字段。
    let mut ar: LuaDebug = unsafe { zeroed() };
    // Safety: `l` 为本用例存活的 LuaState；`b"sl\0"` 是 NUL 结尾的 what 掩码，
    // `&mut ar` 指向上一步已初始化的记录；返回 0 表示该层不存在。
    if unsafe { lua_getinfo(l, level, cstr(b"sl\0"), &mut ar) } == 0 {
      return None;
    }

    // Safety: 掩码含 'S' 时 lua_getinfo 把 `ar.what` 写成 NUL 结尾串（未写则为 null，
    // 由前面的空指针判断兜住）。
    if !ar.what.is_null() && unsafe { cstr_raw(ar.what) } == b"Lua" {
      return Some(ar);
    }

    level += 1;
  }
}
