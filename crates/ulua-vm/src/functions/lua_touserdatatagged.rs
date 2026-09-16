use core::{
  ffi::{c_int, c_void},
  ptr::null_mut,
};

use crate::{
  functions::index_2_addr::index2addr,
  macros::{ttisuserdata::ttisuserdata, uvalue::uvalue},
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_touserdatatagged(l: *mut lua_State, idx: c_int, tag: c_int) -> *mut c_void {
  unsafe {
    let o: StkId = index2addr(l, idx);

    // uvalue(o)->tag is a u8 in the Udata struct, while tag is a c_int (i32).
    // We cast the struct field to i32 to perform the comparison.
    if ttisuserdata!(o) && (uvalue!(o).tag as i32) == tag {
      uvalue!(o).data.as_ptr() as *mut c_void
    } else {
      null_mut()
    }
  }
}
