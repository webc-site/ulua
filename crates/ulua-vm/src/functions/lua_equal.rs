use core::ptr::eq;

use crate::{
  functions::index_2_addr::index_2_addr,
  macros::{equalobj::equalobj, lua_o_nilobject::LUA_O_NILOBJECT},
  records::lua_state::LuaState,
};

/// `lua_equal` 核心（cpp `VM/src/lapi.cpp`）。调用序契约（正确性，非内存安全）：
/// `index1`/`index2` 为栈内合法（伪）索引；命中 `__eq` 元方法时可回跑 Lua 代码
/// （改栈、可抛错），须处于受保护帧。任一索引越界（硬化的 `index_2_addr` 返回
/// 哨兵）直接返回 0。
pub fn lua_equal(l: &mut LuaState, index1: i32, index2: i32) -> i32 {
  let o1 = index_2_addr(l, index1);
  let o2 = index_2_addr(l, index2);

  let nil_ptr = LUA_O_NILOBJECT;

  if eq(o1, nil_ptr) || eq(o2, nil_ptr) {
    0
  } else {
    // SAFETY:两个操作数均指向栈上有效 TValue；`equalobj` 的 `__eq` 元方法
    // 路径可回跑 Lua 代码（改栈/可抛错），须处于受保护帧——调用序契约。
    unsafe {
      let lp = l.as_mut_ptr();
      equalobj!(lp, o1, o2) as i32
    }
  }
}
