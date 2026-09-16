use core::{
  ffi::{c_int, c_void},
  ptr::null_mut,
};

use crate::{
  functions::index_2_addr::index2addr,
  macros::{bufvalue::bufvalue, ttisbuffer::ttisbuffer},
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_tobuffer(l: *mut lua_State, idx: c_int, len: *mut usize) -> *mut c_void {
  unsafe {
    let o: StkId = index2addr(l, idx);

    if !ttisbuffer!(o) {
      return null_mut();
    }

    let b = bufvalue!(o);

    if !len.is_null() {
      *len = b.len as usize;
    }

    b.data.as_ptr() as *mut c_void
  }
}
