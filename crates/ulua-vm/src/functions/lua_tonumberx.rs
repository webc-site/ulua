use core::{ffi::c_int, mem::zeroed};

use crate::{
  functions::index_2_addr::index_2_addr,
  macros::{nvalue::nvalue, tonumber::tonumber},
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_tonumberx(l: *mut lua_State, idx: c_int, isnum: *mut c_int) -> f64 {
  unsafe {
    let mut n: TValue = zeroed();
    let mut o = index_2_addr(l, idx) as *const TValue;

    if tonumber!(o, &mut n) {
      if !isnum.is_null() {
        *isnum = 1;
      }
      nvalue!(o)
    } else {
      if !isnum.is_null() {
        *isnum = 0;
      }
      0.0
    }
  }
}
