//! Source: `VM/src/lapi.cpp:272-280` (hand-ported)

use core::ptr::{copy, eq};

use crate::{
  functions::{index_2_addr::index_2_addr, lapi_barrier::lua_c_threadbarrier_lapi},
  macros::{api_check::api_check, lua_o_nilobject::LUA_O_NILOBJECT},
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// `lua_insert` 核心（cpp `VM/src/lapi.cpp:272`）。调用序契约（正确性，非内存安
/// 全）：`idx` 为指向栈内既有槽位的合法索引（debug 断言非哨兵；硬化的
/// `index_2_addr` 使越界索引返回哨兵——`offset_from(哨兵)` 与 cpp 对悬垂指针取
/// 距离同为契约违例，须避免）；旋转写 `p..top` 栈段并经 threadbarrier 同步。
pub(crate) fn lua_insert(l: &mut LuaState, idx: i32) {
  // SAFETY: `l` 存活（引用形保证）；index_2_addr 已硬化；copy/offset 的指针
  // 前提（p 与 top 同属一块栈区）由调用序契约与 VM 栈不变式成立。
  unsafe {
    let lp = l.as_mut_ptr();
    lua_c_threadbarrier_lapi(lp);
    let p: StkId = index_2_addr(lp, idx);
    api_check!(lp, !eq(p, LUA_O_NILOBJECT));

    let count = (*lp).top.offset_from(p);
    if count > 1 {
      let val = *(*lp).top.sub(1);
      copy(p, p.add(1), (count - 1) as usize);
      *p = val;
    }
  }
}
