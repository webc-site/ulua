use core::ffi::c_int;

use crate::{
  functions::index_2_addr::index2addr,
  macros::{api_check::api_check, hvalue::hvalue, ttistable::ttistable},
  records::{lua_state::lua_State, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_setsafeenv(l: *mut lua_State, objindex: c_int, enabled: c_int) {
  unsafe {
    let o: *const TValue = index2addr(l, objindex);
    api_check!(l, ttistable!(o));
    let t: *mut LuaTable = hvalue!(o);
    (*t).safeenv = if enabled != 0 { 1 } else { 0 };
  }
}
