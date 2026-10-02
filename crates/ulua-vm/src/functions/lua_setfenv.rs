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

/// `lua_setfenv` 核心（cpp VM/src/lapi.cpp:1106）。调用序契约（正确性，非内存安全）：
/// `api_checknelems!(l,1)` 要求 `l.top-1` 已压新 env 且为 table（is_table），
/// `idx` 为合法索引、`index_2_addr` 所得 `o` 非 LUA_O_NILOBJECT；命中 Function/Thread 分支时 `(*o).value.gc` 须为
/// 存活 Closure/LuaState（写其 `env`/`gt` 为 hvalue(top-1)），随后 `lua_c_objbarrier` 置灰防漏；末尾 `top` 回退 1。非 Function/Thread 返回 0。
pub fn lua_setfenv(l: &mut LuaState, idx: i32) -> i32 {
  unsafe {
    let mut res: i32 = 1;
    api_checknelems!(l, 1);
    let o: StkId = index_2_addr(l, idx);
    api_check!(l, !eq(o, LUA_O_NILOBJECT));
    api_check!(l, (*l.top.sub(1)).is_table());
    if let Some(cl) = (*(*o).value.gc).as_closure_mut() {
      cl.env = (*l.top.sub(1)).as_table_ptr();
    } else if let Some(th) = (*(*o).value.gc).as_thread_mut() {
      th.gt = (*l.top.sub(1)).as_table_ptr();
    } else {
      res = 0;
    }
    if res != 0 {
      // r16-v9b 授权拆语句（r16-v3 #60 判例）：第三实参原位现读 top-1 与宏体
      // `&mut *$l` 重借用构成 E0503。等价论证：求值序本即先读 top-1 后入障，句间
      // 无对 `l` 场/栈槽写点；`as_table_ptr` 只读，取指针值不延存借用，语义逐位不变。
      let top_env_table = (*l.top.sub(1)).as_table_ptr();
      lua_c_objbarrier!(l, gcvalue!(o), top_env_table);
    }
    // 消费 env 槽一格：`rewind_top(1)` 提交原语镜像原 `top = top.sub(1)` 落值
    l.rewind_top(1);
    res
  }
}
