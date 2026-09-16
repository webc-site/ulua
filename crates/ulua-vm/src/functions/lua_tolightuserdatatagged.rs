use core::{
  ffi::{c_int, c_void},
  ptr::null_mut,
};

use crate::{
  functions::index_2_addr::index2addr,
  macros::{lightuserdatatag::lightuserdatatag, ttislightuserdata::ttislightuserdata},
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_tolightuserdatatagged(l: *mut lua_State, idx: c_int, tag: c_int) -> *mut c_void {
  unsafe {
    let o: StkId = index2addr(l, idx);

    if !ttislightuserdata!(o) || lightuserdatatag!(o) != tag {
      null_mut()
    } else {
      (*o).value.p
    }
  }
}
