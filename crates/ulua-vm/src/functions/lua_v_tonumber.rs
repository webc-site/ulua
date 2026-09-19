use core::ptr::null;

use crate::{
  functions::lua_o_str_2_d::lua_o_str_2_d,
  macros::{setnvalue::setnvalue, svalue::svalue, ttisnumber::ttisnumber, ttisstring::ttisstring},
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn lua_v_tonumber(obj: *const TValue, n: *mut TValue) -> *const TValue {
  unsafe {
    if ttisnumber!(obj) {
      return obj;
    }

    if ttisstring!(obj) {
      let p = svalue!(obj);
      if let Some(num) = lua_o_str_2_d(p) {
        setnvalue!(n, num);
        return n;
      }
    }

    null()
  }
}
