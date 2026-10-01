//! Source: `VM/src/lapi.cpp:384-393` (hand-ported)

use core::ptr::eq;

use crate::{
  functions::{index_2_addr::index_2_addr, lua_v_lessthan::lua_v_lessthan},
  macros::lua_o_nilobject::LUA_O_NILOBJECT,
  records::lua_state::LuaState,
  type_aliases::stk_id::StkId,
};

/// `lua_lessthan` 核心（cpp `VM/src/lapi.cpp:384`）。调用序契约（正确性，非内存
/// 安全）：`index1`/`index2` 为栈内合法（伪）索引；命中 `__lt` 元方法时可回跑
/// Lua 代码（改栈、可抛错），须处于受保护帧。任一索引越界（硬化的
/// `index_2_addr` 返回哨兵）直接返回 0——与 cpp 正索引越界分支逐位一致。
pub fn lua_lessthan(l: &mut LuaState, index1: i32, index2: i32) -> i32 {
  // SAFETY: `l` 存活（引用形保证）；index_2_addr 已对任意索引硬化（越界返回
  // 哨兵，无栈外指针算术）；哨兵臂提前返回 0，lua_v_lessthan 仅接收非哨兵栈
  // 槽，沿用本帧栈界与元方法回跑契约。
  unsafe {
    let lp = l.as_mut_ptr();
    let o1: StkId = index_2_addr(&*lp, index1);
    let o2: StkId = index_2_addr(&*lp, index2);

    let nil_ptr = LUA_O_NILOBJECT;

    if eq(o1, nil_ptr) || eq(o2, nil_ptr) {
      0
    } else {
      lua_v_lessthan(lp, &*o1, &*o2)
    }
  }
}
