use core::{ffi::c_int, ptr::null};

use crate::{
  functions::index_2_addr::index2addr,
  macros::{ttisvector::ttisvector, vvalue::vvalue},
  type_aliases::{lua_state::lua_State, stk_id::StkId},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_tovector(l: *mut lua_State, idx: c_int) -> *const f32 {
  unsafe {
    let o: StkId = index2addr(l, idx);
    if !ttisvector!(o) {
      null()
    } else {
      vvalue!(o).as_ptr()
    }
  }
}
