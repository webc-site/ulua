use core::ptr::eq;

use crate::{
  functions::index_2_addr::index_2_addr,
  macros::{
    api_check::api_check, api_checknelems::api_checknelems, gcvalue::gcvalue,
    lua_c_objbarrier::lua_c_objbarrier, lua_o_nilobject::LUA_O_NILOBJECT,
  },
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// # Safety
/// `l` 须为存活 `LuaState`：`api_checknelems!(l,1)` 要求 `(*l).top-1` 已压新 env 且为 table（is_table），
/// `idx` 为合法索引、`index_2_addr` 所得 `o` 非 LUA_O_NILOBJECT；命中 Function/Thread 分支时 `(*o).value.gc` 须为
/// 存活 Closure/LuaState（写其 `env`/`gt` 为 hvalue(top-1)），随后 `lua_c_objbarrier` 置灰防漏；末尾 `top` 回退 1。非 Function/Thread 返回 0。
/// cpp VM/src/lapi.cpp:1106
pub unsafe fn lua_setfenv(l: *mut LuaState, idx: i32) -> i32 {
  unsafe {
    let mut res: i32 = 1;
    api_checknelems!(l, 1);
    let o: StkId = index_2_addr(l, idx);
    api_check!(l, !eq(o, LUA_O_NILOBJECT));
    api_check!(l, (*(*l).top.sub(1)).is_table());
    if let Some(cl) = (*(*o).value.gc).as_closure_mut() {
      cl.env = (*(*l).top.sub(1)).as_table_ptr();
    } else if let Some(th) = (*(*o).value.gc).as_thread_mut() {
      th.gt = (*(*l).top.sub(1)).as_table_ptr();
    } else {
      res = 0;
    }
    if res != 0 {
      lua_c_objbarrier!(l, gcvalue!(o), (*(*l).top.sub(1)).as_table_ptr());
    }
    (*l).top = (*l).top.sub(1);
    res
  }
}
