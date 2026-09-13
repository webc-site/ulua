use core::ffi::c_int;

use crate::{
  functions::index_2_addr::index2addr,
  macros::{api_check::api_check, hvalue::hvalue, registry::registry, ttistable::ttistable},
  records::{lua_state::lua_State, lua_table::LuaTable},
  type_aliases::t_value::TValue,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn lua_setreadonly(l: *mut lua_State, objindex: c_int, enabled: c_int) {
  unsafe {
    let o: *const TValue = index2addr(l, objindex);
    api_check!(l, ttistable!(o));

    let t: *mut LuaTable = hvalue!(o);

    // The registry macro returns a reference &TValue.
    // hvalue! expects a pointer *mut TValue.
    // We use addr_of! to get a pointer from the reference safely before casting.
    api_check!(
      l,
      t != hvalue!(core::ptr::addr_of!(*registry!(l)) as *mut TValue)
    );

    (*t).readonly = if enabled != 0 { 1 } else { 0 };
  }
}
