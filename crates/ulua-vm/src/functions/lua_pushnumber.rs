use crate::{
  functions::ensure_stack::ensure_stack,
  macros::{api_incr_top::api_incr_top, setnvalue::setnvalue},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_lua_pushnumber"))]
pub unsafe fn lua_pushnumber(l: *mut lua_State, n: f64) {
  unsafe {
    ensure_stack(l, 1);
    setnvalue!((*l).top, n);
    api_incr_top!(l);
  }
}
