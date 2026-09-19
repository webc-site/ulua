use core::ffi::c_int;

use crate::{
  functions::index_2_addr::index2addr,
  macros::{api_check::api_check, hvalue::hvalue, ttistable::ttistable},
  type_aliases::{lua_state::lua_State, lua_table::LuaTable, t_value::TValue},
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_getreadonly(l: *mut lua_State, objindex: c_int) -> c_int {
  unsafe {
    let o: *const TValue = index2addr(l, objindex);

    api_check!(l, ttistable!(o));

    let t: *mut LuaTable = hvalue!(o);

    (*t).readonly as c_int
  }
}
