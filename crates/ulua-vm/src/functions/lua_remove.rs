//! Source: `VM/src/lapi.cpp:262-270` (hand-ported)

use core::ptr::eq;

use crate::{
  functions::{c_slice_mut, index_2_addr::index_2_addr},
  macros::{api_check::api_check, lua_o_nilobject::LUA_O_NILOBJECT},
  records::lua_state::LuaState,
};

/// `lua_remove` 核心（cpp `VM/src/lapi.cpp:262`）。调用序契约（正确性，非内存安
/// 全）：`idx` 为指向栈内既有槽位的合法索引（越界索引经硬化的 `index_2_addr`
/// 返回哨兵——对哨兵取槽距与 cpp 对悬垂指针取距离同为契约违例，须避
/// 免）；左移 `idx+1..top` 段并下移栈顶一格。
pub(crate) fn lua_remove(l: &mut LuaState, idx: i32) {
  // SAFETY: `l` 存活（引用形保证）；index_2_addr 已硬化；`p` 与 `top` 同属一块栈区
  // （VM 栈不变式），故 `[p, top)` 可独占切片；shift-down 写序由调用序契约保证界内。
  unsafe {
    let lp = l.as_mut_ptr();
    let p = index_2_addr(&*lp, idx);
    api_check!(lp, !eq(p, LUA_O_NILOBJECT));
    // cpp `while (++p < L->top) setobj2s(L, p-1, p)`：区间 `[p, top)` 两端点左移一格，
    // 收为一次切片 `copy_within`（dst<src 的 shift-down 前向写序与 cpp 循环逐位一致）。
    let len = LuaState::slot_distance(p, (*lp).top);
    if len > 1 {
      c_slice_mut(p, len as usize).copy_within(1.., 0);
    }
    // 弹栈一格：`rewind_top(1)` 提交原语镜像原 `top = top.sub(1)` 落值
    (*lp).rewind_top(1);
  }
}
