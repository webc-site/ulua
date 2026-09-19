use core::ffi::c_int;

use crate::{
  functions::index_2_addr::index2addr,
  macros::{luai_num_2_unsigned::luai_num2unsigned, nvalue::nvalue, tonumber::tonumber},
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn lua_tounsignedx(l: *mut lua_State, idx: c_int, isnum: *mut c_int) -> u32 {
  unsafe {
    let mut n = TValue::default();
    // The tonumber! macro may reassign the pointer if it needs to point to the converted temporary.
    let mut o = index2addr(l, idx);

    if tonumber!(o, &mut n) {
      let res = luai_num2unsigned(nvalue!(o));

      if !isnum.is_null() {
        *isnum = 1;
      }
      res
    } else {
      if !isnum.is_null() {
        *isnum = 0;
      }
      0
    }
  }
}
