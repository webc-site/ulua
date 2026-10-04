//! Source: `VM/src/lapi.cpp:272-280` (hand-ported)

use core::ptr::eq;

use crate::{
  functions::{c_slice_mut, index_2_addr::index_2_addr, lapi_barrier::lua_c_threadbarrier_lapi},
  macros::{api_check::api_check, lua_o_nilobject::LUA_O_NILOBJECT},
  records::lua_state::LuaState,
};

/// `lua_insert` 核心（cpp `VM/src/lapi.cpp:272`）。调用序契约（正确性，非内存安
/// 全）：`idx` 为指向栈内既有槽位的合法索引（debug 断言非哨兵；硬化的
/// `index_2_addr` 使越界索引返回哨兵——对哨兵取槽距与 cpp 对悬垂指针取
/// 距离同为契约违例，须避免）；旋转写 `p..top` 栈段并经 threadbarrier 同步。
pub(crate) fn lua_insert(l: &mut LuaState, idx: i32) {
  // SAFETY: `l` 存活（引用形保证）；index_2_addr 已硬化；`p` 与 `top` 同属一块栈区
  // （VM 栈不变式），故 `[p, top)` 可独占切片；shift-up 写序由调用序契约保证界内。
  unsafe {
    let lp = l.as_mut_ptr();
    lua_c_threadbarrier_lapi(lp);
    let p = index_2_addr(&*lp, idx);
    api_check!(lp, !eq(p, LUA_O_NILOBJECT));

    // cpp `for (q=top; q>p; q--) setobj2s(q, q-1); setobj2s(p, L->top)`：区间 `[p, top)`
    // 两端点右旋一格，收为一次切片 `copy_within`（dst>src 的 shift-up 后向写序与 cpp
    // 循环逐位一致）；`L->top` 溢出读数即原 `top[-1]`，先取后旋、落笔 `p`。
    let len = LuaState::slot_distance(p, (*lp).top);
    if len > 1 {
      let win = c_slice_mut(p, len as usize);
      let val = win[len as usize - 1];
      win.copy_within(0..len as usize - 1, 1);
      win[0] = val;
    }
  }
}
