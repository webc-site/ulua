use crate::{
  enums::lua_type::LuaType,
  functions::ensure_stack::ensure_stack,
  macros::{api_incr_top::api_incr_top, setlvalue::setlvalue},
  type_aliases::lua_state::lua_State,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
#[cfg_attr(feature = "capi", unsafe(export_name = "ulua_lua_pushinteger_64"))]
pub(crate) unsafe fn lua_pushinteger_64(l: *mut lua_State, n: i64) {
  unsafe {
    ensure_stack(l, 1);
    let _ = LuaType::Integer;
    setlvalue!((*l).top, n);
    api_incr_top!(l);
  }
}
