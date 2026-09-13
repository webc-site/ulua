use core::{
  ffi::{c_int, c_void},
  ptr::null_mut,
};

use crate::{
  functions::index_2_addr::index2addr,
  macros::ttislightuserdata::ttislightuserdata,
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_tolightuserdata(l: *mut lua_State, idx: c_int) -> *mut c_void {
  unsafe {
    let o: StkId = index2addr(l, idx);

    if !ttislightuserdata!(o) {
      null_mut()
    } else {
      // Based on the provided context for pvalue(o) in lobject.h:
      // #define pvalue(o) check_exp(ttislightuserdata(o), (o)->value.p)
      // and the example lua_touserdata which accesses (*o).value.p directly.
      (*o).value.p
    }
  }
}
