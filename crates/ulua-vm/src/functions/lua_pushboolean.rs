use core::ffi::c_int;

use crate::{
  functions::ensure_stack::ensure_stack,
  macros::{api_incr_top::api_incr_top, setbvalue::setbvalue},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_lua_pushboolean"))]
pub unsafe fn lua_pushboolean(l: *mut lua_State, b: c_int) {
  unsafe {
    ensure_stack(l, 1);
    // The setbvalue macro requires TValue and lua_Type to be in scope at the call site.
    setbvalue!((*l).top, b != 0); // ensure that true is 1
    api_incr_top!(l);
  }
}
