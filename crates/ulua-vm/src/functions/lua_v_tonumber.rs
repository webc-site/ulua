use core::ptr::null;

use crate::{
  functions::lua_o_str_2_d::lua_o_str_2_d,
  macros::{setnvalue::setnvalue, svalue::svalue, ttisnumber::ttisnumber, ttisstring::ttisstring},
  type_aliases::t_value::TValue,
};

pub(crate) unsafe fn lua_v_tonumber(obj: *const TValue, n: *mut TValue) -> *const TValue {
  unsafe {
    if ttisnumber!(obj) {
      return obj;
    }

    if ttisstring!(obj) {
      let mut num_out: f64 = 0.0;
      let p = svalue!(obj);
      if lua_o_str_2_d(p, &mut num_out) != 0 {
        setnvalue!(n, num_out);
        return n;
      }
    }

    null()
  }
}
