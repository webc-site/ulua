//! Source: `VM/src/lapi.cpp:1350-1363` (hand-ported)

use crate::{
  functions::{
    ensure_stack::ensure_stack, index_2_addr::index_2_addr, lapi_barrier::lua_c_threadbarrier_lapi,
    lua_h_next::lua_h_next,
  },
  macros::{api_check::api_check, api_checknelems::api_checknelems, api_incr_top::api_incr_top},
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// `lua_next` 核心（cpp `VM/src/lapi.cpp:1553`）。调用序契约（正确性，非内存安全）：
/// 栈顶已压 1 个 key（`api_checknelems 1`，读 `top-1`）；`idx` 为指向 table 的合法
/// （伪）索引（debug 断言 `is_table`；与 cpp 同构，release 下非 table 索引为类型
/// 混用契约违例）；`ensure_stack(1)` 保证有 key 时 `lua_h_next` 可写 `top` 并
/// `api_incr_top`、无 key 时回退 `top-1`；可触发 GC，须处于受保护帧；跨线程经
/// threadbarrier 同步。
pub(crate) fn lua_next(l: &mut LuaState, idx: i32) -> i32 {
  // SAFETY: `l` 存活（引用形保证）；index_2_addr 已对任意索引硬化（越界返回
  // 哨兵）；api_checknelems/api_check/lua_h_next 的指针前提由引用形、调用序契约
  // 与 VM 栈不变式成立。
  unsafe {
    let lp = l.as_mut_ptr();
    api_checknelems!(lp, 1);
    lua_c_threadbarrier_lapi(lp);
    ensure_stack(lp, 1);
    let t: StkId = index_2_addr(&*lp, idx);
    api_check!(lp, (*t).is_table());

    let more = lua_h_next(lp, &*(*t).as_table_ptr(), (*lp).top.sub(1));
    if more != 0 {
      api_incr_top!(lp);
    } else {
      (*lp).top = (*lp).top.sub(1);
    }
    more
  }
}
