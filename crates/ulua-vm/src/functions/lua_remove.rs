//! Source: `VM/src/lapi.cpp:262-270` (hand-ported)

use core::ptr::{copy, eq};

use crate::{
  functions::index_2_addr::index_2_addr,
  macros::{api_check::api_check, lua_o_nilobject::LUA_O_NILOBJECT},
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// `lua_remove` 核心（cpp `VM/src/lapi.cpp:262`）。调用序契约（正确性，非内存安
/// 全）：`idx` 为指向栈内既有槽位的合法索引（越界索引经硬化的 `index_2_addr`
/// 返回哨兵——对哨兵做 `offset_from` 与 cpp 对悬垂指针取距离同为契约违例，须避
/// 免）；左移 `idx+1..top` 段并下移栈顶一格。
pub(crate) fn lua_remove(l: &mut LuaState, idx: i32) {
  // SAFETY: `l` 存活（引用形保证）；index_2_addr 已硬化；copy/offset 的指针
  // 前提（p 与 top 同属一块栈区）由调用序契约与 VM 栈不变式成立。
  unsafe {
    let lp = l.as_mut_ptr();
    let p: StkId = index_2_addr(&*lp, idx);
    api_check!(lp, !eq(p, LUA_O_NILOBJECT));
    let count = (*lp).top.offset_from(p) - 1;
    if count > 0 {
      copy(p.add(1), p, count as usize);
    }
    (*lp).top = (*lp).top.sub(1);
  }
}
