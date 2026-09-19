use core::mem::zeroed;

use crate::{
  functions::{index_2_addr::index2addr, lua_v_tonumber::lua_v_tonumber},
  macros::ttisnumber::ttisnumber,
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_isnumber(l: *mut lua_State, idx: i32) -> i32 {
  // SAFETY：index2addr 依赖 C API 契约 —— l 有效且 idx 为合法（伪）索引。
  let o = unsafe { index2addr(l, idx) };
  // SAFETY：o 指向栈上有效 TValue；tonumber_ 失败时写回零值 TValue。
  unsafe {
    if ttisnumber!(o) {
      1
    } else {
      let mut n: TValue = zeroed();
      (!lua_v_tonumber(o, &mut n).is_null()) as i32
    }
  }
}
