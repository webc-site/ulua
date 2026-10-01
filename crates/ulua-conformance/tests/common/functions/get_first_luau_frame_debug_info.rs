use ulua_vm::records::{lua_debug::LuaDebug, lua_state::LuaState};

use crate::common::functions::{
  cstr_text::cstr_raw,
  safe_api::{getinfo, zero_debug},
};
pub fn get_first_luau_frame_debug_info(l: *mut LuaState) -> Option<LuaDebug> {
  // 自 0 层起向上扫栈，直到找到首个 Lua 帧或栈尽（`getinfo` 返回 0 即 None）；
  // `level` 是栈层级，本帧局部递增。
  let mut level = 0;
  loop {
    // `zero_debug` 交出全零 LuaDebug（与 cpp `lua_Debug ar = {}` 同形），
    // 随后 lua_getinfo 按掩码填充所需字段；返回 0 表示该层不存在。
    let mut ar = zero_debug();
    if getinfo(l, level, b"sl\0", &mut ar) == 0 {
      return None;
    }

    // 掩码含 'S' 时 lua_getinfo 把 `ar.what` 写成 NUL 结尾串（未写则为 null，
    // 由前面的空指针判断兜住）。
    // Safety: `ar.what` 非空时为 VM 写入的 NUL 结尾串。
    if !ar.what.is_null() && unsafe { cstr_raw(ar.what) } == b"Lua" {
      return Some(ar);
    }

    level += 1;
  }
}
