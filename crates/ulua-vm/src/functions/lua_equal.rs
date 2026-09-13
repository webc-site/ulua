use core::{ffi::c_int, ptr::eq};

use crate::{
  functions::index_2_addr::index2addr,
  macros::{equalobj::equalobj, lua_o_nilobject::luaO_nilobject},
  type_aliases::{lua_state::lua_State, stk_id::StkId, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[unsafe(export_name = "ulua_lua_equal")]
pub unsafe fn lua_equal(l: *mut lua_State, index1: c_int, index2: c_int) -> c_int {
  // SAFETY：index2addr 依赖 C API 契约 —— l 有效且索引合法。
  let o1: StkId = unsafe { index2addr(l, index1) };
  let o2: StkId = unsafe { index2addr(l, index2) };

  let nil_ptr = luaO_nilobject;

  let i = if eq(o1, nil_ptr) || eq(o2, nil_ptr) {
    0
  } else {
    // SAFETY：两个指针均指向栈上有效 TValue；equalobj 可能触发元方法（可 longjmp）。
    if unsafe { equalobj!(l, o1 as *const TValue, o2 as *const TValue) } {
      1
    } else {
      0
    }
  };

  i as c_int
}
