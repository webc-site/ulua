use core::ptr::eq;

use crate::{
  functions::index_2_addr::index_2_addr,
  macros::{equalobj::equalobj, lua_o_nilobject::LUA_O_NILOBJECT},
  records::lua_state::LuaState,
  type_aliases::{stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_equal(l: *mut LuaState, index1: i32, index2: i32) -> i32 {
  // Safety:index_2_addr 依赖 C API 契约 —— l 有效且索引合法。
  let o1: StkId = unsafe { index_2_addr(l, index1) };
  let o2: StkId = unsafe { index_2_addr(l, index2) };

  let nil_ptr = LUA_O_NILOBJECT;

  let i = if eq(o1, nil_ptr) || eq(o2, nil_ptr) {
    0
  } else {
    // Safety:两个指针均指向栈上有效 TValue；equalobj 可能触发元方法（可 longjmp）。
    if unsafe { equalobj!(l, o1 as *const TValue, o2 as *const TValue) } {
      1
    } else {
      0
    }
  };

  i as i32
}
