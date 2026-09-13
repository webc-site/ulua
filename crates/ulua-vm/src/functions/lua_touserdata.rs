use core::{
  ffi::{c_int, c_void},
  ptr::null_mut,
};

use crate::{
  functions::index_2_addr::index2addr,
  macros::{ttislightuserdata::ttislightuserdata, ttisuserdata::ttisuserdata, uvalue::uvalue},
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_touserdata(l: *mut lua_State, idx: c_int) -> *mut c_void {
  unsafe {
    let o: StkId = index2addr(l, idx);

    if ttisuserdata!(o) {
      // uvalue(o) returns a pointer to the Udata struct.
      // The data field is a char[1] at the end of the struct.
      // We return the address of that array as a void pointer.
      uvalue!(o).data.as_ptr() as *mut c_void
    } else if ttislightuserdata!(o) {
      // pvalue(o) is defined as a constant in the provided context, but the C++ macro
      // accesses (o)->value.p. Based on the provided PVALUE constant and the
      // requirement to follow the C++ logic, we access the pointer value.
      (*o).value.p
    } else {
      null_mut()
    }
  }
}
