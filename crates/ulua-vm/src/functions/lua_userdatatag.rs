use core::ffi::c_int;

use crate::{
  functions::index_2_addr::index2addr,
  macros::{ttisuserdata::ttisuserdata, uvalue::uvalue},
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_userdatatag(l: *mut lua_State, idx: c_int) -> c_int {
  unsafe {
    let o: StkId = index2addr(l, idx);

    if ttisuserdata!(o) {
      // uvalue(o) returns a reference to the Udata struct.
      // The tag field is a uint8_t.
      uvalue!(o).tag as c_int
    } else {
      -1
    }
  }
}
