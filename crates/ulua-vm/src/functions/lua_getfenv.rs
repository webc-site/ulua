use core::ptr::eq;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    ensure_stack::ensure_stack, index_2_addr::index_2_addr, lapi_barrier::lua_c_threadbarrier_lapi,
  },
  macros::{
    api_check::api_check, api_incr_top::api_incr_top, lua_o_nilobject::LUA_O_NILOBJECT,
    sethvalue::sethvalue, setnilvalue::setnilvalue, ttype::ttype,
  },
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// `lua_getfenv` 核心（cpp `lapi.cpp:996`）。调用序契约（正确性，非内存安全）：
/// `l` 处于可 GC/可分配（thread barrier）的受保护帧，`ensure_stack(l, 1)` 留 1 结果槽；
/// `idx` 经 `index_2_addr` 解析为非 `LUA_O_NILOBJECT` 的栈槽（`api_check`），命中 Function/Thread
/// 分支时该槽 `value.gc` 须为存活 Closure/LuaState（读其 `env`/`gt` 压栈，`sethvalue` 写屏障可触发 GC）。
pub fn lua_getfenv(l: &mut LuaState, idx: i32) {
  unsafe {
    lua_c_threadbarrier_lapi(l);
    ensure_stack(l, 1);

    let o: StkId = index_2_addr(l, idx);
    api_check!(l, !eq(o, LUA_O_NILOBJECT));

    match ttype!(o) {
      x if x == LuaType::Function as u32 => {
        sethvalue!(l, l.top, (*o).as_closure().env);
      }
      x if x == LuaType::Thread as u32 => {
        sethvalue!(l, l.top, (*o).as_thread().gt);
      }
      _ => {
        setnilvalue!(l.top);
      }
    }

    api_incr_top!(l);
  }
}
