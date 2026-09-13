use core::ffi::c_int;

use crate::{
  functions::index_2_addr::index2addr,
  macros::{ttislightuserdata::ttislightuserdata, ttisuserdata::ttisuserdata},
  type_aliases::{lua_state::lua_State, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_isuserdata(l: *mut lua_State, idx: c_int) -> c_int {
  unsafe {
    let o: *const TValue = index2addr(l, idx);
    (ttisuserdata!(o) || ttislightuserdata!(o)) as c_int
  }
}
