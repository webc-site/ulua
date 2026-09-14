use core::{ffi::c_int, mem::zeroed};

use crate::{
  functions::index_2_addr::index2addr,
  macros::{luai_num_2_unsigned::luai_num2unsigned, nvalue::nvalue, tonumber::tonumber},
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

pub(crate) unsafe fn lua_tounsignedx(l: *mut lua_State, idx: c_int, isnum: *mut c_int) -> u32 {
  unsafe {
    let mut n: TValue = zeroed();
    // The tonumber! macro may reassign the pointer if it needs to point to the converted temporary.
    let mut o = index2addr(l, idx);

    if tonumber!(o, &mut n) {
      let mut res: u32 = 0;
      let num = nvalue!(o);
      luai_num2unsigned(&mut res, num);

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
